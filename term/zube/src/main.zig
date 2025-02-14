const std = @import("std");
const lib = @import("zube_lib");

const Cube = lib.Cube;
const Vec3f = lib.Vec3f;

const gwidth: usize = 200;
const gheight: usize = 60;

const cubedims: i32 = 27;
const camdistance: f32 = 200;
const xscale: f32 = 175;
const yscale: f32 = 100;
const delta: f32 = 0.9;

const rotx: f32 = 0.01;
const roty: f32 = 0.04;
const rotz: f32 = 0.05;
const fdelay: u64 = 0;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var buffer = try Buffer.init(allocator, gheight, gwidth);
    defer buffer.deinit(allocator);
    var cube = Cube.init(cubedims);
    const rotation = Vec3f.init(rotx, roty, rotz);

    try std.io.getStdOut().writer().print("\x1b[2J\x1b[?25l", .{});
    while (true) {
        buffer.clear();
        buffer.render_cube(&cube);
        try buffer.display();

        cube.rotate(rotation);

        std.Thread.sleep(fdelay * std.time.ns_per_ms);
    }
}

const Buffer = struct {
    height: usize,
    width: usize,
    pixels: []u8,
    depth: []f32,

    pub fn init(allocator: std.mem.Allocator, height: usize, width: usize) !Buffer {
        const buffer = Buffer{
            .height = height,
            .width = width,
            .pixels = try allocator.alloc(u8, width * height),
            .depth = try allocator.alloc(f32, width * height),
        };

        return buffer;
    }

    pub fn deinit(self: *Buffer, allocator: std.mem.Allocator) void {
        allocator.free(self.pixels);
        allocator.free(self.depth);
    }

    pub fn clear(self: *Buffer) void {
        @memset(self.pixels, ' ');
        @memset(self.depth, 1e+12);
    }

    pub fn render_cube(self: *Buffer, cube: *Cube) void {
        const step = delta;

        var cubeu = -cube.dims;
        while (cubeu <= cube.dims) : (cubeu += step) {
            var cubev = -cube.dims;
            while (cubev <= cube.dims) : (cubev += step) {
                self.calc_surface(cube, '^', cubeu, cubev, cube.dims);
                self.calc_surface(cube, '.', cubeu, cubev, -cube.dims);
                self.calc_surface(cube, '@', cube.dims, cubeu, cubev);
                self.calc_surface(cube, '*', -cube.dims, cubeu, cubev);
                self.calc_surface(cube, ',', cubeu, cube.dims, cubev);
                self.calc_surface(cube, '|', cubeu, -cube.dims, cubev);
            }
        }
    }

    pub fn display(self: *Buffer) !void {
        const stdout_file = std.io.getStdOut().writer();
        var bw = std.io.bufferedWriter(stdout_file);
        const stdout = bw.writer();

        try stdout.print("\x1b[H", .{});
        for (0..self.height) |y| {
            for (0..self.width) |x| {
                const idx = self.index(x, y);
                try stdout.print("{c}", .{self.pixels[idx]});
            }
            try stdout.print("\n", .{});
        }
        try bw.flush();
    }

    fn calc_surface(self: *Buffer, cube: *Cube, char: u8, x: f32, y: f32, z: f32) void {
        var point = Vec3f.init(x, y, z);
        point.rot_zyx(cube.rotation);
        point.z += camdistance;

        if (point.z <= 0.1) {
            return;
        }
        const screenx: usize = @intFromFloat(self.halfwidth() + xscale * point.x / point.z);
        const screeny: usize = @intFromFloat(self.halfheight() + yscale * point.y / point.z);

        if (self.inbounds(screenx, screeny)) {
            self.set(screenx, screeny, point.z, char);
        }
    }

    pub fn set(self: *Buffer, x: usize, y: usize, depth: f32, char: u8) void {
        const idx = self.index(x, y);
        if (self.depth[idx] > depth) {
            self.depth[idx] = depth;
            self.pixels[idx] = char;
        }
    }

    fn halfheight(self: *Buffer) f32 {
        const height: f32 = @floatFromInt(self.height);
        return height / 2;
    }

    fn halfwidth(self: *Buffer) f32 {
        const width: f32 = @floatFromInt(self.width);
        return width / 2;
    }

    fn inbounds(self: *Buffer, x: usize, y: usize) bool {
        return x < self.width and y < self.height;
    }

    pub fn index(self: *Buffer, x: usize, y: usize) usize {
        return y * self.width + x;
    }
};
