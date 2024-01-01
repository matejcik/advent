const std = @import("std");

pub const Point = struct {
    x: isize,
    y: isize,

    pub fn new(x: isize, y: isize) Point {
        return Point{
            .x = x,
            .y = y,
        };
    }

    pub fn gridDistance(self: Point, other: Point) usize {
        return @as(usize, @abs(self.x - other.x) + @abs(self.y - other.y));
    }

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

    pub fn step(self: Point, dir: Direction) Point {
        return self.add(dir.dir);
    }

    pub fn back(self: Point, dir: Direction) Point {
        return self.step(dir.back());
    }
};

pub const Direction = struct {
    dir: Point,

    pub fn new(x: isize, y: isize) Direction {
        return Direction{
            .dir = Point.new(x, y),
        };
    }

    pub fn add(self: Direction, other: Direction) Direction {
        return Direction{
            .dir = self.dir.add(other.dir),
        };
    }

    pub fn sub(self: Direction, other: Direction) Direction {
        return Direction{
            .dir = self.dir.sub(other.dir),
        };
    }

    pub fn back(self: Direction) Direction {
        return Direction.ZERO.sub(self);
    }

    pub const ZERO = Direction.new(0, 0);
    pub const UP = Direction.new(0, -1);
    pub const DOWN = Direction.new(0, 1);
    pub const LEFT = Direction.new(-1, 0);
    pub const RIGHT = Direction.new(1, 0);

    pub const ORTHO = [_]Direction{
        Direction.UP,
        Direction.RIGHT,
        Direction.DOWN,
        Direction.LEFT,
    };

    pub const AROUND = [_]Direction{
        Direction.UP.add(Direction.LEFT),
        Direction.UP,
        Direction.UP.add(Direction.RIGHT),
        Direction.RIGHT,
        Direction.DOWN.add(Direction.RIGHT),
        Direction.DOWN,
        Direction.DOWN.add(Direction.LEFT),
        Direction.LEFT,
    };
};

pub fn Grid(comptime T: type) type {
    const GridMutability = enum {
        mutable,
        immutable,
        owned,
    };
    const GridData = union(GridMutability) {
        mutable: []T,
        immutable: []const T,
        owned: struct {
            data: []T,
            allocator: std.mem.Allocator,
        },

        pub fn view(self: @This()) []const T {
            switch (self) {
                GridMutability.mutable => return self.mutable,
                GridMutability.immutable => return self.immutable,
                GridMutability.owned => return self.owned.data,
            }
        }

        pub fn viewMut(self: @This()) ![]T {
            switch (self) {
                GridMutability.mutable => return self.mutable,
                GridMutability.immutable => return error.Immutable,
                GridMutability.owned => return self.owned.data,
            }
        }

        pub fn alloc(allocator: std.mem.Allocator, size: usize) !@This() {
            return @This(){ .owned = .{
                .data = try allocator.alloc(T, size),
                .allocator = allocator,
            } };
        }

        pub fn deinit(self: @This()) void {
            switch (self) {
                GridMutability.mutable => {},
                GridMutability.immutable => {},
                GridMutability.owned => {
                    self.owned.allocator.free(self.owned.data);
                },
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

        pub fn deinit(self: Grid(T)) void {
            self.data.deinit();
        }

        pub fn newRegular(data: GridData, width: usize, height: usize) Grid(T) {
            return Grid(T){
                .data = data,
                .stride = width,
                .width = width,
                .height = height,
            };
        }

        pub fn newAlloc(allocator: std.mem.Allocator, width: usize, height: usize) !Grid(T) {
            return Grid(T){
                .data = try GridData.alloc(allocator, width * height),
                .stride = width,
                .width = width,
                .height = height,
            };
        }

        pub fn at(self: Grid(T), x: usize, y: usize) ?T {
            if (x >= self.width or y >= self.height) {
                return null;
            }
            return self.data.view()[y * self.stride + x];
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
            const buffer = self.viewMut() catch return null;
            if (x >= self.width or y >= self.height) {
                return null;
            }
            return &buffer[y * self.stride + x];
        }

        pub fn ptrPoint(self: *Grid(T), p: Point) ?*T {
            return self.ptrAt(@intCast(p.x), @intCast(p.y));
        }

        pub fn view(self: Grid(T)) []const T {
            return self.data.view();
        }

        pub fn viewMut(self: Grid(T)) ![]T {
            return self.data.viewMut();
        }

        pub fn line(self: Grid(T), y: usize) ?[]const T {
            const idx = self.pointToIndex(Point.new(0, @intCast(y))) orelse return null;
            return self.data.view()[idx .. idx + self.width];
        }

        pub fn lineMut(self: Grid(T), y: usize) ?[]T {
            var buffer = self.data.viewMut() catch return null;
            const idx = self.pointToIndex(Point.new(0, @intCast(y))) orelse return null;
            return buffer[idx .. idx + self.width];
        }

        pub fn print(self: Grid(T)) !void {
            var out = std.io.getStdOut().writer();
            for (0..self.height) |y| {
                for (0..self.width) |x| {
                    try out.print("{c}", .{self.at(x, y).?});
                }
                try out.print("\n", .{});
            }
        }

        pub fn printReturn(self: Grid(T)) !void {
            try std.io.getStdOut().writer().print("\x1b[1;1H", .{});
            try self.print();
        }
    };
}
