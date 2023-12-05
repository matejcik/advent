const std = @import("std");
const print = std.debug.print;
const expect = std.testing.expect;

const Range = struct {
    start: isize,
    end: isize,

    pub fn new(start: isize, len_: isize) Range {
        return Range{ .start = start, .end = start + len_ - 1 };
    }

    pub fn empty() Range {
        return Range{ .start = 0, .end = -1 };
    }

    pub fn len(self: Range) isize {
        return self.end - self.start + 1;
    }

    pub fn intersection(self: Range, other: Range) ?Range {
        if (self.start > other.end or self.end < other.start) {
            return null;
        }
        return Range{
            .start = @max(self.start, other.start),
            .end = @min(self.end, other.end),
        };
    }

    pub fn contains(self: Range, other: Range) bool {
        return self.start <= other.start and self.end >= other.end;
    }

    pub fn containsScalar(self: Range, scalar: isize) bool {
        return self.start <= scalar and self.end >= scalar;
    }
};

pub fn part1(data: []const u8, alloc: std.mem.Allocator) !void {
    _ = data;
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator) !void {
    _ = alloc;
    _ = data;
}
