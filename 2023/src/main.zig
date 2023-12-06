const std = @import("std");

const MAX_SIZE: usize = 1 * 1024 * 1024;

const PARTS = [_]type{
    @import("day01"),
    @import("day02"),
    @import("day03"),
    @import("day04"),
    @import("day05"),
};

fn runSingle(comptime n: u32, comptime solver_impl: type) !void {
    const file_stem = std.fmt.comptimePrint("{:0>2}", .{n});
    const input_file = try std.fs.cwd().openFile("inputs/" ++ file_stem ++ ".txt", .{ .mode = .read_only });
    var source = std.io.StreamSource{ .file = input_file };
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = alloc.deinit();
    const data: []const u8 = try source.reader().readAllAlloc(alloc.allocator(), MAX_SIZE);
    defer alloc.allocator().free(data);

    try solver_impl.part1(data, alloc.allocator());
    try source.seekTo(0);
    try solver_impl.part2(data, alloc.allocator());
}

pub fn main() !void {
    inline for (PARTS, 0..) |part, n| {
        try runSingle(n + 1, part);
    }
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.std.mem.Allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
