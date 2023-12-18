const std = @import("std");
const print = std.debug.print;
const expect = std.debug.expect;

fn nextElem(data: []isize) isize {
    if (data.len == 0) {
        return 0;
    }
    if (data.len == 1) {
        return data[0];
    }

    var all_same_diff = false;
    var end = data.len;
    while (!all_same_diff) {
        const first_diff = data[1] - data[0];
        all_same_diff = true;
        for (0..end - 1) |i| {
            data[i] = data[i + 1] - data[i];
            if (data[i] != first_diff) {
                all_same_diff = false;
            }
        }
        end -= 1;
        if (end == 0) {
            unreachable;
        }
    }
    // extrapolate
    var extrap: isize = data[end - 1];
    for (end..data.len) |i| {
        extrap += data[i];
    }
    return extrap;
}

test "extrapolation examples" {
    var ex1 = [_]isize{ 0, 3, 6, 9, 12, 15 };
    try std.testing.expectEqual(nextElem(&ex1), 18);

    var ex2 = [_]isize{ 1, 3, 6, 10, 15, 21 };
    try std.testing.expectEqual(nextElem(&ex2), 28);

    var ex3 = [_]isize{ 10, 13, 16, 21, 30, 45 };
    try std.testing.expectEqual(nextElem(&ex3), 68);
}

pub fn part1(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    var linebuf = std.ArrayList(isize).init(alloc);
    defer linebuf.deinit();

    var total: isize = 0;
    var lines_iter = std.mem.tokenizeScalar(u8, data, '\n');
    while (lines_iter.next()) |line| {
        linebuf.clearRetainingCapacity();
        var line_iter = std.mem.tokenizeScalar(u8, line, ' ');
        while (line_iter.next()) |num_str| {
            const num = try std.fmt.parseInt(isize, num_str, 10);
            try linebuf.append(num);
        }
        total += nextElem(linebuf.items);
    }

    return std.fmt.bufPrint(result_buf, "{}", .{total});
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    var linebuf = std.ArrayList(isize).init(alloc);
    defer linebuf.deinit();

    var total: isize = 0;
    var lines_iter = std.mem.tokenizeScalar(u8, data, '\n');
    while (lines_iter.next()) |line| {
        linebuf.clearRetainingCapacity();
        var line_iter = std.mem.tokenizeScalar(u8, line, ' ');
        while (line_iter.next()) |num_str| {
            const num = try std.fmt.parseInt(isize, num_str, 10);
            try linebuf.insert(0, num);
        }
        total += nextElem(linebuf.items);
    }

    return std.fmt.bufPrint(result_buf, "{}", .{total});
}
