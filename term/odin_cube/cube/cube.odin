package cube

import m "../math"

Cube :: struct {
	position: m.Vec3,
	rotation: m.Vec3,
	size:     f32,
}

cube_build :: proc(size: f32) -> Cube {
	return Cube{position = m.vec3_zeros(), rotation = m.vec3_zeros(), size = size}
}

cube_rotate :: proc(self: ^Cube, angles: m.Vec3) {
	self.rotation.x += angles.x
	self.rotation.y += angles.y
	self.rotation.z += angles.z
}

