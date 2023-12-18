const advent = @import("advent");
const std = @import("std");

pub fn part1(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    _ = alloc;
    var first_digit: u8 = 0;
    var last_digit: u8 = 0;
    var total: u32 = 0;
    for (data) |byte| {
        switch (byte) {
            '1'...'9' => {
                if (first_digit == 0) {
                    first_digit = byte - '0';
                }
                last_digit = byte - '0';
            },
            '\n' => {
                total += first_digit * 10 + last_digit;
                first_digit = 0;
                last_digit = 0;
            },
            else => {},
        }
    }
    return std.fmt.bufPrint(result_buf, "{}", .{total});
}

const TRIE_SIZE = 10000;

fn Trie(comptime T: type) type {
    return struct {
        nodes: [TRIE_SIZE]TrieNode,
        next_node: usize,

        const TrieNode = struct {
            children: [96]?usize,
            fail: usize,
            is_end_of: ?NodeValue,

            fn init() TrieNode {
                var new = TrieNode{
                    .children = undefined,
                    .fail = 0,
                    .is_end_of = null,
                };
                @memset(&new.children, null);
                return new;
            }

            fn find(self: *const TrieNode, ch: u8) ?usize {
                const i = ch - ' ';
                return self.children[i];
            }

            fn set(self: *TrieNode, ch: u8, value: usize) void {
                const i = ch - ' ';
                self.children[i] = value;
            }
        };

        const NodeValue = struct {
            length: usize,
            value: T,
        };

        fn init() Trie(T) {
            var trie = Trie(T){
                .nodes = undefined,
                .next_node = 1,
            };
            @memset(&trie.nodes, TrieNode.init());
            return trie;
        }

        fn lookup(self: *const Trie(T), cursor: usize, ch: u8) usize {
            const ptr = self.node(cursor);
            if (ptr.find(ch)) |child| {
                return child;
            } else if (ptr.fail != cursor) {
                return self.lookup(ptr.fail, ch);
            } else {
                return cursor;
            }
        }

        fn lookupString(self: *const Trie(T), start: usize, str: []const u8) usize {
            var cursor = start;
            for (str) |ch| {
                cursor = self.lookup(cursor, ch);
            }
            return cursor;
        }

        fn is_end_of(self: *const Trie(T), cursor: usize) ?NodeValue {
            return self.node(cursor).is_end_of;
        }

        fn node(self: *const Trie(T), index: usize) *const TrieNode {
            return &self.nodes[index];
        }

        fn root(self: *const Trie(T)) *const TrieNode {
            return self.node(0);
        }

        pub fn addWord(self: *Trie(T), word: []const u8, value: T) void {
            var ptr = &self.nodes[0];
            for (word) |c| {
                if (ptr.find(c) == null) {
                    ptr.set(c, self.next_node);
                    self.next_node += 1;
                }
                ptr = &self.nodes[ptr.find(c) orelse unreachable];
            }
            ptr.is_end_of = .{ .length = word.len, .value = value };
        }

        pub fn buildBacklinks(self: *Trie(T)) void {
            var queue = std.fifo.LinearFifo(usize, .{ .Static = self.next_node }).init();
            queue.writeItem(0) catch unreachable;
            @setEvalBranchQuota(self.next_node * (96 * 2));
            while (queue.readItem()) |cursor| {
                const ptr = &self.nodes[cursor];
                for (ptr.children, 0..) |child_, c| {
                    const child = child_ orelse continue;
                    queue.writeItem(child) catch unreachable;
                    if (cursor != 0) {
                        self.nodes[child].fail = self.lookup(ptr.fail, c + ' ');
                    }
                }
            }
        }

        pub fn withWords(words: []const Word) Trie(u32) {
            var trie = Trie(T).init();
            for (words) |word| {
                trie.addWord(word.word, word.value);
            }
            trie.buildBacklinks();
            return trie;
        }
    };
}

const Word = struct {
    word: []const u8,
    value: u32,

    pub fn new(word: []const u8, value: u32) Word {
        return Word{ .word = word, .value = value };
    }
};

const WORDS = [_]Word{
    Word.new("1", 1),
    Word.new("2", 2),
    Word.new("3", 3),
    Word.new("4", 4),
    Word.new("5", 5),
    Word.new("6", 6),
    Word.new("7", 7),
    Word.new("8", 8),
    Word.new("9", 9),
    Word.new("one", 1),
    Word.new("two", 2),
    Word.new("three", 3),
    Word.new("four", 4),
    Word.new("five", 5),
    Word.new("six", 6),
    Word.new("seven", 7),
    Word.new("eight", 8),
    Word.new("nine", 9),
};

const TRIE = Trie(u32).withWords(&WORDS);

//build_trie(&WORDS) catch unreachable;

const expect = std.testing.expect;

test "trie should go home on unknown character" {
    try expect(TRIE.lookup(0, 'a') == 0);
}

test "trie should be ok with overlaps" {
    const trie = comptime Trie(u32).withWords(&[_]Word{
        Word.new("foo", 1),
        Word.new("oof", 2),
    });

    var cursor = trie.lookupString(0, "foo");
    try expect((trie.is_end_of(cursor) orelse unreachable).value == 1);

    cursor = trie.lookupString(0, "oof");
    try expect((trie.is_end_of(cursor) orelse unreachable).value == 2);

    cursor = trie.lookupString(0, "foooof");
    try expect((trie.is_end_of(cursor) orelse unreachable).value == 2);

    cursor = trie.lookupString(0, "fooof");
    try expect((trie.is_end_of(cursor) orelse unreachable).value == 2);

    cursor = trie.lookupString(0, "foof");
    try expect((trie.is_end_of(cursor) orelse unreachable).value == 2);
}

test "trie should find words" {
    var cursor = TRIE.lookupString(0, "one");
    try expect((TRIE.is_end_of(cursor) orelse unreachable).value == 1);

    cursor = TRIE.lookupString(cursor, "two");
    try expect((TRIE.is_end_of(cursor) orelse unreachable).value == 2);

    cursor = TRIE.lookupString(0, "twthree");
    try expect((TRIE.is_end_of(cursor) orelse unreachable).value == 3);

    cursor = TRIE.lookupString(0, "eighthree");
    try expect((TRIE.is_end_of(cursor) orelse unreachable).value == 3);
}

const Match = struct {
    start: usize,
    end: usize,
    value: u32,

    fn make(pos: usize, len: usize, value: u32) @This() {
        return @This(){
            .start = pos + 1 - len,
            .end = pos + 1,
            .value = value,
        };
    }
};

fn printMatchRange(line: []const u8, first: *const Match, last: *const Match) void {
    for (0..first.start) |i| {
        std.debug.print("{c}", .{line[i]});
    }
    std.debug.print("\x1b[1;31m", .{});
    if (first.end < last.start) {
        for (first.start..first.end) |i| {
            std.debug.print("{c}", .{line[i]});
        }
        std.debug.print("\x1b[0m", .{});
        for (first.end..last.start) |i| {
            std.debug.print("{c}", .{line[i]});
        }
    } else {
        for (first.start..last.start) |i| {
            std.debug.print("{c}", .{line[i]});
        }
    }
    std.debug.print("\x1b[1;31m", .{});
    for (last.start..last.end) |i| {
        std.debug.print("{c}", .{line[i]});
    }
    std.debug.print("\x1b[0m", .{});
    for (last.end..line.len) |i| {
        std.debug.print("{c}", .{line[i]});
    }
}

fn findLineTotal(line: []const u8) !u32 {
    var first_digit: ?Match = null;
    var last_digit: Match = undefined;

    var walker: usize = 0;
    for (line, 0..) |ch, i| {
        walker = TRIE.lookup(walker, ch);
        if (TRIE.is_end_of(walker)) |value| {
            last_digit = Match.make(i, value.length, value.value);
            if (first_digit == null) {
                first_digit = last_digit;
            }
        }
    }

    const first = first_digit orelse return error.NoMatch;
    const line_total = first.value * 10 + last_digit.value;

    //printMatchRange(line, &first, &last_digit);
    //std.debug.print("  {}\n", .{line_total});
    return line_total;
}

test "line total for two words" {
    try expect(try findLineTotal("one two") == 12);
}

test "line total for one word" {
    try expect(try findLineTotal("one") == 11);
    try expect(try findLineTotal("fivejdmljr") == 55);
}

test "line total with overlapping word ends" {
    try expect(try findLineTotal("eighthree") == 83);
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    _ = alloc;
    var total: u32 = 0;

    var iter = std.mem.tokenizeScalar(u8, data, '\n');
    while (iter.next()) |line| {
        total += try findLineTotal(line);
    }

    // var first_digit: ?Match = null;
    // var last_digit: Match = undefined;
    // var total: u32 = 0;
    // var walker: usize = 0;
    // var line: u32 = 1;
    // while (reader.readByte()) |byte| {
    //     if (byte == '\n') {
    //         const first = first_digit orelse return error.NoMatch;
    //         total += first.value * 10 + last_digit.value;
    //         std.debug.print("  {}\n", .{total});
    //         first_digit = null;
    //         walker = 0;
    //         line += 1;
    //         continue;
    //     }
    //     // if (walker != &TRIE.root and walker == walker.lookup(byte)) {
    //     //     std.debug.print("bug at line {} char {c}\n", .{ line, byte });
    //     // }
    //     walker = TRIE.lookup(walker, byte);
    //     if (TRIE.node(walker).is_end_of) |value| {
    //         if (first_digit == 0) {
    //             first_digit = value.value;
    //         }
    //         last_digit = value.value;
    //     }
    // } else |err| {
    //     switch (err) {
    //         error.EndOfStream => {},
    //         else => return err,
    //     }
    // }
    return std.fmt.bufPrint(result_buf, "{}", .{total});
}
