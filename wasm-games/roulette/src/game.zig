const w4 = @import("wasm4.zig");
const std = @import("std");

const SCREEN_SIZE = 160;
const SCREEN_HALF = SCREEN_SIZE / 2;

const Self = @This();

curr: u4,
stop: bool,

pub fn init() Self {
    return .{ .curr = 0, .stop = false };
}

pub fn draw(self: *Self) void {
    w4.DRAW_COLORS.* = 3;
    w4.text("balance: 69", 5, 5);
    w4.text("wins: 420", 5, 15);

    const gamepad = w4.GAMEPAD1.*;
    self.stop = gamepad & w4.BUTTON_1 != 0;

    var buffer: [1]u8 = undefined;
    for (range(5)) |_, i| {
        if ((i + self.curr) % 2 == 0) {
            w4.DRAW_COLORS.* = 3;
        } else w4.DRAW_COLORS.* = 2;
        var pad: i32 = 0;
        if (i != 0) pad = 40 * @intCast(i32, i);
        w4.rect(pad - 20, SCREEN_HALF - 20, 40, 40);

        w4.DRAW_COLORS.* = 4;
        w4.text(std.fmt.bufPrint(&buffer, "{}", .{self.curr + i}) catch "failed", pad - 5, SCREEN_HALF - 5);
    }
}

pub fn update(self: *Self) void {
    if (self.stop) return;

    if (self.curr == 6) {
        self.curr = 0;
    } else self.curr += 1;
}

fn range(len: usize) []const void {
    return @as([*]void, undefined)[0..len];
}
