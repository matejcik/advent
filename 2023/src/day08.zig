const std = @import("std");
const print = std.debug.print;

const Node = struct {
    children: [2]*Node,
    name: []const u8,
    zloop_steps: usize,
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

    fn zloopInit(self: *Graph) void {
        var iter = self.map.valueIterator();
        while (iter.next()) |val| {
            val.zloop_steps = 0;
        }
    }

    const Zloop = struct {
        cursor: usize,
        loop: usize,

        fn merge(self: Zloop, other: Zloop) Zloop {
            std.testing.expectEqual(self.loop, self.cursor) catch unreachable;
            std.testing.expectEqual(other.loop, other.cursor) catch unreachable;

            const lcm = self.loop * (other.loop / std.math.gcd(self.loop, other.loop));
            return Zloop{
                .cursor = lcm,
                .loop = lcm,
            };

            // var bigger = if (self.cursor < other.cursor) other else self;
            // var smaller = if (self.cursor < other.cursor) self else other;
            // while (bigger.cursor != smaller.cursor) {
            //     smaller.cursor += smaller.loop;
            //     if (smaller.cursor > bigger.cursor) {
            //         const tmp = bigger;
            //         bigger = smaller;
            //         smaller = tmp;
            //     }
            // }
            // const loop_lcm = bigger.loop * (smaller.loop / std.math.gcd(bigger.loop, smaller.loop));
            // return Zloop{
            //     .cursor = smaller.cursor,
            //     .loop = loop_lcm,
            // };
        }
    };

    fn zloopFrom(self: *Graph, start: *Node) Zloop {
        var steps: usize = 0;
        var node = start;
        var first_z: usize = 0;
        self.zloopInit();
        outer: while (true) {
            for (self.directions) |d| {
                steps += 1;
                const idx: usize = if (d == 'L') 0 else 1;
                node = node.children[idx];
                if (node.zloop_steps != 0 and node.zloop_steps % self.directions.len == steps % self.directions.len) {
                    break :outer;
                }
                node.zloop_steps = steps;
                if (node.name[2] == 'Z' and first_z == 0) {
                    first_z = steps;
                }
            }
        }
        return Zloop{
            .cursor = first_z,
            .loop = steps - first_z,
        };
    }

    fn ghostWalk(self: *Graph, alloc: std.mem.Allocator) !usize {
        var ghosts = try std.ArrayList(Zloop).initCapacity(alloc, self.map.capacity());
        defer ghosts.deinit();

        // get all starting nodes
        var values_iter = self.map.valueIterator();
        while (values_iter.next()) |val| {
            if (val.name[2] == 'A') {
                ghosts.appendAssumeCapacity(self.zloopFrom(val));
            }
        }
        var result = ghosts.items[0];
        for (ghosts.items[1..]) |g| {
            result = result.merge(g);
        }
        return result.cursor;
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

pub fn part1(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    var graph = try Graph.build(data, alloc);
    defer graph.deinit();

    return std.fmt.bufPrint(result_buf, "{}", .{graph.walkPart1()});
}

pub fn part2(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    var graph = try Graph.build(data, alloc);
    defer graph.deinit();

    return std.fmt.bufPrint(result_buf, "{}", .{try graph.ghostWalk(alloc)});
}
