const std = @import("std");

pub fn part1(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    _ = alloc;

    std.debug.assert(data[data.len - 1] == '\n');

    var total: u64 = 0;
    var hash: u8 = 0;
    for (data[0 .. data.len - 1]) |c| {
        if (c == ',') {
            total += @intCast(hash);
            hash = 0;
            continue;
        }
        hash +%= c;
        hash *%= 17;
    }
    total += @intCast(hash);
    return std.fmt.bufPrint(result_buf, "{}", .{total});
}

test "part1" {
    const in = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";
    var result_buf = [_]u8{0} ** 128;
    const result = try part1(in, std.testing.allocator, &result_buf);
    try std.testing.expect(std.mem.eql(u8, result, "1320"));
}

fn calcHash(s: []const u8) u8 {
    var hash: u8 = 0;
    for (s) |c| {
        hash +%= c;
        hash *%= 17;
    }
    return hash;
}

const MapItem = struct {
    hash: u8,
    key: []const u8,
    value: u8,
};

const HMap = struct {
    data: [256]std.ArrayList(MapItem),

    pub fn init(alloc: std.mem.Allocator) HMap {
        var self = HMap{ .data = undefined };
        for (0..self.data.len) |i| {
            self.data[i] = std.ArrayList(MapItem).initCapacity(alloc, 256) catch unreachable;
        }
        return self;
    }

    pub fn deinit(self: *HMap) void {
        for (self.data) |list| {
            list.deinit();
        }
    }

    pub fn remove(self: *HMap, item: MapItem) void {
        var list = &self.data[item.hash];
        for (list.items, 0..) |v, i| {
            if (std.mem.eql(u8, v.key, item.key)) {
                _ = list.orderedRemove(i);
                return;
            }
        }
    }

    pub fn update(self: *HMap, item: MapItem) void {
        var list = &self.data[item.hash];
        for (list.items, 0..) |v, i| {
            if (std.mem.eql(u8, v.key, item.key)) {
                list.items[i] = item;
                return;
            }
        }
        list.append(item) catch unreachable;
    }

    pub fn process(self: *HMap, s: []const u8) void {
        if (s[s.len - 1] == '-') {
            const key = s[0 .. s.len - 1];
            const item = MapItem{
                .hash = calcHash(key),
                .key = key,
                .value = 0,
            };
            self.remove(item);
            return;
        } else if (s[s.len - 2] == '=') {
            const key = s[0 .. s.len - 2];
            const item = MapItem{
                .hash = calcHash(key),
                .key = key,
                .value = s[s.len - 1],
            };
            self.update(item);
            return;
        } else {
            unreachable;
        }
    }

    pub fn focusingPower(self: HMap) u64 {
        var total: u64 = 0;
        for (self.data, 1..) |list, i| {
            for (list.items, 1..) |item, j| {
                const item_power = i * j * (item.value - '0');
                // std.debug.print("in box {}, item {} ({s}) is lens {c} => power {}\n", .{ i, j, item.key, item.value, item_power });
                total += item_power;
            }
        }
        return total;
    }
};

pub fn part2(data: []const u8, alloc: std.mem.Allocator, result_buf: []u8) anyerror![]const u8 {
    var map = HMap.init(alloc);
    defer map.deinit();

    std.debug.assert(data[data.len - 1] == '\n');
    var iter = std.mem.splitScalar(u8, data[0 .. data.len - 1], ',');
    while (iter.next()) |s| {
        map.process(s);
    }

    return std.fmt.bufPrint(result_buf, "{}", .{map.focusingPower()});
}

test "part2" {
    const in = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";
    var result_buf = [_]u8{0} ** 128;
    const result = try part2(in, std.testing.allocator, &result_buf);
    try std.testing.expect(std.mem.eql(u8, result, "145"));
}
