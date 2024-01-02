const std = @import("std");
const expect = std.testing.expect;
const expectEqual = std.testing.expectEqual;

const MAX_SEGMENTS = 30;
const MAX_WIDTH = 128;

const ONE: u128 = 1;

fn sum(comptime T: type, self: []const T) T {
    var res: T = 0;
    for (self) |n| {
        res += n;
    }
    return res;
}

fn printBin(mask: u128, width: usize) void {
    for (0..width) |i| {
        const ch: u8 = if (mask & (ONE << @intCast(i)) > 0) '#' else '.';
        std.debug.print("{c}", .{ch});
    }
    std.debug.print("\n", .{});
}

const Mask = struct {
    num_mask: u128,
    spaces_mask: u128,
    width_mask: u128,

    pub fn init(num: u32, width: usize) Mask {
        var num_mask: u128 = 0;
        for (0..num) |i| {
            num_mask |= ONE << @intCast(i);
        }
        const spaces_mask: u128 = ONE << @intCast(num + 1) | ONE;

        var width_mask: u128 = 0;
        for (0..width) |i| {
            width_mask |= ONE << @intCast(i);
        }

        return Mask{
            .num_mask = num_mask,
            .spaces_mask = spaces_mask,
            .width_mask = width_mask,
        };
    }

    pub fn testNum(self: Mask, pattern: u128, pos: usize) bool {
        const num_at = self.num_mask << @intCast(pos);
        return (pattern & num_at) == (num_at & self.width_mask);
    }

    pub fn testNumDebug(self: Mask, pattern: u128, pos: usize, width: usize) bool {
        std.debug.print("orig num:   ", .{});
        printBin(self.num_mask, width);
        std.debug.print("moved num:  ", .{});
        const num_at = self.num_mask << @intCast(pos);
        printBin(num_at, width);
        std.debug.print("masked num: ", .{});
        printBin(num_at & self.width_mask, width);
        std.debug.print("pattern:    ", .{});
        printBin(pattern, width);
        std.debug.print("match:      ", .{});
        printBin(pattern & num_at, width);
        return (pattern & num_at) == (num_at & self.width_mask);
    }

    pub fn testSpaces(self: Mask, pattern: u128, pos: usize) bool {
        const spaces_at = ((self.spaces_mask << @intCast(pos)) >> 1) & self.width_mask;
        return (pattern & spaces_at) == spaces_at;
    }
};

const Record = struct {
    width: usize,
    used_mask: u128,
    empty_mask: u128,
    any_mask: u128,
    segments_buf: [MAX_SEGMENTS]u32,
    segments_len: usize,

    pub fn parse(line: []const u8) Record {
        var used_mask: u128 = 0;
        var empty_mask: u128 = 0;
        var any_mask: u128 = 0;
        var segments_buf = [_]u32{0} ** MAX_SEGMENTS;
        // generate the masks and calculate width
        var width: usize = 0;
        for (line, 0..) |c, i| {
            switch (c) {
                ' ' => {
                    width = i;
                    break;
                },
                '#' => used_mask |= ONE << @intCast(i),
                '.' => empty_mask |= ONE << @intCast(i),
                '?' => any_mask |= ONE << @intCast(i),
                else => unreachable,
            }
        }

        // parse the numbers
        var segments_len: usize = 0;
        const numbers = line[width + 1 ..];
        var iter = std.mem.splitScalar(u8, numbers, ',');
        while (iter.next()) |nstr| {
            const n = std.fmt.parseInt(u32, nstr, 10) catch unreachable;
            segments_buf[segments_len] = n;
            segments_len += 1;
        }
        return Record{
            .width = width,
            .used_mask = used_mask,
            .empty_mask = empty_mask,
            .any_mask = any_mask,
            .segments_buf = segments_buf,
            .segments_len = segments_len,
        };
    }

    pub fn expand(self: *Record, comptime factor: u8) void {
        std.debug.assert(self.width * factor <= MAX_WIDTH);
        std.debug.assert(self.segments_len * factor <= MAX_SEGMENTS);

        var used_mask = self.used_mask;
        var empty_mask = self.empty_mask;
        var any_mask = self.any_mask;

        inline for (1..factor) |i| {
            const w = (self.width + 1) * i;
            used_mask |= self.used_mask << @intCast(w);
            empty_mask |= self.empty_mask << @intCast(w);
            any_mask |= self.any_mask << @intCast(w);
            any_mask |= ONE << @intCast(w - 1);

            const s = self.segments_len * i;
            for (0..self.segments_len) |n| {
                self.segments_buf[s + n] = self.segments_buf[n];
            }
        }

        self.used_mask = used_mask;
        self.empty_mask = empty_mask;
        self.any_mask = any_mask;
        self.width = self.width * factor + factor - 1;
        self.segments_len *= factor;
    }

    pub fn dynCountOptions(self: Record) u64 {
        // Segment dynamic buffers.
        var a = [_]u64{0} ** MAX_WIDTH;
        var b = [_]u64{0} ** MAX_WIDTH;

        // "Segment" is a sequence of ###s in the input, specified by a number in the
        // second half of the input.
        // The minimum width of a valid pattern is the sum of all segments + one space
        // between each segment.
        const min_segments_width = sum(u32, self.segments_buf[0..self.segments_len]) + self.segments_len - 1;
        // If all N-1 segments are squished at one end of the pattern, the Nth one
        // can be in one of the P position slots:
        const segment_positions = self.width - min_segments_width + 1;
        // Every segment has P slots, but they are not the same real positions:
        // slot 0 for segment 0 is at offset 0 from start, but slot 0 for segment 1 is
        // at offset (slot 0 width + 1) to account for segment 0 at start, etc.

        // Segment buffers.
        // In this dynamic task, cur[i] represents "how many possible positions are there
        // for the previous segments, given that segment S is at position slot i".
        var prev = &a;
        var cur = &b;

        // Set up prev state.
        // For segment 0, there is exactly 1 option for "previous segments" at every
        // valid position slot.
        const first_segmask = Mask.init(self.segments_buf[0], self.width);
        for (0..segment_positions) |i| {
            if (!self.testEmptyBetween(0, i)) {
                break;
            }
            prev[i] = if (self.testPos(i, first_segmask)) 1 else 0;
        }

        // Position offset. At slot 0, segment 1 comes after the segment 0 width.
        var ofs = self.segments_buf[0] + 1;

        for (1..self.segments_len) |s| {
            const segment = self.segments_buf[s];
            const segmask = Mask.init(segment, self.width);
            for (0..segment_positions) |i| {
                // clear current slot
                cur[i] = 0;
                const valid = self.testPos(ofs + i, segmask);
                if (!valid) {
                    // zero options if the segment can't be placed here
                    continue;
                }
                // Assuming that segment S is at position i, the segment S-1 could be
                // placed on any position slot up to i...
                // ...unless there is a required piece in the empty space.
                // So the number of options for S is the sum of all options for S-1
                // on all the previous slots up to the first one that leaves out
                // the required piece.
                for (0..i + 1) |j| {
                    const empty_start = ofs + i - j;
                    if (!self.testEmptyBetween(empty_start, j)) {
                        break;
                    }
                    cur[i] += prev[i - j];
                }
            }

            // move on to the next segment:
            // 1. update offset
            ofs += segment + 1;
            // 2. cur --> prev
            const tmp = prev;
            prev = cur;
            cur = tmp;
        }

        // correction for the impossible positions of the last segment at the end of the pattern
        // ofs now points to the position of the "next segment" after the last one
        for (0..segment_positions) |i| {
            if (!self.testEmptyBetween(ofs + i, segment_positions - i)) {
                // if the space is not (allowed to be) fully empty until the end of the
                // pattern, the last segment can't be placed here
                prev[i] = 0;
            }
        }

        // all options is the sum of options of the last segment on all position slots
        return sum(u64, prev[0..segment_positions]);
    }

    pub fn enumerateOptions(self: Record) u32 {
        // "Segment" is a sequence of ###s in the input, specified by a number in the
        // second half of the input.
        // The minimum width of a valid pattern is the sum of all segments + one space
        // between each segment.
        const min_segments_width = sum(u32, self.segments_buf[0..self.segments_len]) + self.segments_len - 1;
        // If all N-1 segments are squished at one end of the pattern, the Nth one
        // can be in one of the P position slots:
        const segment_positions = self.width - min_segments_width + 1;
        // Every segment has P slots, but they are not the same real positions:
        // slot 0 for segment 0 is at offset 0 from start, but slot 0 for segment 1 is
        // at offset (slot 0 width + 1) to account for segment 0 at start, etc.

        // now the debug part
        return self.genOptions(0, 0, 0, 0, segment_positions);
    }

    fn genOptions(self: Record, patmask: u128, n: usize, ofs: usize, startpos: usize, segment_positions: usize) u32 {
        if (n >= self.segments_len) {
            if (self.used_mask & patmask != self.used_mask) {
                return 0;
            }
            if (self.empty_mask & ~patmask != self.empty_mask) {
                return 0;
            }
            return 1;
        }
        const seg = self.segments_buf[n];
        const segmask = Mask.init(seg, self.width);

        var total: u32 = 0;

        for (startpos..segment_positions) |pos| {
            if (!self.testPos(ofs + pos, segmask)) {
                continue;
            }
            const newmask = patmask | (segmask.num_mask << @intCast(ofs + pos));
            total += self.genOptions(newmask, n + 1, ofs + seg + 1, pos, segment_positions);
        }
        return total;
    }

    fn numMask(num: u32) u128 {
        var mask: u128 = 0;
        for (0..num) |i| {
            mask |= @as(u128, 1) << @intCast(i);
        }
        return mask;
    }

    fn printBin(self: Record, mask: u128, prev: u64) void {
        for (0..self.width) |i| {
            const ch: u8 = if (mask & (ONE << @intCast(i)) > 0) '#' else '.';
            std.debug.print("{c}", .{ch});
        }
        std.debug.print(" {}\n", .{prev});
    }

    fn printPattern(self: Record) void {
        for (0..self.width) |i| {
            const bit: u128 = ONE << @intCast(i);
            const ch: u8 = if (self.any_mask & bit > 0) '?' else if (self.used_mask & bit > 0) '#' else if (self.empty_mask & bit > 0) '.' else ' ';
            std.debug.print("{c}", .{ch});
        }
        std.debug.print("\n", .{});
    }

    fn testPosAlt(self: Record, pos: usize, num: u32) bool {
        if (pos > 0) {
            if (self.pattern[pos - 1] == '#') {
                return false;
            }
        }
        for (pos..pos + num) |i| {
            if (self.pattern[i] == '.') {
                return false;
            }
        }
        if (pos + num < self.width) {
            if (self.pattern[pos + num] == '#') {
                return false;
            }
        }
        return true;
    }

    fn testEmptyBetween(self: Record, start: usize, shift: usize) bool {
        const used_mask = self.used_mask >> @intCast(start);
        const clear_mask = ~(~@as(u128, 0) << @intCast(shift));
        const res = used_mask & clear_mask == 0;
        return res;
    }

    fn testPos(self: Record, pos: usize, mask: Mask) bool {
        // std.debug.print("TEST:\n", .{});
        // self.printPattern();
        // std.debug.print("\n", .{});
        // std.debug.print("\n", .{});
        const used_mask = self.used_mask | self.any_mask;
        const empty_mask = self.empty_mask | self.any_mask;
        const ret = mask.testNum(used_mask, pos) and mask.testSpaces(empty_mask, pos);
        // if (ret) {
        //     const mmask = mask.num_mask << @intCast(pos);
        //     self.printBin(mmask);
        //     std.debug.print(" OK\n", .{});
        // } else {
        //     std.debug.print(" FAIL\n", .{});
        // }
        return ret;
    }
};

test "test" {
    const in = "?###???????? 3,2,1";
    try expectEqual(Record.parse(in).enumerateOptions(), 10);
}

test "part1 example" {
    const DATA =
        \\???.### 1,1,3
        \\.??..??...?##. 1,1,3
        \\?#?#?#?#?#?#?#? 1,3,1,6
        \\????.#...#... 4,1,1
        \\????.######..#####. 1,6,5
        \\?###???????? 3,2,1
    ;
    var result_buf = [_]u8{0} ** 64;
    const result = try part1(DATA, std.testing.allocator, &result_buf);
    try expect(std.mem.eql(u8, result, "21"));
}

test "manual expansion" {
    const in = "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3";
    try expectEqual(Record.parse(in).enumerateOptions(), 1);
}

test "dyn counterexample" {
    const in = "??#???? 1,1";
    const rec = Record.parse(in);
    const good_result = rec.enumerateOptions();
    const test_result = rec.dynCountOptions();
    try expectEqual(good_result, @intCast(test_result));
}

test "dyn failing case" {
    const in = ".#?????####.?.#? 1,1,5,1";
    const rec = Record.parse(in);
    const good_result = rec.enumerateOptions();
    const test_result = rec.dynCountOptions();
    try expectEqual(good_result, @intCast(test_result));
}

test "part2" {
    const in = "???.### 1,1,3";
    const expanded = "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3";
    var rec = Record.parse(in);
    rec.expand(5);
    const expected = Record.parse(expanded);
    try expectEqual(rec.used_mask, expected.used_mask);
    try expectEqual(rec.empty_mask, expected.empty_mask);
    try expectEqual(rec.any_mask, expected.any_mask);
    try expectEqual(rec.width, expected.width);
    try expectEqual(rec.segments_len, expected.segments_len);
    try expectEqual(rec.enumerateOptions(), 1);
}

test "part2more" {
    const in = ".??..??...?##. 1,1,3";
    var rec = Record.parse(in);
    rec.expand(5);
    try expectEqual(rec.enumerateOptions(), 16384);
}

pub fn part1(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    _ = alloc;

    var total: usize = 0;
    var lines_iter = std.mem.tokenizeScalar(u8, data, '\n');
    while (lines_iter.next()) |line| {
        const result = Record.parse(line).dynCountOptions();
        total += result;
    }

    return std.fmt.bufPrint(result_buf, "{}", .{total});
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    _ = alloc;

    var total: usize = 0;
    var lines_iter = std.mem.tokenizeScalar(u8, data, '\n');
    while (lines_iter.next()) |line| {
        var rec = Record.parse(line);
        rec.expand(5);
        const result = rec.dynCountOptions();
        total += result;
    }

    return std.fmt.bufPrint(result_buf, "{}", .{total});
}
