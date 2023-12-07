const std = @import("std");

const CARDS = "23456789TJQKA";
const JCARDS = "J23456789TQKA";

const Hand = struct {
    text: []const u8,
    values: [5]u8,
    class: u8,

    bid: usize,

    fn _computeValues(self: *Hand, joker: bool) void {
        const cards = if (joker) JCARDS else CARDS;
        for (self.text, 0..) |ch, i| {
            const idx = std.mem.indexOfScalar(u8, cards, ch).?;
            self.values[i] = @as(u8, @intCast(idx));
        }
    }

    fn _computeClass(self: *Hand, joker: bool) void {
        var counter_incl_joker = [_]u8{0} ** CARDS.len;
        for (self.values) |value| {
            counter_incl_joker[value] += 1;
        }
        var counter: []u8 = undefined;
        var joker_count: u8 = 0;
        if (joker) {
            counter = counter_incl_joker[1..];
            joker_count = counter_incl_joker[0];
        } else {
            counter = counter_incl_joker[0..];
        }
        std.mem.sort(u8, counter, void{}, std.sort.desc(u8));
        self.class = switch (counter[0] + joker_count) {
            5 => 7, // five of a kind
            4 => 6, // four of a kind
            3 => blk: {
                if (counter[1] == 2) { // full house
                    break :blk 5;
                } else {
                    break :blk 4; // three of a kind
                }
            },
            2 => blk: {
                if (counter[1] == 2) { // two pair
                    break :blk 3;
                } else {
                    break :blk 2; // one pair
                }
            },
            1 => 1, // high card
            else => unreachable,
        };
    }

    pub fn build(text: []const u8, joker: bool) Hand {
        var tok = std.mem.tokenizeScalar(u8, text, ' ');
        var hand = Hand{
            .text = tok.next().?,
            .class = 0,
            .values = undefined,
            .bid = std.fmt.parseInt(usize, tok.next().?, 10) catch unreachable,
        };
        hand._computeValues(joker);
        hand._computeClass(joker);
        return hand;
    }

    pub fn lessThan(_: void, self: Hand, other: Hand) bool {
        if (self.class != other.class) {
            return self.class < other.class;
        }
        for (self.values, other.values) |a, b| {
            if (a != b) {
                return a < b;
            }
        }
        return false;
    }
};

pub fn cardTotal(data: []const u8, alloc: std.mem.Allocator, joker: bool) !usize {
    var hands = std.ArrayList(Hand).init(alloc);
    defer hands.deinit();

    var lines_iter = std.mem.tokenizeScalar(u8, data, '\n');
    while (lines_iter.next()) |line| {
        try hands.append(Hand.build(line, joker));
    }

    var total: usize = 0;
    std.mem.sort(Hand, hands.items, void{}, Hand.lessThan);
    for (hands.items, 1..) |hand, rank| {
        total += hand.bid * rank;
    }
    return total;
}

pub fn part1(data: []const u8, alloc: std.mem.Allocator) !void {
    const total = try cardTotal(data, alloc, false);
    std.debug.print("Day 7 part 1: {}\n", .{total});
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator) !void {
    const total = try cardTotal(data, alloc, true);
    std.debug.print("Day 7 part 2: {}\n", .{total});
}
