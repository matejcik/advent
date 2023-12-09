const std = @import("std");
const print = std.debug.print;

const Node = struct {
    children: [2]*const Node,
    name: []const u8,
};

const Graph = struct {
    map: std.StringHashMap(Node),
    directions: []const u8,

    pub fn build(data: []const u8, alloc: std.mem.Allocator) !Graph {
        var lines_iter = std.mem.tokenizeScalar(u8, data, '\n');
        const directions = lines_iter.next().?;

        var graph = Graph{
            .map = std.StringHashMap(Node).init(alloc),
            .directions = directions,
        };

        const lines = std.mem.count(u8, data, "\n");
        try graph.map.ensureTotalCapacity(@intCast(lines));

        while (lines_iter.next()) |line| {
            // XKM = (FRH, RLM)
            const name = line[0..3];
            const left = line[7..10];
            const right = line[12..15];

            var v = graph.map.getOrPutAssumeCapacity(name);
            v.value_ptr.name = name;
            v.value_ptr.children[0] = graph.map.getOrPutAssumeCapacity(left).value_ptr;
            v.value_ptr.children[1] = graph.map.getOrPutAssumeCapacity(right).value_ptr;
        }

        return graph;
    }

    pub fn deinit(self: *Graph) void {
        self.map.deinit();
    }

    fn walkPart1(self: Graph) usize {
        var node: *const Node = self.map.getPtr("AAA").?;
        const end = self.map.getPtr("ZZZ").?;
        var steps: usize = 0;
        outer: while (true) {
            for (self.directions) |d| {
                steps += 1;
                const idx: usize = if (d == 'L') 0 else 1;
                node = node.children[idx];
                if (node == end) {
                    break :outer;
                }
            }
        }
        return steps;
    }

    fn ghostWalk(self: *Graph, alloc: std.mem.Allocator) !usize {
        var ghosts = try std.ArrayList(*const Node).initCapacity(alloc, self.map.capacity());
        defer ghosts.deinit();

        // get all starting nodes
        var values_iter = self.map.valueIterator();
        while (values_iter.next()) |val| {
            if (val.name[2] == 'A') {
                try ghosts.append(val);
            }
        }

        var steps: usize = 0;
        outer: while (true) {
            inner: for (self.directions) |d| {
                steps += 1;
                const idx: usize = if (d == 'L') 0 else 1;
                for (0..ghosts.items.len) |i| {
                    ghosts.items[i] = ghosts.items[i].children[idx];
                }
                // check if all ghosts are at the end
                for (ghosts.items) |ghost| {
                    if (ghost.name[2] != 'Z') {
                        continue :inner;
                    }
                }
                break :outer;
            }
        }
        return steps;
    }
};

test "testdata for part 1" {
    const TEST_DATA =
        \\RL
        \\
        \\AAA = (BBB, CCC)
        \\BBB = (DDD, EEE)
        \\CCC = (ZZZ, GGG)
        \\DDD = (DDD, DDD)
        \\EEE = (EEE, EEE)
        \\GGG = (GGG, GGG)
        \\ZZZ = (ZZZ, ZZZ)
    ;
    var graph = try Graph.build(TEST_DATA, std.testing.allocator);
    defer graph.deinit();

    const steps = graph.walkPart1();
    try std.testing.expectEqual(steps, 2);
}

test "testdata for part 2" {
    const TEST_DATA =
        \\LR
        \\
        \\11A = (11B, XXX)
        \\11B = (XXX, 11Z)
        \\11Z = (11B, XXX)
        \\22A = (22B, XXX)
        \\22B = (22C, 22C)
        \\22C = (22Z, 22Z)
        \\22Z = (22B, 22B)
        \\XXX = (XXX, XXX)
    ;
    var graph = try Graph.build(TEST_DATA, std.testing.allocator);
    defer graph.deinit();

    const steps = try graph.ghostWalk(std.testing.allocator);
    try std.testing.expectEqual(steps, 6);
}

pub fn part1(data: []const u8, alloc: std.mem.Allocator) !void {
    var graph = try Graph.build(data, alloc);
    defer graph.deinit();

    print("Day 8 part 1: {}\n", .{graph.walkPart1()});
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator) !void {
    var graph = try Graph.build(data, alloc);
    defer graph.deinit();

    print("Day 8 part 2: {}\n", .{try graph.ghostWalk(alloc)});
}
