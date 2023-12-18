const std = @import("std");
const print = std.debug.print;
const expect = std.testing.expect;
const expectEqual = std.testing.expectEqual;

pub fn solveQuadratic(ia: isize, ib: isize, ic: isize) ?struct { isize, isize } {
    const a = @as(f64, @floatFromInt(ia));
    const b = @as(f64, @floatFromInt(ib));
    const c = @as(f64, @floatFromInt(ic));
    const discriminant = b * b - 4.0 * a * c;
    if (discriminant < 0.0) {
        return null;
    }
    const sqrtDiscriminant = std.math.sqrt(discriminant);
    const t0 = (-b - sqrtDiscriminant) / (2.0 * a);
    const t1 = (-b + sqrtDiscriminant) / (2.0 * a);
    return .{ @as(isize, @intFromFloat(std.math.ceil(t0))), @as(isize, @intFromFloat(std.math.floor(t1))) };
}

pub fn solveTimeDistance(time: isize, distance_to_beat: isize) ?struct { isize, isize } {
    // distance_to_beat < x * (time - x)
    // distance_to_beat < x * time - x^2
    // x^2 - time * x + distance_to_beat < 0
    return solveQuadratic(1, -time, distance_to_beat);
}

test "example" {
    const result = solveTimeDistance(7, 9).?;
    try expectEqual(result[0], 2);
    try expectEqual(result[1], 5);
}

pub fn part1(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    _ = alloc;

    var lines = std.mem.splitScalar(u8, data, '\n');
    const times = lines.next().?;
    const distances = lines.next().?;

    var total: isize = 1;

    var times_iter = std.mem.tokenizeScalar(u8, times, ' ');
    var distances_iter = std.mem.tokenizeScalar(u8, distances, ' ');
    _ = times_iter.next();
    _ = distances_iter.next();
    while (true) {
        const time_ = times_iter.next();
        if (time_ == null) {
            break;
        }
        const time = try std.fmt.parseInt(isize, time_.?, 10);
        const distance = try std.fmt.parseInt(isize, distances_iter.next().?, 10);
        const result = solveTimeDistance(time, distance).?;
        total *= result[1] - result[0] + 1;
    }

    return std.fmt.bufPrint(result_buf, "{}", .{total});
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    _ = alloc;

    var lines = std.mem.splitScalar(u8, data, '\n');
    const times = lines.next().?;
    const distances = lines.next().?;

    var times_iter = std.mem.tokenizeScalar(u8, times, ' ');
    var distances_iter = std.mem.tokenizeScalar(u8, distances, ' ');
    _ = times_iter.next();
    _ = distances_iter.next();

    var time: usize = 0;
    var distance: usize = 0;

    while (true) {
        const time_ = times_iter.next();
        if (time_ == null) {
            break;
        }
        const distance_ = distances_iter.next().?;

        const timepart = try std.fmt.parseInt(usize, time_.?, 10);
        time *= std.math.pow(usize, 10, time_.?.len);
        time += timepart;

        const distancepart = try std.fmt.parseInt(usize, distance_, 10);
        distance *= std.math.pow(usize, 10, distance_.len);
        distance += distancepart;
    }

    const result = solveTimeDistance(@as(isize, @intCast(time)), @as(isize, @intCast(distance))).?;
    const total = result[1] - result[0] + 1;
    return std.fmt.bufPrint(result_buf, "{}", .{total});
}
