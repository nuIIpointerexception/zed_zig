const std = @import("std");

/// FIXME(doc): Documentation style fix needed
// TODO(viable): This is colored!
// TODO This isn't colored! (requires colon)
// @Help
// FIXME: Nothing to fix.
// PERF(viable): I'm just joking!
// NOTE(perf): This will be colored too
// HACK: Quick workaround
// WARN: Be careful here
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
