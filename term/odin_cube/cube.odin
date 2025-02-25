package main

import "core:fmt"
import "core:time"

import buf "buffer"
import cu "cube"
import "math"
import ren "renderer"

main :: proc() {
	buffer := buf.build(60, 200)
	defer free(&buffer)

	cube := cu.build(27)

	renderer := ren.Renderer {
		buffer = &buffer,
		cube   = &cube,
		camera = math.vec3f_build(0, 0, -200),
	}

	for {
		buf.clear(&buffer)
		
		ren.render_cube(&renderer)
		ren.render_screen(&renderer)

		cu.cube_rotate(&cube, math.vec3f_build(0.01, 0.03, 0.005))

		time.sleep(7 * time.Millisecond)
	}
}

