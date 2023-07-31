const w4 = @import("wasm4.zig");
var game = @import("game.zig").init();

export fn start() void {
    w4.PALETTE.* = .{
        0xfff6d3,
        0xf9a875,
        0xeb6b6f,
        0x7c3f58,
    };
}

var frame_count: u8 = 0;
export fn update() void {
    frame_count += 1;
    if (frame_count % 15 == 0)
        game.update();
    game.draw();
}
