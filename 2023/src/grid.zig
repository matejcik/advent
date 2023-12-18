const std = @import("std");

pub const Point = struct {
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

    pub const UP = Point{ .x = 0, .y = -1 };
    pub const DOWN = Point{ .x = 0, .y = 1 };
    pub const LEFT = Point{ .x = -1, .y = 0 };
    pub const RIGHT = Point{ .x = 1, .y = 0 };

    pub const DIRS_AROUND = [_]Point{
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

pub fn Grid(comptime T: type) type {
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
