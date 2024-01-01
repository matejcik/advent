const std = @import("std");
const gridlib = @import("gridlib");

const Point = gridlib.Point;
const Grid = gridlib.Grid(u8);
const Direction = gridlib.Direction;

const SIMD_SIZE_BYTES = 256 / 8;
const SIMD_WORDS = SIMD_SIZE_BYTES / @sizeOf(u32);
const Vector = @Vector(SIMD_WORDS, u32);

const Expansion = struct {
    cols: std.ArrayList(usize),
    rows: std.ArrayList(usize),

    pub fn deinit(self: Expansion) void {
        self.cols.deinit();
        self.rows.deinit();
    }
};

fn expandUniverse(universe: *const Grid, alloc: std.mem.Allocator, factor: usize) !Expansion {
    const simd_cols = try std.math.divCeil(usize, universe.width, SIMD_WORDS);
    const simd_cols_full = universe.width / SIMD_WORDS;

    const row_expected_total = '.' * universe.width;
    const col_expected_total = '.' * universe.height;

    var col_sums_simd = try std.ArrayList(Vector).initCapacity(alloc, simd_cols);
    defer col_sums_simd.deinit();
    var row_sums = try std.ArrayList(u32).initCapacity(alloc, universe.height);
    defer row_sums.deinit();

    // set up the vectors
    for (0..simd_cols) |_| {
        col_sums_simd.appendAssumeCapacity(@splat(0));
    }

    var segment_buf = [_]u8{0} ** SIMD_WORDS;

    // process row by row
    for (0..universe.height) |y| {
        var line = universe.line(y).?;
        var row_sum: u32 = 0;

        // deal with full segments
        for (0..simd_cols_full) |x| {
            // create the SIMD vector out of the next SIMD_WORDS cells
            const simd_x = x * SIMD_WORDS;
            const slice = line[simd_x..];
            const vec: Vector = slice[0..SIMD_WORDS].*;

            // add the sum of the segment to the row sum
            row_sum += @reduce(.Add, vec);

            // add the vector to the matching column sum
            col_sums_simd.items[x] += vec;
        }

        // deal with the leftover
        if (simd_cols_full != simd_cols) {
            const leftover_start = simd_cols_full * SIMD_WORDS;
            const leftover = line[leftover_start..];
            @memcpy(segment_buf[0..leftover.len], leftover);
            // we don't need to clear the leftover because it's always going to be
            // copied over the same size and there will be zeroes in the tail
            const vec: Vector = segment_buf;

            // add the sum of the segment to the row sum
            row_sum += @reduce(.Add, vec);

            // add the vector to the matching column sum
            col_sums_simd.items[simd_cols_full] += vec;
        }

        row_sums.appendAssumeCapacity(row_sum);
    }

    // mark expanded columns
    var cy: usize = 0;
    var cols = try std.ArrayList(usize).initCapacity(alloc, simd_cols * SIMD_WORDS);
    for (col_sums_simd.items) |simd| {
        for (0..SIMD_WORDS) |i| {
            cols.appendAssumeCapacity(cy);
            cy += if (simd[i] == col_expected_total) factor else 1;
        }
    }

    // mark expanded rows
    var cx: usize = 0;
    var rows = try std.ArrayList(usize).initCapacity(alloc, universe.height);
    for (row_sums.items) |r| {
        rows.appendAssumeCapacity(cx);
        cx += if (r == row_expected_total) factor else 1;
    }

    return Expansion{
        .cols = cols,
        .rows = rows,
    };
}

fn distancesExpanded(data: []const u8, alloc: std.mem.Allocator, factor: usize) !usize {
    const universe = Grid.new(.{ .immutable = data }, '\n');

    const expansion = try expandUniverse(&universe, alloc, factor);
    defer expansion.deinit();

    var galaxies = try std.ArrayList(Point).initCapacity(alloc, universe.width * universe.height / 10);
    defer galaxies.deinit();

    // identify all galaxies
    var pos: usize = 0;
    const view = universe.view();
    while (true) {
        const new_pos = std.mem.indexOfScalarPos(u8, view, pos, '#');
        if (new_pos) |p| {
            pos = p + 1;
            const point = universe.indexToPoint(p);
            const translated = Point{
                .x = @intCast(expansion.cols.items[@intCast(point.x)]),
                .y = @intCast(expansion.rows.items[@intCast(point.y)]),
            };
            try galaxies.append(translated);
        } else {
            break;
        }
    }

    var total: usize = 0;
    for (0..galaxies.items.len - 1) |i| {
        for (i + 1..galaxies.items.len) |j| {
            const a = galaxies.items[i];
            const b = galaxies.items[j];
            total += a.gridDistance(b);
        }
    }

    return total;
}

pub fn part1(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    const total = try distancesExpanded(data, alloc, 2);
    return std.fmt.bufPrint(result_buf, "{}", .{total});
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    const total = try distancesExpanded(data, alloc, 1_000_000);
    return std.fmt.bufPrint(result_buf, "{}", .{total});
}
