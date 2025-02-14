const std = @import("std");

pub const Vec3f = struct {
    x: f32,
    y: f32,
    z: f32,

    pub fn init(x: f32, y: f32, z: f32) Vec3f {
        return Vec3f{ .x = x, .y = y, .z = z };
    }

    pub fn zeros() Vec3f {
        return Vec3f{ .x = 0, .y = 0, .z = 0 };
    }

    pub fn rot_zyx(self: *Vec3f, angles: Vec3f) void {
        self.rot_z(angles.z);
        self.rot_y(angles.y);
        self.rot_x(angles.x);
    }

    pub fn rot_x(self: *Vec3f, angle: f32) void {
        const sin = std.math.sin(angle);
        const cos = std.math.cos(angle);
        const x = self.x;
        const y = self.y;
        const z = self.z;

        self.x = x;
        self.y = y * cos + -z * sin;
        self.z = y * sin + z * cos;
    }

    pub fn rot_y(self: *Vec3f, angle: f32) void {
        const sin = std.math.sin(angle);
        const cos = std.math.cos(angle);
        const x = self.x;
        const y = self.y;
        const z = self.z;

        self.x = x * cos + z * sin;
        self.y = y;
        self.z = -x * sin + z * cos;
    }

    pub fn rot_z(self: *Vec3f, angle: f32) void {
        const sin = std.math.sin(angle);
        const cos = std.math.cos(angle);
        const x = self.x;
        const y = self.y;
        const z = self.z;

        self.x = x * cos + -y * sin;
        self.y = x * sin + y * cos;
        self.z = z;
    }
};

pub const Cube = struct {
    dims: f32,
    rotation: Vec3f,

    pub fn init(dims: f32) Cube {
        return Cube{ .dims = dims, .rotation = Vec3f.zeros() };
    }

    pub fn rotate(self: *Cube, rotation: Vec3f) void {
        self.rotation.x += rotation.x;
        self.rotation.y += rotation.y;
        self.rotation.z += rotation.z;
    }
};
