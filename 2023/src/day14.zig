const std = @import("std");
const gridlib = @import("gridlib");
const Grid = gridlib.Grid(u8);
const Point = gridlib.Point;

const MAX_WIDTH = 128;

const SIMD_WIDTH_BITS = 256;
const SIMD_WORDS = SIMD_WIDTH_BITS / (8 * @sizeOf(u8));

const BoolVec = @Vector(SIMD_WORDS, bool);
const U8Vec = @Vector(SIMD_WORDS, u8);
const U16Vec = @Vector(SIMD_WORDS, u16);

const ONES: U8Vec = @splat(1);
const ZEROES: U8Vec = @splat(0);

fn sum(comptime T: type, self: []const T) T {
    var res: T = 0;
    for (self) |n| {
        res += n;
    }
    return res;
}

pub fn part1novec(data: []const u8) u32 {
    const grid = Grid.new(.{ .immutable = data }, '\n');
    var breaks = [_]u8{@intCast(grid.height)} ** MAX_WIDTH;
    var stones = [_]u8{0} ** MAX_WIDTH;
    var totals = [_]u32{0} ** MAX_WIDTH;

    for (0..grid.height) |y| {
        const line = grid.line(y).?;
        for (line, 0..) |ch, x| {
            switch (ch) {
                '#' => {
                    breaks[x] = @intCast(grid.height - y - 1);
                    stones[x] = 0;
                },
                'O' => {
                    totals[x] += breaks[x] - stones[x];
                    stones[x] += 1;
                },
                else => {},
            }
        }
    }
    // std.debug.print("\nnovec column totals: ", .{});
    // for (0..grid.width) |x| {
    //     std.debug.print("{} ", .{totals[x]});
    // }
    // std.debug.print("\n", .{});

    return sum(u32, &totals);
}

pub fn northLoad(grid: *const Grid) u32 {
    std.debug.assert(grid.stride * grid.height + SIMD_WORDS <= grid.view().len);

    const Item = struct {
        breaks: U8Vec,
        stones: U8Vec,
        totals: U16Vec,
    };

    const elems = (grid.width + SIMD_WORDS - 1) / SIMD_WORDS;

    var array = [_]Item{.{
        .breaks = @splat(@intCast(grid.height)),
        .stones = @splat(0),
        .totals = @splat(0),
    }} ** MAX_WIDTH;

    for (0..grid.height) |y| {
        // grab index of the first element of the line
        const idx = grid.pointToIndex(.{ .x = 0, .y = @intCast(y) }).?;
        // grab the line buffer. this should be the line plus some data from the next line at the end
        const line = grid.view()[idx..][0 .. elems * SIMD_WORDS];
        // vec of next values
        const simd_next_value: U8Vec = @splat(@intCast(grid.height - y - 1));

        for (array[0..elems], 0..) |*item, elem| {
            const ofs = elem * SIMD_WORDS;
            const segment = line[ofs..][0..SIMD_WORDS];

            const simd_segment: U8Vec = segment.*;
            const simd_blocks = simd_segment == @as(U8Vec, @splat('#'));
            const simd_stones = simd_segment == @as(U8Vec, @splat('O'));
            // if char == 'O' {
            //     totals[x] += breaks[x] - stones[x];
            item.totals += @select(u8, simd_stones, item.breaks - item.stones, ZEROES);
            //     stones[x] += 1;
            item.stones += @select(u8, simd_stones, ONES, ZEROES);
            // } else if char == '#' {
            //     stones[x] = 0;
            item.stones = @select(u8, simd_blocks, ZEROES, item.stones);
            //     breaks[x] = @intCast(value - 1);
            item.breaks = @select(u8, simd_blocks, simd_next_value, item.breaks);
        }
    }
    // collect totals
    var total: u32 = 0;
    for (0..grid.width / SIMD_WORDS) |i| {
        total += @reduce(.Add, array[i].totals);
    }
    if (grid.width % SIMD_WORDS != 0) {
        const lastvec = array[grid.width / SIMD_WORDS].totals;
        for (0..grid.width % SIMD_WORDS) |i| {
            total += lastvec[i];
        }
    }

    return total;
}

test "part1 example novec" {
    const total = part1novec(PART1_EXAMPLE);
    try std.testing.expectEqual(total, 136);
}

test "part1 example vec" {
    const grid = expandForSimd(PART1_EXAMPLE, std.testing.allocator);
    defer std.testing.allocator.free(grid.data.mutable);
    const total = northLoad(&grid);
    try std.testing.expectEqual(total, 136);
}

fn expandForSimd(data: []const u8, alloc: std.mem.Allocator) Grid {
    const gridpat = Grid.new(.{ .immutable = data }, '\n');

    // preallocate a longer buffer, so that when we try to read a line whose width is
    // a multiple of SIMD_WORDS, all the data is still in the buffer
    // (the trailing part is going to be noise from the next line, but we don't care)
    var data_extended = alloc.alloc(u8, data.len + SIMD_WORDS) catch unreachable;
    @memcpy(data_extended[0..data.len], data);

    var grid = Grid.new(.{ .mutable = data_extended }, '\n');
    grid.height = gridpat.height;

    return grid;
}

pub fn part1(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    const grid = expandForSimd(data, alloc);
    defer alloc.free(grid.data.mutable);

    const total = northLoad(&grid);
    return std.fmt.bufPrint(result_buf, "{}", .{total});
}

fn spinStep(view: []u8, start: usize, row_stride: isize, row_count: usize, step: isize, step_count: usize) void {
    for (0..step_count) |i| {
        const row_start = @as(isize, @intCast(start)) + @as(isize, @intCast(i)) * step;
        var next_stone = row_start;
        for (0..row_count) |j| {
            const cur = row_start + @as(isize, @intCast(j)) * row_stride;
            switch (view[@intCast(cur)]) {
                '#' => {
                    next_stone = cur + row_stride;
                },
                'O' => {
                    view[@intCast(cur)] = '.';
                    view[@intCast(next_stone)] = 'O';
                    next_stone += row_stride;
                },
                else => {},
            }
        }
    }
}

const PART1_EXAMPLE =
    \\O....#....
    \\O.OO#....#
    \\.....##...
    \\OO.#O....O
    \\.O.....O#.
    \\O.#..O.#.#
    \\..O..#O..O
    \\.......O..
    \\#....###..
    \\#OO..#....
    \\
;

const SPIN_NORTH_DATA =
    \\OOOO.#.O..
    \\OO..#....#
    \\OO..O##..O
    \\O..#.OO...
    \\........#.
    \\..#....#.#
    \\..O..#.O.O
    \\..O.......
    \\#....###..
    \\#....#....
    \\
;

test "spin step" {
    const buf = std.testing.allocator.alloc(u8, PART1_EXAMPLE.len) catch unreachable;
    defer std.testing.allocator.free(buf);
    @memcpy(buf, PART1_EXAMPLE);
    const gridpat = Grid.new(.{ .mutable = buf }, '\n');

    // spin step north
    spinStep(buf, 0, @intCast(gridpat.stride), gridpat.height, 1, gridpat.width);

    std.debug.print("\nspin north:\n{s}", .{buf});

    try std.testing.expect(std.mem.eql(u8, buf, SPIN_NORTH_DATA));
}

fn spinCycle(grid: *Grid) void {
    const buf = grid.viewMut() catch unreachable;
    // tilt north
    spinStep(buf, 0, @intCast(grid.stride), grid.height, 1, grid.width);
    // tilt west
    spinStep(buf, 0, 1, grid.width, @intCast(grid.stride), grid.height);
    // tilt south
    spinStep(buf, @intCast((grid.height - 1) * grid.stride), -@as(isize, @intCast(grid.stride)), grid.height, 1, grid.width);
    // tilt east
    spinStep(buf, @intCast(grid.width - 1), -1, grid.width, @intCast(grid.stride), grid.height);
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    const gridpat = Grid.new(.{ .immutable = data }, '\n');

    const DESIRED_CYCLES = 1_000_000_000;

    // preallocate a longer buffer, so that when we try to read a line whose width is
    // a multiple of SIMD_WORDS, all the data is still in the buffer
    // (the trailing part is going to be noise from the next line, but we don't care)
    var data_extended = alloc.alloc(u8, data.len + SIMD_WORDS) catch unreachable;
    defer alloc.free(data_extended);
    @memcpy(data_extended[0..data.len], data);

    var loads = try std.ArrayList(u32).initCapacity(alloc, 1000);
    defer loads.deinit();

    var grid = Grid.new(.{ .mutable = data_extended }, '\n');
    grid.height = gridpat.height;

    while (true) {
        spinCycle(&grid);
        const load = northLoad(&grid);
        for (loads.items, 0..) |l, i| {
            if (l == load) {
                const cycles_looping: u64 = DESIRED_CYCLES - i - 1;
                const cycle_len: u64 = loads.items.len - i;
                const result_load_idx = cycles_looping % cycle_len;
                const result = loads.items[@intCast(result_load_idx)];
                return std.fmt.bufPrint(result_buf, "{}", .{result});
            }
        }
        try loads.append(load);
    }

    return error.CycleNotFound;
}
