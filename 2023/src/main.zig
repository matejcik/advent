const std = @import("std");

const part1 = @import("01.zig");

const PARTS = [_]type{
    @import("01.zig"),
    @import("02.zig"),
};

fn runSingle(comptime n: u32, comptime solver_impl: type) !void {
    const file_stem = std.fmt.comptimePrint("{:02}", .{n});
    const input_file = try std.fs.cwd().openFile("inputs/" ++ file_stem ++ ".txt", .{ .mode = .read_only });
    var source = std.io.StreamSource{ .file = input_file };

    try solver_impl.part1(source.reader());
    try source.seekTo(0);
    try solver_impl.part2(source.reader());
}

pub fn main() !void {
    inline for (PARTS, 0..) |part, n| {
        try runSingle(n + 1, part);
    }
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
