const std = @import("std");
const print = std.debug.print;
const expect = std.testing.expect;
const expectEqual = std.testing.expectEqual;
const ArrayList = std.ArrayList;
const sliceEql = @import("advent.zig").sliceEql;

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

    pub fn splitAt(self: Range, pivot: isize) struct { ?Range, ?Range } {
        if (self.start > pivot) {
            return .{ null, self };
        }
        if (self.end < pivot) {
            return .{ self, null };
        }
        return .{
            .{ .start = self.start, .end = pivot - 1 },
            .{ .start = pivot, .end = self.end },
        };
    }

    pub fn contains(self: Range, other: Range) bool {
        return self.start <= other.start and self.end >= other.end;
    }

    pub fn containsScalar(self: Range, scalar: isize) bool {
        return self.start <= scalar and self.end >= scalar;
    }

    pub fn lessThan(_: void, self: Range, other: Range) bool {
        return self.start < other.start;
    }

    /// Merge overlapping ranges in-place.
    pub fn merge(ranges_: *[]Range) void {
        if (ranges_.len < 2) {
            return;
        }
        var ranges = ranges_.*;
        std.mem.sort(Range, ranges, void{}, Range.lessThan);
        var i: usize = 0;
        var nxt: usize = 1;
        while (nxt < ranges.len) {
            if (ranges[i].end >= ranges[nxt].start and ranges[i].end <= ranges[nxt].end) {
                ranges[i].end = ranges[nxt].end;
                nxt += 1;
            } else {
                i += 1;
                ranges[i] = ranges[nxt];
                nxt += 1;
            }
        }
        // truncate the destination array
        ranges_.*.len = i + 1;
    }
};

test "adjacent range merge" {
    var input = [_]Range{
        .{ .start = 1, .end = 3 },
        .{ .start = 3, .end = 5 },
        .{ .start = 5, .end = 7 },
    };
    const expected = [_]Range{
        .{ .start = 1, .end = 7 },
    };

    var input_slice = @as([]Range, input[0..]);

    Range.merge(&input_slice);
    try expectEqual(input_slice.len, 1);
    try expect(sliceEql(Range, input_slice, &expected));
}

test "non-adjacent range merge" {
    var input = [_]Range{
        .{ .start = 1, .end = 2 },
        .{ .start = 2, .end = 3 },
        .{ .start = 4, .end = 5 },
        .{ .start = 5, .end = 6 },
        .{ .start = 7, .end = 8 },
        .{ .start = 8, .end = 9 },
    };
    const expected = [_]Range{
        .{ .start = 1, .end = 3 },
        .{ .start = 4, .end = 6 },
        .{ .start = 7, .end = 9 },
    };

    var input_slice = @as([]Range, input[0..]);
    Range.merge(&input_slice);

    try expect(sliceEql(Range, input_slice, &expected));
}

test "overlapping range merge" {
    var input = [_]Range{
        .{ .start = 1, .end = 3 },
        .{ .start = 2, .end = 4 },
        .{ .start = 3, .end = 5 },
        .{ .start = 4, .end = 6 },
        .{ .start = 5, .end = 7 },
        .{ .start = 6, .end = 8 },
    };
    const expected = [_]Range{
        .{ .start = 1, .end = 8 },
    };

    var input_slice = @as([]Range, input[0..]);
    Range.merge(&input_slice);

    try expect(sliceEql(Range, input_slice, &expected));
}

const RangeMapping = struct {
    range: Range,
    dest: isize,

    pub fn mapScalar(self: RangeMapping, scalar: isize) ?isize {
        if (self.range.containsScalar(scalar)) {
            return self.dest + scalar - self.range.start;
        }
        return null;
    }

    pub fn mapRange(self: RangeMapping, range: Range) ?Range {
        if (self.range.intersection(range)) |segment| {
            return Range{
                .start = self.dest + segment.start - self.range.start,
                .end = self.dest + segment.end - self.range.start,
            };
        }
        return null;
    }

    pub fn lessThan(_: void, self: RangeMapping, other: RangeMapping) bool {
        return Range.lessThan(void{}, self.range, other.range);
    }
};

pub fn parseSection(data: []const u8, alloc: std.mem.Allocator) !ArrayList(RangeMapping) {
    var mappings = ArrayList(RangeMapping).init(alloc);

    var line_iter = std.mem.tokenizeScalar(u8, data, '\n');
    _ = line_iter.next() orelse return error.InvalidInput; // skip header line
    while (line_iter.next()) |line| {
        var numIter = std.mem.tokenizeScalar(u8, line, ' ');
        const dest = try std.fmt.parseInt(isize, numIter.next().?, 10);
        const start = try std.fmt.parseInt(isize, numIter.next().?, 10);
        const length = try std.fmt.parseInt(isize, numIter.next().?, 10);
        const m = RangeMapping{
            .range = Range.new(start, length),
            .dest = dest,
        };
        try mappings.append(m);
    }
    std.mem.sort(RangeMapping, mappings.items, void{}, RangeMapping.lessThan);
    return mappings;
}

fn mapSeedInSection(seed: isize, section: []const RangeMapping) isize {
    for (section) |mapping| {
        if (mapping.mapScalar(seed)) |mapped| {
            return mapped;
        }
    }
    return seed;
}

pub fn getMinLocation(data: []const u8, alloc: std.mem.Allocator) !isize {
    var sections = ArrayList(ArrayList(RangeMapping)).init(alloc);
    defer {
        for (sections.items) |section| {
            section.deinit();
        }
        sections.deinit();
    }

    var section_iter = std.mem.tokenizeSequence(u8, data, "\n\n");
    const seed_section = section_iter.next() orelse return error.InvalidInput;
    while (section_iter.next()) |section| {
        const section_ranges = try parseSection(section, alloc);
        try sections.append(section_ranges);
    }

    var min_location: isize = std.math.maxInt(isize);

    var seed_iter = std.mem.tokenizeScalar(u8, seed_section, ' ');
    _ = seed_iter.next() orelse return error.InvalidInput; // skip "seeds:"
    while (seed_iter.next()) |seed_str| {
        const seed = try std.fmt.parseInt(isize, seed_str, 10);
        var mapped_seed = seed;
        for (sections.items) |section| {
            mapped_seed = mapSeedInSection(mapped_seed, section.items);
        }
        min_location = @min(min_location, mapped_seed);
    }
    return min_location;
}

pub fn part1(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    const min_location = try getMinLocation(data, alloc);
    return std.fmt.bufPrint(result_buf, "{}", .{min_location});
}

const TEST_DATA =
    \\seeds: 79 14 55 13
    \\
    \\seed-to-soil map:
    \\50 98 2
    \\52 50 48
    \\
    \\soil-to-fertilizer map:
    \\0 15 37
    \\37 52 2
    \\39 0 15
    \\
    \\fertilizer-to-water map:
    \\49 53 8
    \\0 11 42
    \\42 0 7
    \\57 7 4
    \\
    \\water-to-light map:
    \\88 18 7
    \\18 25 70
    \\
    \\light-to-temperature map:
    \\45 77 23
    \\81 45 19
    \\68 64 13
    \\
    \\temperature-to-humidity map:
    \\0 69 1
    \\1 0 69
    \\
    \\humidity-to-location map:
    \\60 56 37
    \\56 93 4
    \\
;

test "part1 test data" {
    const result = try getMinLocation(TEST_DATA, std.testing.allocator);
    try expectEqual(result, 35);
}

test "part2 test data" {
    const result = try getMinLocation2(TEST_DATA, std.testing.allocator);
    try expectEqual(result, 46);
}

pub fn getMinLocation2(data: []const u8, alloc: std.mem.Allocator) !isize {
    var sections = ArrayList(ArrayList(RangeMapping)).init(alloc);
    defer {
        for (sections.items) |section| {
            section.deinit();
        }
        sections.deinit();
    }

    var section_iter = std.mem.tokenizeSequence(u8, data, "\n\n");
    const seed_section = section_iter.next() orelse return error.InvalidInput;
    while (section_iter.next()) |section| {
        const section_ranges = try parseSection(section, alloc);
        try sections.append(section_ranges);
    }

    var seed_ranges = ArrayList(Range).init(alloc);
    defer seed_ranges.deinit();
    var new_seed_ranges = ArrayList(Range).init(alloc);
    defer new_seed_ranges.deinit();

    // parse the seeds as ranges
    var seed_iter = std.mem.tokenizeScalar(u8, seed_section, ' ');
    _ = seed_iter.next() orelse return error.InvalidInput; // skip "seeds:"
    while (seed_iter.next()) |start_str| {
        const start = try std.fmt.parseInt(isize, start_str, 10);
        const length = try std.fmt.parseInt(isize, seed_iter.next().?, 10);
        try seed_ranges.append(Range.new(start, length));
    }
    std.mem.sort(Range, seed_ranges.items, void{}, Range.lessThan);

    var cur = &seed_ranges;
    var next = &new_seed_ranges;
    for (sections.items) |section| {
        next.clearRetainingCapacity();
        var s: usize = 0; // seed index
        var m: usize = 0; // mapping index
        while (s < cur.items.len) {
            var sr = &cur.items[s];
            if (m >= section.items.len) {
                // no more mappings, just copy the rest of the seeds
                try next.append(sr.*);
                s += 1;
            } else if (sr.start < section.items[m].range.start) {
                // seed range starts before mapping range
                const split = sr.splitAt(section.items[m].range.start);
                try next.append(split[0].?); // we know the range starts left of the split point so this will always be set
                if (split[1]) |r| {
                    sr.* = r;
                } else {
                    s += 1;
                }
            } else {
                if (sr.start <= section.items[m].range.end) {
                    // seed range starts somewhere inside the mapping range
                    const split = sr.splitAt(section.items[m].range.end + 1);
                    const mapped = section.items[m].mapRange(split[0].?).?;
                    try next.append(mapped);
                    if (split[1]) |r| {
                        sr.* = r;
                    } else {
                        s += 1;
                    }
                } else {
                    // go to the next mapping
                    m += 1;
                }
            }
        }

        // sort and merge the new array
        Range.merge(&next.items);
        // swap cur/next
        const tmp = next;
        next = cur;
        cur = tmp;
    }

    return cur.items[0].start;
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    const min_location = try getMinLocation2(data, alloc);
    return std.fmt.bufPrint(result_buf, "{}", .{min_location});
}
