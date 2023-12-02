const std = @import("std");

const part1 = @import("01.zig");

pub fn main() !void {
    const file = try std.fs.cwd().openFile("inputs/01.txt", .{ .mode = .read_only });
    var source = std.io.StreamSource{ .file = file };
    try part1.part1(source.reader());
    try source.seekTo(0);
    try part1.part2(source.reader());
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
