const std = @import("std");

fn stripZigExt(path: []const u8) []const u8 {
    if (!std.mem.endsWith(u8, path, ".zig")) {
        return path;
    }
    return path[0 .. path.len - 4];
}

pub fn openInput(srcInfo: std.builtin.SourceLocation) !std.io.AnyReader {
    const src_root = std.fs.path.dirname(std.fs.path.dirname(srcInfo.file));
    const basename = stripZigExt(std.fs.path.basename(srcInfo.file));

    const input_path = src_root ++ "/inputs/" ++ basename ++ ".txt";
    return try input_path.open(.{ .read = true }).reader();
}

pub fn sliceEql(comptime T: type, a: []const T, b: []const T) bool {
    if (a.len != b.len) return false;
    for (a, b) |a_elem, b_elem| {
        if (!std.meta.eql(a_elem, b_elem)) return false;
    }
    return true;
}
