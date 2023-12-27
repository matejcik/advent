const std = @import("std");
const gridlib = @import("gridlib");

const Point = gridlib.Point;
const Grid = gridlib.Grid(u8);
const Direction = gridlib.Direction;

const DIRECTIONS = blk: {
    const L = [_]Direction{ Direction.UP, Direction.RIGHT };
    const J = [_]Direction{ Direction.UP, Direction.LEFT };
    const vpipe = [_]Direction{ Direction.UP, Direction.DOWN };
    const dash = [_]Direction{ Direction.LEFT, Direction.RIGHT };
    const seven = [_]Direction{ Direction.LEFT, Direction.DOWN };
    const F = [_]Direction{ Direction.RIGHT, Direction.DOWN };
    var pointmap: [128][2]Direction = [_][2]Direction{[_]Direction{ Direction.ZERO, Direction.ZERO }} ** 128;
    pointmap['L'] = L;
    pointmap['J'] = J;
    pointmap['|'] = vpipe;
    pointmap['-'] = dash;
    pointmap['7'] = seven;
    pointmap['F'] = F;
    break :blk pointmap;
};

fn connectsBack(grid: *const Grid, point: Point, direction: Direction) bool {
    const next = point.step(direction);
    const back = direction.back();
    const dirs = DIRECTIONS[grid.point(next) orelse return false];
    return std.meta.eql(dirs[0], back) or std.meta.eql(dirs[1], back);
}

fn walk(grid: *const Grid, start: Point, direction: Direction) usize {
    var point = start.step(direction);
    var prevdir = direction.back();
    var count: usize = 1;
    while (!std.meta.eql(point, start)) {
        const c = grid.point(point) orelse return 0;
        const dirs = DIRECTIONS[c];
        const nextdir = if (std.meta.eql(prevdir, dirs[0]))
            dirs[1]
        else if (std.meta.eql(prevdir, dirs[1]))
            dirs[0]
        else
            return 0;
        point = point.step(nextdir);
        prevdir = nextdir.back();
        count += 1;
    }
    return count;
}

pub fn part1(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    _ = alloc;

    const grid = Grid.new(.{ .immutable = data }, '\n');
    const start = grid.indexToPoint(std.mem.indexOfScalar(u8, data, 'S').?);
    var connects: ?Direction = null;
    var connections: u8 = 0;
    for (Direction.ORTHO) |dir| {
        if (connectsBack(&grid, start, dir)) {
            connections += 1;
            connects = dir;
        }
    }

    if (connections != 2) {
        return error.UnexpectedInput;
    }

    const steps = walk(&grid, start, connects.?);
    return std.fmt.bufPrint(result_buf, "{}", .{steps / 2});
}

fn walkMark(grid: *const Grid, mark: *Grid, start: Point, direction: Direction) usize {
    mark.ptrPoint(start).?.* = grid.point(start).?;
    var point = start.step(direction);
    var prevdir = direction.back();
    var count: usize = 1;
    while (!std.meta.eql(point, start)) {
        mark.ptrPoint(point).?.* = grid.point(point).?;
        const c = grid.point(point) orelse return 0;
        const dirs = DIRECTIONS[c];
        const nextdir = if (std.meta.eql(prevdir, dirs[0]))
            dirs[1]
        else if (std.meta.eql(prevdir, dirs[1]))
            dirs[0]
        else
            return 0;
        point = point.step(nextdir);
        prevdir = nextdir.back();
        count += 1;
    }
    return count;
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    const grid = Grid.new(.{ .immutable = data }, '\n');
    const start = grid.indexToPoint(std.mem.indexOfScalar(u8, data, 'S').?);
    var connects: ?Direction = null;
    var connbits: usize = 0;
    for (Direction.ORTHO, 0..) |dir, i| {
        if (connectsBack(&grid, start, dir)) {
            connbits |= @as(usize, 1) << @intCast(i);
            connects = dir;
        }
    }

    const dirbits = [_]u8{
        '.', // 0000
        '.', // 0001
        '.', // 0010
        'L', // 0011
        '.', // 0100
        '|', // 0101
        'F', // 0110
        '.', // 0111
        '.', // 1000
        'J', // 1001
        '-', // 1010
        '.', // 1011
        '7', // 1100
        '.', // 1101
        '.', // 1110
        '.', // 1111
    };

    if (dirbits[connbits] == '.') {
        return error.UnexpectedInput;
    }

    const mark_buf = try alloc.alloc(u8, grid.width * grid.height);
    defer alloc.free(mark_buf);
    @memset(mark_buf, ' ');
    var mark = Grid.newRegular(.{ .mutable = mark_buf }, grid.width, grid.height);

    _ = walkMark(&grid, &mark, start, connects.?);
    mark.ptrPoint(start).?.* = dirbits[connbits];

    var tiles: usize = 0;
    var counting: usize = 0;
    var starter: u8 = 0;
    for (mark_buf) |c| {
        switch (c) {
            '|' => counting = 1 - counting,
            'L', 'F' => {
                starter = c;
            },
            'J' => {
                if (starter == 'F') {
                    counting = 1 - counting;
                }
                starter = 0;
            },
            '7' => {
                if (starter == 'L') {
                    counting = 1 - counting;
                }
                starter = 0;
            },
            '-' => {},
            else => {
                tiles += counting;
            },
        }
    }
    return std.fmt.bufPrint(result_buf, "{}", .{tiles});
}
