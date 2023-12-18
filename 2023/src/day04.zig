const std = @import("std");
const print = std.debug.print;
const expect = std.testing.expect;

const MAX_WINNING_NUMBERS = 20;
const MAX_GAMES = 256;

const Props = struct {
    numbers_start: usize,
    numbers_you_have_start: usize,
};

fn winsAtLine(line: []const u8, props: *const Props) usize {
    var winning_numbers = [_]u16{0} ** MAX_WINNING_NUMBERS;
    var start = props.numbers_start;
    var i: usize = 0;
    while (start < props.numbers_you_have_start) : (start += 3) {
        winning_numbers[i] = (@as(u16, line[start]) << 8) | line[start + 1];
        i += 1;
    }

    var wins: u32 = 0;
    start = props.numbers_you_have_start;
    while (start < line.len) : (start += 3) {
        const number: u16 = (@as(u16, line[start]) << 8) | line[start + 1];
        if (std.mem.indexOfScalar(u16, &winning_numbers, number) != null) {
            wins += 1;
        }
    }

    return wins;
}

test "winsAtLine" {
    const data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    const props = try calculateProps(data);
    try expect(props.numbers_start == 8);
    try expect(props.numbers_you_have_start == 25);
    try expect(winsAtLine(data, &props) == 4);
}

fn calculateProps(data: []const u8) !Props {
    const numbers_start = 2 + (std.mem.indexOfScalar(u8, data, ':') orelse return error.BadLine);
    const numbers_you_have_start = 2 + (std.mem.indexOfScalar(u8, data, '|') orelse return error.BadLine);

    return Props{
        .numbers_start = numbers_start,
        .numbers_you_have_start = numbers_you_have_start,
    };
}

pub fn part1(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    _ = alloc;
    const props = try calculateProps(data);

    var total: usize = 0;

    var iter = std.mem.tokenizeScalar(u8, data, '\n');
    while (iter.next()) |line| {
        const wins = winsAtLine(line, &props);
        if (wins > 0) {
            total += std.math.pow(usize, 2, wins - 1);
        }
    }

    return std.fmt.bufPrint(result_buf, "{}", .{total});
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    _ = alloc;
    const props = try calculateProps(data);
    const games = std.mem.count(u8, data, "\n");

    var card_counts = [_]usize{1} ** MAX_GAMES;

    var iter = std.mem.tokenizeScalar(u8, data, '\n');
    var i: usize = 0;
    while (iter.next()) |line| {
        const wins = winsAtLine(line, &props);
        for (i + 1..i + wins + 1) |j| {
            card_counts[j] += card_counts[i];
        }
        i += 1;
    }
    var total: usize = 0;
    for (0..games) |x| {
        total += card_counts[x];
    }

    return std.fmt.bufPrint(result_buf, "{}", .{total});
}
