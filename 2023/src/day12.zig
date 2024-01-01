const std = @import("std");
const expect = std.testing.expect;
const expectEqual = std.testing.expectEqual;

const MAX_SEGMENTS = 10;
const MAX_WIDTH = 64;

const ONE: u64 = 1;

fn sum(self: []const u32) u32 {
    var res: u32 = 0;
    for (self) |n| {
        res += n;
    }
    return res;
}

fn printBin(mask: u64, width: usize) void {
    for (0..width) |i| {
        const ch: u8 = if (mask & (ONE << @intCast(i)) > 0) '#' else '.';
        std.debug.print("{c}", .{ch});
    }
    std.debug.print("\n", .{});
}

const Mask = struct {
    num_mask: u64,
    spaces_mask: u64,
    width_mask: u64,

    pub fn init(num: u32, width: usize) Mask {
        var num_mask: u64 = 0;
        for (0..num) |i| {
            num_mask |= ONE << @intCast(i);
        }
        const spaces_mask: u64 = ONE << @intCast(num + 1) | ONE;

        var width_mask: u64 = 0;
        for (0..width) |i| {
            width_mask |= ONE << @intCast(i);
        }

        return Mask{
            .num_mask = num_mask,
            .spaces_mask = spaces_mask,
            .width_mask = width_mask,
        };
    }

    pub fn testNum(self: Mask, pattern: u64, pos: usize) bool {
        const num_at = self.num_mask << @intCast(pos);
        return (pattern & num_at) == (num_at & self.width_mask);
    }

    pub fn testNumDebug(self: Mask, pattern: u64, pos: usize, width: usize) bool {
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

    pub fn testSpaces(self: Mask, pattern: u64, pos: usize) bool {
        const spaces_at = ((self.spaces_mask << @intCast(pos)) >> 1) & self.width_mask;
        return (pattern & spaces_at) == spaces_at;
    }
};

const Record = struct {
    width: usize,
    pattern: []const u8,
    can_be_used_mask: u64,
    can_be_empty_mask: u64,
    segments_buf: [MAX_SEGMENTS]u32,
    segments_len: usize,

    pub fn parse(line: []const u8) Record {
        var can_be_used_mask: u64 = 0;
        var can_be_empty_mask: u64 = 0;
        var segments_buf = [_]u32{0} ** MAX_SEGMENTS;
        // generate the masks and calculate width
        var width: usize = 0;
        for (line, 0..) |c, i| {
            switch (c) {
                ' ' => {
                    width = i;
                    break;
                },
                '#' => can_be_used_mask |= ONE << @intCast(i),
                '.' => can_be_empty_mask |= ONE << @intCast(i),
                '?' => {
                    can_be_used_mask |= ONE << @intCast(i);
                    can_be_empty_mask |= ONE << @intCast(i);
                },
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
            .pattern = line[0..width],
            .width = width,
            .can_be_used_mask = can_be_used_mask,
            .can_be_empty_mask = can_be_empty_mask,
            .segments_buf = segments_buf,
            .segments_len = segments_len,
        };
    }

    pub fn dynCountOptions(self: Record) u32 {
        // Segment dynamic buffers.
        var a = [_]u32{0} ** MAX_WIDTH;
        var b = [_]u32{0} ** MAX_WIDTH;

        // "Segment" is a sequence of ###s in the input, specified by a number in the
        // second half of the input.
        // The minimum width of a valid pattern is the sum of all segments + one space
        // between each segment.
        const min_segments_width = sum(self.segments_buf[0..self.segments_len]) + self.segments_len - 1;
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
            prev[i] = if (self.testPos(i, first_segmask)) 1 else 0;
        }

        // Position offset. At slot 0, segment 1 comes after the segment 0 width.
        var ofs = self.segments_buf[0] + 1;

        for (1..self.segments_len) |s| {
            const segment = self.segments_buf[s];
            const segmask = Mask.init(segment, self.width);
            // reset valid options
            var valid_options: u32 = 0;
            for (0..segment_positions) |i| {
                // Assuming that segment S is at position i, the segment S-1 could be
                // placed on any position slot up to i.
                // So the number of options for S is the sum of all options for S-1
                // on all the previous slots.
                // valid_options is an accumulator for this number.
                valid_options += prev[i];
                const valid = self.testPos(ofs + i, segmask);
                cur[i] = if (valid) valid_options else 0;
            }

            // move on to the next segment:
            // 1. update offset
            ofs += segment + 1;
            // 2. cur --> prev
            const tmp = prev;
            prev = cur;
            cur = tmp;
        }

        // all options is the sum of options of the last segment on all position slots
        return sum(prev[0..segment_positions]);
    }

    fn numMask(num: u32) u64 {
        var mask: u64 = 0;
        for (0..num) |i| {
            mask |= @as(u64, 1) << @intCast(i);
        }
        return mask;
    }

    fn printBin(self: Record, mask: u64) void {
        for (0..self.width + 3) |i| {
            const ch: u8 = if (mask & (ONE << @intCast(i)) > 0) '#' else '.';
            std.debug.print("{c}", .{ch});
        }
        std.debug.print("\n", .{});
    }

    fn printPattern(self: Record) void {
        for (0..self.width) |i| {
            const bit: u64 = ONE << @intCast(i);
            const ch: u8 = if (self.can_be_used_mask & bit > 0 and self.can_be_empty_mask & bit > 0) '?' else if (self.can_be_used_mask & bit > 0) '#' else if (self.can_be_empty_mask & bit > 0) '.' else ' ';
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

    fn testPos(self: Record, pos: usize, mask: Mask) bool {
        // std.debug.print("TEST:\n", .{});
        // self.printPattern();
        // std.debug.print("\n", .{});
        // std.debug.print("\n", .{});
        const ret = mask.testNum(self.can_be_used_mask, pos) and mask.testSpaces(self.can_be_empty_mask, pos);
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
    try expectEqual(Record.parse(in).dynCountOptions(), 10);
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
    _ = data;
    _ = alloc;
    return result_buf;
}
