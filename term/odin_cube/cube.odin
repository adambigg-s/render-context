package main

import "core:fmt"
import "core:time"

import buf "buffer"
import cu "cube"
import m "math"
import ren "renderer"

main :: proc() {
	buffer := buf.buffer_build(60, 200)
	defer free(&buffer)

	cube := cu.cube_build(30)

	renderer := ren.Renderer {
		buffer = &buffer,
		cube   = &cube,
		camera = m.vec3_build(0, 0, -200),
	}

	for {
		buf.clear(&buffer)

		ren.render_cube(&renderer)
		ren.render_screen(&renderer)

		cu.cube_rotate(&cube, m.vec3_build(0.01, 0.03, 0.005))

		time.sleep(5 * time.Millisecond)
	}
}

