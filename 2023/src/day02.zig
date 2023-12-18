const std = @import("std");
const print = std.debug.print;
const expect = std.testing.expect;

const Game = struct {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,

    pub fn init(id: u32) Game {
        return Game{
            .id = id,
            .max_red = 0,
            .max_green = 0,
            .max_blue = 0,
        };
    }

    pub fn updateMax(self: *Game, red: u32, green: u32, blue: u32) void {
        self.max_red = @max(red, self.max_red);
        self.max_green = @max(green, self.max_green);
        self.max_blue = @max(blue, self.max_blue);
    }

    pub fn power(self: *const Game) u32 {
        return self.max_red * self.max_green * self.max_blue;
    }
};

fn processGameLine(line: []const u8) !Game {
    var token = std.mem.tokenizeSequence(u8, line, " ");
    _ = token.next() orelse return error.InvalidLine;
    const id_str = token.next() orelse return error.InvalidLine;
    const id = try std.fmt.parseInt(u32, id_str[0 .. id_str.len - 1], 10);
    var game = Game.init(id);

    var red: u32 = 0;
    var green: u32 = 0;
    var blue: u32 = 0;
    while (token.next()) |nstr| {
        const n = try std.fmt.parseInt(u32, nstr, 10);
        const color = token.next() orelse return error.InvalidLine;
        switch (color[0]) {
            'r' => red = n,
            'g' => green = n,
            'b' => blue = n,
            else => return error.InvalidLine,
        }
        if (color[color.len - 1] == ';') {
            game.updateMax(red, green, blue);
            red = 0;
            green = 0;
            blue = 0;
        }
    }
    game.updateMax(red, green, blue);

    return game;
}

const SAMPLE_INPUT =
    \\Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    \\Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    \\Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    \\Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    \\Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    \\
;

test "sample input part1" {
    const reader = std.io.fixedBufferStream(SAMPLE_INPUT);
    var source = std.io.StreamSource{ .const_buffer = reader };
    try expect(try part1Total(source.reader()) == 8);
}

fn part1Total(data: []const u8) !u32 {
    var total: u32 = 0;
    var iter = std.mem.tokenizeScalar(u8, data, '\n');
    while (iter.next()) |line| {
        const game = try processGameLine(line);
        if (game.max_red <= 12 and game.max_green <= 13 and game.max_blue <= 14) {
            total += game.id;
        }
    }

    return total;
}

pub fn part1(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    _ = alloc;
    const total = try part1Total(data);
    return std.fmt.bufPrint(result_buf, "{}", .{total});
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    _ = alloc;
    var total: u32 = 0;
    var iter = std.mem.tokenizeScalar(u8, data, '\n');
    while (iter.next()) |line| {
        const game = try processGameLine(line);
        total += game.power();
    }

    return std.fmt.bufPrint(result_buf, "{}", .{total});
}
