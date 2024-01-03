const std = @import("std");
const gridlib = @import("gridlib");
const Grid = gridlib.Grid(u8);
const Point = gridlib.Point;

const MAX_WIDTH = 128;

const SIMD_WIDTH_BITS = 256;
const SIMD_WORDS = SIMD_WIDTH_BITS / (8 * @sizeOf(u8));

const BoolVec = @Vector(SIMD_WORDS, bool);
const U8Vec = @Vector(SIMD_WORDS, u8);
const U32Vec = @Vector(SIMD_WORDS, u32);

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

    return sum(u32, &totals);
}

pub fn part1vec(data: []const u8, alloc: std.mem.Allocator) u32 {
    // preallocate a longer buffer, so that when we try to read a line whose width is
    // a multiple of SIMD_WORDS, all the data is still in the buffer
    // (the trailing part is going to be noise from the next line, but we don't care)
    var data_extended = alloc.alloc(u8, data.len + MAX_WIDTH) catch unreachable;
    defer alloc.free(data_extended);
    @memcpy(data_extended[0..data.len], data);

    const grid = Grid.new(.{ .immutable = data_extended }, '\n');

    // set up the vectors:
    // breaks are the #stops for each column
    var breaks = [_]U8Vec{@splat(@intCast(grid.height))} ** (MAX_WIDTH / SIMD_WORDS);
    // stones are the number of stones already piled on the previous break
    var stones = [_]U8Vec{@splat(0)} ** (MAX_WIDTH / SIMD_WORDS);
    // totals are the sum of the stone values
    var totals = [_]U32Vec{@splat(0)} ** (MAX_WIDTH / SIMD_WORDS);

    // number of vectors that fit the whole line
    const elems = (grid.width + SIMD_WORDS - 1) / SIMD_WORDS;

    for (0..grid.height) |y| {
        // grab index of the first element of the line
        const idx = grid.pointToIndex(.{ .x = 0, .y = @intCast(y) }).?;
        // grab the line buffer. this should be the line plus some data from the next line at the end
        const line = grid.view()[idx..][0 .. elems * SIMD_WORDS];
        // vec of next values
        const simd_next_value: U8Vec = @splat(@intCast(grid.height - y - 1));

        for (0..elems) |elem| {
            const ofs = elem * SIMD_WORDS;
            const segment = line[ofs..][0..SIMD_WORDS];
            const simd_segment: U8Vec = segment.*;
            const simd_blocks = simd_segment == @as(U8Vec, @splat('#'));
            const simd_stones = simd_segment == @as(U8Vec, @splat('O'));
            // if char == '0' {
            //     totals[x] += breaks[x] - stones[x];
            totals[elem] += @select(u8, simd_stones, breaks[elem] - stones[elem], ZEROES);
            //     stones[x] += 1;
            stones[elem] += @select(u8, simd_stones, ONES, ZEROES);
            // } else if char == '#' {
            //     stones[x] = 0;
            stones[elem] = @select(u8, simd_blocks, ZEROES, stones[elem]);
            //     breaks[x] = @intCast(value - 1);
            breaks[elem] = @select(u8, simd_blocks, simd_next_value, breaks[elem]);
        }

        // for (line, 0..) |ch, x| {
        //     switch (ch) {
        //         '#' => {
        //             breaks[x] = @intCast(value - 1);
        //             stones[x] = 0;
        //         },
        //         'O' => {
        //             totals[x] += breaks[x] - stones[x];
        //             stones[x] += 1;
        //         },
        //         else => {},
        //     }
        // }
    }
    // collect totals
    var total: u32 = 0;
    for (0..grid.width / SIMD_WORDS) |i| {
        total += @reduce(.Add, totals[i]);
    }
    if (grid.width % SIMD_WORDS != 0) {
        const lastvec = totals[grid.width / SIMD_WORDS];
        for (0..grid.width % SIMD_WORDS) |i| {
            total += lastvec[i];
        }
    }

    return total;
}

pub fn part1(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    // _ = alloc;
    // const total = part1novec(data);

    const total = part1vec(data, alloc);
    return std.fmt.bufPrint(result_buf, "{}", .{total});
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    _ = data;
    _ = alloc;
    return result_buf;
}
