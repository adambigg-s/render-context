package buffer

Buffer :: struct {
	height: uint,
	width:  uint,
	pixels: [dynamic]rune,
	depth:  [dynamic]f32,
}

build :: proc(height: uint, width: uint) -> Buffer {
	pixels := make([dynamic]rune, height * width)
	depth := make([dynamic]f32, height * width)
	buffer := Buffer{height, width, pixels, depth}
	clear(&buffer)

	return buffer
}

free :: proc(self: ^Buffer) {
	delete(self.pixels)
	delete(self.depth)
	self^ = {}
}

clear :: proc(self: ^Buffer) {
	for &pix in self.pixels {
		pix = ' '
	}
	for &depth in self.depth {
		depth = 1e+12
	}
}

set_pixel :: proc(self: ^Buffer, x: uint, y: uint, data: rune, depth: f32) {
	index := index(self, x, y)
	if inbounds(self, x, y) {
		if self.depth[index] < depth {
			return
		}
		self.pixels[index] = data
		self.depth[index] = depth
	}
}

index :: proc(self: ^Buffer, x: uint, y: uint) -> uint {
	return y * self.width + x
}

inbounds :: proc(self: ^Buffer, x: uint, y: uint) -> bool {
	return y < self.height && x < self.width
}

