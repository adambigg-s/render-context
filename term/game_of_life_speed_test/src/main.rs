


use std::mem::swap;

use minifb::{Key, Window, WindowOptions};



type Color = u32;

fn main() {
    let mut buffer = Buffer::cons(1000, 1000);
    let mut world = World::cons(buffer.height, buffer.width);
    let mut window = Window::new(
        "conway's game of life concurrent",
        buffer.width,
        buffer.height,
        WindowOptions {
            ..Default::default()
        },
    ).unwrap();
    window.set_target_fps(250);

    world.randomize();

    while !window.is_key_down(Key::Escape) {
        buffer.clear();
        world.update();
        let mut rend = Renderer { world: &world, buffer: &mut buffer };
        rend.render();

        window.update_with_buffer(&buffer.pixels, buffer.width, buffer.height).unwrap();
    }
}

pub struct Renderer<'d> {
    world: &'d World,
    buffer: &'d mut Buffer,
}

impl Renderer<'_> {
    pub fn render(&mut self) {
        for y in 0..self.world.height {
            for x in 0..self.world.width {
                let idx = self.world.idx(x, y);
                if self.world.worker[idx].is_dead() { continue; }
                self.buffer.place_pixel(x, y, 0xffdddddd);
            }
        }
    }

    pub fn get_input(&mut self) {
    }
}

pub struct Buffer {
    pixels: Vec<Color>,
    height: usize,
    width: usize,
}

impl Buffer {
    fn cons(height: usize, width: usize) -> Buffer {
        Buffer { pixels: vec![0; width * height], height, width, }
    }

    fn clear(&mut self) {
        self.pixels.fill(0);
    }

    fn place_pixel(&mut self, x: usize, y: usize, data: Color) {
        if let Some(idx) = self.inboundsdex(x, y) {
            self.pixels[idx] = data;
        }
    }

    fn inboundsdex(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            return Some(y * self.width + x);
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Cell {
    fn is_dead(&self) -> bool {
        *self == Cell::Dead
    }

    fn is_alive(&self) -> bool {
        *self == Cell::Alive
    }

    fn kill(&mut self) {
        *self = Cell::Dead;
    }

    fn revive(&mut self) {
        *self = Cell::Alive;
    }
}

struct World {
    height: usize,
    width: usize,
    worker: Vec<Cell>,
    next: Vec<Cell>,
}

impl World {
    pub fn cons(height: usize, width: usize) -> World {
        World {
            height, width,
            worker: vec![Cell::Dead; width * height],
            next: vec![Cell::Dead; width * height],
        }
    }

    pub fn randomize(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {

                if rand::random::<bool>() {
                    let idx = self.idx(x, y);
                    self.worker[idx].revive();
                }
            }
        }
    }

    pub fn update(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_live_neighbors(x, y);

                let idx = self.idx(x, y);
                let curr = &mut self.worker[idx];
                let next = &mut self.next[idx];

                if curr.is_alive() {
                    if !(2..=3).contains(&neighbors) {
                        next.kill();
                    }
                    else {
                        next.revive();
                    }
                }
                else if neighbors == 3 {
                    next.revive();
                }
                else {
                    next.kill();
                }
            }
        }

        self.flip();
    }

    fn count_live_neighbors(&self, x: usize, y: usize) -> usize {
        let mut neighbors = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }

                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;

                if self.inbounds(nx, ny) && self.worker[self.idx(nx, ny)].is_alive() {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }

    fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn flip(&mut self) {
        swap(&mut self.worker, &mut self.next);
    }
}
