const std = @import("std");

// TODO(viable): This is colored!
// TODO This isn't colored!
// FIXME(viable): Nothing to fix.
// BUG(viable): I'm just joking!
pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("Hello, world!\n", .{});
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit();

    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
