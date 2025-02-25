package cube

import "../math"

Cube :: struct {
	position: math.Vec3,
	rotation: math.Vec3,
	size:     f32,
}

build :: proc(size: f32) -> Cube {
	return Cube{position = math.vec3_zeros(), rotation = math.vec3_zeros(), size = size}
}

cube_rotate :: proc(self: ^Cube, angles: math.Vec3) {
	self.rotation.x += angles.x
	self.rotation.y += angles.y
	self.rotation.z += angles.z
}

