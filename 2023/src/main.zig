const std = @import("std");
const clap = @import("clap");

const MAX_SIZE: usize = 1 * 1024 * 1024;

const SolverFunc = fn ([]const u8, std.mem.Allocator, []u8) anyerror![]const u8;

//fn DayImpl(comptime day: comptime_int, comptime solver_impl: type) type {
const DayImpl = struct {
    input_filename: []const u8,
    day: u8,
    parts: [2]*const SolverFunc,

    pub fn init(comptime day: comptime_int, comptime solver_impl: type) @This() {
        const file_stem = std.fmt.comptimePrint("{:0>2}", .{day});
        return DayImpl{
            .parts = [_]*const SolverFunc{ solver_impl.part1, solver_impl.part2 },
            .input_filename = "input/" ++ file_stem ++ ".txt",
            .day = @as(u8, day),
        };
    }

    fn loadData(self: DayImpl, alloc: std.mem.Allocator) ![]const u8 {
        const input_file = std.fs.cwd().openFile(self.input_filename, .{ .mode = .read_only }) catch unreachable;
        var source = std.io.StreamSource{ .file = input_file };
        return source.reader().readAllAlloc(alloc, MAX_SIZE) catch unreachable;
    }

    pub fn runPart(self: DayImpl, part: u8, alloc: std.mem.Allocator) !void {
        const data = try self.loadData(alloc);
        defer alloc.free(data);

        var result_buf = [_]u8{0} ** 256;
        const result = try self.parts[@as(usize, part)](data, alloc, &result_buf);
        std.debug.print("Day {} Part {}: {s}\n", .{ self.day, part + 1, result });
    }

    pub fn timeIt(self: DayImpl, alloc: std.mem.Allocator) !f64 {
        const data = try self.loadData(alloc);
        defer alloc.free(data);

        const a = try timeit(self.day, 1, self.parts[0], data, alloc);
        const b = try timeit(self.day, 2, self.parts[1], data, alloc);
        return a + b;
    }
};

const PARTS = [_]DayImpl{
    DayImpl.init(1, @import("day01")),
    DayImpl.init(2, @import("day02")),
    DayImpl.init(3, @import("day03")),
    DayImpl.init(4, @import("day04")),
    DayImpl.init(5, @import("day05")),
    DayImpl.init(6, @import("day06")),
    DayImpl.init(7, @import("day07")),
    DayImpl.init(8, @import("day08")),
    DayImpl.init(9, @import("day09")),
    DayImpl.init(10, @import("day10")),
    DayImpl.init(11, @import("day11")),
    DayImpl.init(12, @import("day12")),
};

fn arith_mean(items: []const f64) f64 {
    var sum: f64 = 0;
    for (items) |item| {
        sum += item;
    }
    return sum / @as(f64, @floatFromInt(items.len));
}

fn write_time(writer: anytype, time: f64) !void {
    if (time < 1_000) {
        try writer.print("{d: >3.3} ns", .{time});
    } else if (time < 1_000_000) {
        try writer.print("{d: >3.3} μs", .{time / 1_000});
    } else if (time < 1_000_000_000) {
        try writer.print("{d: >3.3} ms", .{time / 1_000_000});
    } else {
        try writer.print("{d: >3.3} s", .{time / 1_000_000_000});
    }
}

fn timeit(day: u32, part: u8, F: *const SolverFunc, data: []const u8, alloc: std.mem.Allocator) !f64 {
    var result_buf = [_]u8{0} ** 256;
    var timer = try std.time.Timer.start();
    const result = try F(data, alloc, &result_buf);
    const measure = timer.lap();

    const tries = @max(5, 1_000_000_000 / measure);

    var tries_times = try std.ArrayList(f64).initCapacity(alloc, tries);
    defer tries_times.deinit();

    for (0..tries) |_| {
        _ = timer.lap();
        _ = try F(data, alloc, &result_buf);
        const time = timer.lap();
        tries_times.appendAssumeCapacity(@floatFromInt(time));
    }
    const min = std.mem.min(f64, tries_times.items);
    const max = std.mem.max(f64, tries_times.items);
    const mean = arith_mean(tries_times.items);

    var writer = std.io.getStdOut().writer();
    try std.fmt.format(writer, "Day {} Part {}: ", .{ day, part });
    _ = try writer.write("avg: ");
    try write_time(writer, mean);
    _ = try writer.write(", min: ");
    try write_time(writer, min);
    _ = try writer.write(", max: ");
    try write_time(writer, max);
    try std.fmt.format(writer, " ({} tries)\t\tAnswer: {s}\n", .{ tries, result });
    return mean;
}

const DayPart = struct {
    day: u8,
    part: u8,

    pub fn parser(in: []const u8) !DayPart {
        var tok = std.mem.tokenizeScalar(u8, in, ':');
        const daystr = tok.next() orelse return error.EmptyArgument;
        const partstr = tok.next() orelse return error.MissingPartSep;
        if (tok.next() != null)
            return error.TooManyParts;

        const day = try std.fmt.parseInt(u8, daystr, 10);
        if (day < 1 or day > PARTS.len)
            return error.InvalidDay;
        const part = try std.fmt.parseInt(u8, partstr, 10);
        if (part < 1 or part > 2)
            return error.InvalidPart;

        return DayPart{ .day = day - 1, .part = part - 1 };
    }
};

pub fn main() !void {
    // First we specify what parameters our program can take.
    // We can use `parseParamsComptime` to parse a string into an array of `Param(Help)`
    const params = comptime clap.parseParamsComptime(
        \\-h, --help             Display this help and exit.
        \\<TASK>...
        \\
        \\Specify TASK as day:part to run a specific part of a specific day. When no
        \\TASK is specified, all parts of all days are run and performance-timed.
    );

    var alloc = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = alloc.deinit();

    // Initialize our diagnostics, which can be used for reporting useful errors.
    // This is optional. You can also pass `.{}` to `clap.parse` if you don't
    // care about the extra information `Diagnostics` provides.
    const parsers = .{
        .TASK = DayPart.parser,
    };
    var diag = clap.Diagnostic{};
    var res = clap.parse(clap.Help, &params, parsers, .{
        .diagnostic = &diag,
        .allocator = alloc.allocator(),
    }) catch |err| {
        // Report useful error and exit
        diag.report(std.io.getStdErr().writer(), err) catch {};
        return err;
    };
    if (res.args.help != 0)
        return clap.help(std.io.getStdErr().writer(), clap.Help, &params, .{});

    defer res.deinit();

    var total: f64 = 0;
    if (res.positionals.len == 0) {
        for (PARTS) |part| {
            total += try part.timeIt(alloc.allocator());
        }
        var writer = std.io.getStdOut().writer();
        _ = try writer.write("Total time: ");
        try write_time(writer, total);
        _ = try writer.write("\n");
        return;
    }

    for (res.positionals) |task| {
        try PARTS[task.day].runPart(task.part, alloc.allocator());
    }
}
