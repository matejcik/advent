const std = @import("std");
const print = std.debug.print;

const Point = struct {
    x: isize,
    y: isize,

    pub fn add(self: Point, other: Point) Point {
        return Point{
            .x = self.x + other.x,
            .y = self.y + other.y,
        };
    }

    pub fn sub(self: Point, other: Point) Point {
        return Point{
            .x = self.x - other.x,
            .y = self.y - other.y,
        };
    }

    const UP = Point{ .x = 0, .y = -1 };
    const DOWN = Point{ .x = 0, .y = 1 };
    const LEFT = Point{ .x = -1, .y = 0 };
    const RIGHT = Point{ .x = 1, .y = 0 };

    const DIRS_AROUND = [_]Point{
        Point.UP.add(Point.LEFT),
        Point.UP,
        Point.UP.add(Point.RIGHT),
        Point.LEFT,
        Point.RIGHT,
        Point.DOWN.add(Point.LEFT),
        Point.DOWN,
        Point.DOWN.add(Point.RIGHT),
    };
};

fn Grid(comptime T: type) type {
    const GridMutability = enum {
        mutable,
        immutable,
    };
    const GridData = union(GridMutability) {
        mutable: []T,
        immutable: []const T,

        pub fn view(self: @This()) []const T {
            switch (self) {
                GridMutability.mutable => return self.mutable,
                GridMutability.immutable => return self.immutable,
            }
        }
    };

    return struct {
        data: GridData,
        stride: usize, // full width including line separators
        width: usize, // width of the grid
        height: usize,

        pub fn new(data: GridData, linesep: T) Grid(T) {
            var grid = Grid(T){
                .data = data,
                .stride = 0,
                .width = 0,
                .height = 0,
            };
            const view_ = data.view();
            const sep_index = std.mem.indexOfScalar(T, view_, linesep);
            if (sep_index) |stride| {
                grid.stride = stride + 1;
                grid.width = stride;
            } else {
                grid.stride = view_.len;
                grid.width = view_.len;
            }
            grid.height = view_.len / grid.stride;
            return grid;
        }

        pub fn at(self: Grid(T), x: usize, y: usize) ?T {
            if (x >= self.width or y >= self.height) {
                return null;
            }
            return self.data.view()[y * (self.width + 1) + x];
        }

        pub fn indexToPoint(self: Grid(T), index: usize) Point {
            return Point{
                .x = @intCast(index % self.stride),
                .y = @intCast(index / self.stride),
            };
        }

        pub fn contains(self: Grid(T), p: Point) bool {
            return p.x >= 0 and p.y >= 0 and p.x < self.width and p.y < self.height;
        }

        pub fn pointToIndex(self: Grid(T), p: Point) ?usize {
            if (!self.contains(p)) {
                return null;
            }
            return @intCast(p.y * @as(isize, @intCast(self.stride)) + p.x);
        }

        pub fn point(self: Grid(T), p: Point) ?T {
            if (p.x < 0 or p.y < 0) {
                return null;
            }
            return self.at(@intCast(p.x), @intCast(p.y));
        }

        pub fn ptrAt(self: *Grid(T), x: usize, y: usize) ?*T {
            if (std.meta.Tag(self.data) != GridMutability.mutable) {
                return null;
            }
            if (x >= self.width or y >= self.height) {
                return null;
            }
            return &self.data.mutable[y * (self.width + 1) + x];
        }

        pub fn view(self: Grid(T)) []const T {
            return self.data.view();
        }
    };
}

fn isPart(char: u8) bool {
    switch (char) {
        '.', '\n', '0'...'9' => return false,
        else => return true,
    }
}

fn findDigits(grid: Grid(u8), idx: usize, direction: Point) ?[2]usize {
    const p = grid.indexToPoint(idx);
    var cur = p.add(direction);
    while (grid.contains(cur) and std.ascii.isDigit(grid.point(cur).?)) {
        cur = cur.add(direction);
    }
    const prev = cur.sub(direction);
    const prev_idx = grid.pointToIndex(prev).?;
    if (prev_idx == idx) {
        return null;
    }
    var left_idx: usize = 0;
    var right_idx: usize = 0;
    if (prev_idx < idx) {
        left_idx = prev_idx;
        right_idx = idx;
    } else {
        left_idx = idx + 1;
        right_idx = prev_idx + 1;
    }
    return .{ left_idx, right_idx };
}

fn findNum(grid: Grid(u8), idx: usize, direction: Point) ?usize {
    if (findDigits(grid, idx, direction)) |digits| {
        const numstr = grid.view()[digits[0]..digits[1]];
        return std.fmt.parseInt(usize, numstr, 10) catch unreachable;
    } else {
        return null;
    }
}

const NumResult = struct {
    count: usize,
    sum: usize,
    ratio: usize,

    pub fn init() NumResult {
        return .{
            .count = 0,
            .sum = 0,
            .ratio = 1,
        };
    }

    pub fn add(self: NumResult, other: NumResult) NumResult {
        return .{
            .count = self.count + other.count,
            .sum = self.sum + other.sum,
            .ratio = self.ratio * other.ratio,
        };
    }

    pub fn addNum(self: *NumResult, num: usize) void {
        self.count += 1;
        self.sum += num;
        self.ratio *= num;
    }
};

fn findNumsLookBothSides(grid: Grid(u8), point: Point) NumResult {
    var result = NumResult.init();
    if (!grid.contains(point)) {
        return result;
    }
    const center_idx = grid.pointToIndex(point).?;
    const left_of = findDigits(grid, center_idx, Point.LEFT);
    const right_of = findDigits(grid, center_idx, Point.RIGHT);
    if (std.ascii.isDigit(grid.view()[center_idx])) {
        const left_idx = (left_of orelse .{ center_idx, center_idx })[0];
        const right_idx = (right_of orelse .{ center_idx, center_idx + 1 })[1];
        const num_str = grid.view()[left_idx..right_idx];
        result.addNum(std.fmt.parseInt(usize, num_str, 10) catch unreachable);
    } else {
        if (left_of) |ln| {
            const lstr = grid.view()[ln[0]..ln[1]];
            result.addNum(std.fmt.parseInt(usize, lstr, 10) catch unreachable);
        }
        if (right_of) |rn| {
            const rstr = grid.view()[rn[0]..rn[1]];
            result.addNum(std.fmt.parseInt(usize, rstr, 10) catch unreachable);
        }
    }
    return result;
}

fn partNumbersAround(grid: Grid(u8), idx: usize) NumResult {
    const p = grid.indexToPoint(idx);
    var result = NumResult.init();
    // left side
    if (findNum(grid, idx, Point.LEFT)) |left_num| {
        result.addNum(left_num);
    }
    // right side
    if (findNum(grid, idx, Point.RIGHT)) |right_num| {
        result.addNum(right_num);
    }
    // top side
    const top = p.add(Point.UP);
    result = result.add(findNumsLookBothSides(grid, top));
    // bottom side
    const bottom = p.add(Point.DOWN);
    result = result.add(findNumsLookBothSides(grid, bottom));

    return result;
}

pub fn part1(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    _ = alloc;

    const grid = Grid(u8).new(.{ .immutable = data }, '\n');
    var total: usize = 0;
    for (grid.view(), 0..) |c, i| {
        if (isPart(c)) {
            const nums = partNumbersAround(grid, i);
            total += nums.sum;
        }
    }

    return std.fmt.bufPrint(result_buf, "{}", .{total});
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    _ = alloc;

    const grid = Grid(u8).new(.{ .immutable = data }, '\n');
    var total: usize = 0;
    for (grid.view(), 0..) |c, i| {
        if (c == '*') {
            const nums = partNumbersAround(grid, i);
            if (nums.count == 2) {
                total += nums.ratio;
            }
        }
    }

    return std.fmt.bufPrint(result_buf, "{}", .{total});
}
