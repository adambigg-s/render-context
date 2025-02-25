package vector

import "core:math"

Vec3 :: struct {
	x: f32,
	y: f32,
	z: f32,
}

vec3_build :: proc(x, y, z: f32) -> Vec3 {
	return Vec3{x, y, z}
}

vec3_zeros :: proc() -> Vec3 {
	return Vec3{0, 0, 0}
}

vec3_rot_zyx :: proc(self: ^Vec3, angles: Vec3) {
	vec3_rot_z(self, angles.z)
	vec3_rot_y(self, angles.y)
	vec3_rot_x(self, angles.x)
}

vec3_rot_x :: proc(self: ^Vec3, angle: f32) {
	sin := math.sin(angle)
	cos := math.cos(angle)
	x, y, z := self.x, self.y, self.z

	self.x = x
	self.y = y * cos + -z * sin
	self.z = y * sin + z * cos
}

vec3_rot_y :: proc(self: ^Vec3, angle: f32) {
	sin := math.sin(angle)
	cos := math.cos(angle)
	x, y, z := self.x, self.y, self.z

	self.x = x * cos + z * sin
	self.y = y
	self.z = -x * sin + z * cos
}

vec3_rot_z :: proc(self: ^Vec3, angle: f32) {
	sin := math.sin(angle)
	cos := math.cos(angle)
	x, y, z := self.x, self.y, self.z

	self.x = x * cos + -y * sin
	self.y = x * sin + y * cos
	self.z = z
}

