package renderer

import "core:fmt"

import "../buffer"
import "../cube"
import "../math"

Renderer :: struct {
	buffer: ^buffer.Buffer,
	cube:   ^cube.Cube,
	camera: math.Vec3,
}

render_cube :: proc(self: ^Renderer) {
	step := cast(f32)1

	for x := -self.cube.size; x < self.cube.size; x += step {
		for y := -self.cube.size; y < self.cube.size; y += step {
			z := self.cube.size
			calc_surface(self, x, y, z, ']')
			calc_surface(self, x, y, -z, '.')
			calc_surface(self, x, z, y, '%')
			calc_surface(self, x, -z, y, '|')
			calc_surface(self, z, x, y, '*')
			calc_surface(self, -z, x, y, ';')
		}
	}
}

calc_surface :: proc(self: ^Renderer, x: f32, y: f32, z: f32, char: rune) {
	point := math.vec3f_build(x, y, z)

	xscale := cast(f32)175
	yscale := cast(f32)100

	math.vec3_rot_zyx(&point, self.cube.rotation)
	point.x -= self.camera.x
	point.y -= self.camera.y
	point.z -= self.camera.z

	if point.z <= 0.1 {
		return
	}

	screenx := cast(uint)(cast(f32)self.buffer.width / 2 + xscale * point.x / point.z)
	screeny := cast(uint)(cast(f32)self.buffer.height / 2 - yscale * point.y / point.z)

	if buffer.inbounds(self.buffer, screenx, screeny) {
		buffer.set_pixel(self.buffer, screenx, screeny, char, point.z)
	}
}

render_screen :: proc(self: ^Renderer) {
	fmt.print("\x1b[H")

	frame := make([]rune, self.buffer.height * (self.buffer.width + 1))
	defer delete(frame)

	i := 0
	for y: uint = 0; y < self.buffer.height; y += 1 {
		for x: uint = 0; x < self.buffer.width; x += 1 {
			frame[i] = self.buffer.pixels[buffer.index(self.buffer, x, y)]
			i += 1
		}
		frame[i] = '\n'
		i += 1
	}

	fmt.printf("%s", frame)
}

