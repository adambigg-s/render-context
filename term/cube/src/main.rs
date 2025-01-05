


const WIDTH: usize = 200;
const HEIGHT: usize = 60;
const CUBE_WIDTH: Float = 27.0;
const DEPTHSCALINGX: Float = 175.0;
const DEPTHSCALINGY: Float = 100.0;
const DELTA: Float = 0.9;
const FRAME_DELAY: u64 = 10;
const DISTANCE: Float = 200.0;
const ROTX: Float = 0.01;
const ROTY: Float = 0.04;
const ROTZ: Float = 0.005;



type Float = f32;

struct Tri {
    x: Float,
    y: Float,
    z: Float,
}

impl Tri {
    fn cons(x: Float, y: Float, z: Float) -> Tri {
        Tri { x, y, z }
    }
}

struct Cube {
    sidelen: Float,
    a: Float,
    b: Float,
    c: Float,
    rotspeed: Tri,
}

impl Cube {
    fn cons(sidelen: Float, rotspeed: Tri) -> Cube {
        Cube{ sidelen, a: 0.0, b: 0.0, c: 0.0, rotspeed }
    }

    fn euler_rotate_u(&self, i: Float, j: Float, k: Float) -> Float {
        let (a, b, c) = (self.a, self.b, self.c);
        i * b.cos() * c.cos()
            + j * (a.sin() * b.sin() * c.cos()
                - a.cos() * c.sin())
            + k * (a.cos() * b.sin() * c.cos()
                + a.sin() * c.sin())
    }

    fn euler_rotate_v(&self, i: Float, j: Float, k: Float) -> Float {
        let (a, b, c) = (self.a, self.b, self.c);
        i * b.cos() * c.sin()
            + j * (a.sin() * b.sin() * c.sin()
                + a.cos() * c.cos())
            + k * (a.cos() * b.sin() * c.sin()
                - a.sin() * c.cos())
    }

    fn euler_rotate_w(&self, i: Float, j: Float, k: Float) -> Float {
        let (a, b) = (self.a, self.b);
        i * -(b.sin())
            + j * a.sin() * b.cos()
            + k * a.cos() * b.cos()
    }

    fn rotate(&mut self) {
        self.a += self.rotspeed.x;
        self.b += self.rotspeed.y;
        self.c += self.rotspeed.z;
    }
}

struct Buffer {
    visual: Vec<char>,
    zbuffer: Vec<Float>,
    height: usize,
    width: usize,
}

impl Buffer {
    fn cons(height: usize, width: usize) -> Buffer {
        let visual = vec![intchr(0); width * height];
        let zbuffer = vec![0.0; width * height];
        Buffer { visual, zbuffer, height, width }
    }

    fn clear(&mut self) {
        self.visual.fill(intchr(0));
        self.zbuffer.fill(0.0);
    }

    fn to_str(&self) -> String {
        let mut string = String::new();
        self.visual.iter().enumerate().for_each(|(idx, ele)| {
            string.push_str("\x1b[2m");
            if idx % self.width != 0 {
                string.push(*ele);
            }
            else {
                string.push('\n');
            }
        });
        string
    }

    fn display(&mut self) {
        // ansi escape to move cursor-line to beginning
        print!("\x1b[H");
        println!("{}", self.to_str());
        self.clear();
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn inbounds(&self, idx: usize) -> bool {
        idx < self.width * self.height
    }
}

struct RenderContext<'d> {
    cube: &'d Cube,
    buffer: &'d mut Buffer,
}

impl<'d> RenderContext<'d> {
    fn cons(cube: &'d Cube, buffer: &'d mut Buffer) -> RenderContext<'d> {
        RenderContext { cube, buffer }
    }
    
    fn render_cube(&mut self) {
        let mut cubex = -self.cube.sidelen;
        while cubex < self.cube.sidelen {
            let mut cubey = -self.cube.sidelen;
            while cubey < self.cube.sidelen {
                self.surface(cubex, cubey, -self.cube.sidelen, intchr(1));
                self.surface(self.cube.sidelen, cubey, cubex, intchr(2));
                self.surface(-self.cube.sidelen, cubey, cubex, intchr(3));
                self.surface(-cubex, cubey, self.cube.sidelen, intchr(4));
                self.surface(cubex, -self.cube.sidelen, -cubey, intchr(5));
                self.surface(cubex, self.cube.sidelen, cubey, intchr(6));
                cubey += DELTA;
            }
            cubex += DELTA;
        }
    }

    fn surface(&mut self, cubex: Float, cubey: Float, cubez: Float, chr: char) {
        let x = self.cube.euler_rotate_u(cubex, cubey, cubez);
        let y = self.cube.euler_rotate_v(cubex, cubey, cubez);
        let z = self.cube.euler_rotate_w(cubex, cubey, cubez) + DISTANCE;

        let invz = 1.0 / z;
        let multx = DEPTHSCALINGX * invz;
        let multy = DEPTHSCALINGY * invz;
        let xp = ((self.buffer.width / 2) as Float + multx * x) as usize;
        let yp = ((self.buffer.height / 2) as Float - multy * y) as usize;

        let idx = self.buffer.idx(xp, yp);
        if self.buffer.inbounds(idx) && invz > self.buffer.zbuffer[idx] {
            self.buffer.zbuffer[idx] = invz;
            self.buffer.visual[idx] = chr;
        }
    }
}

#[inline]
pub fn dump<Any>(_thing: Any) {}

fn intchr(int: u8) -> char {
    match int {
        0 => ' ',
        1 => '$',
        2 => ',',
        3 => '!',
        4 => '~',
        5 => '>',
        6 => '|',
        _ => ' ',
    }
}

fn main() {
    let mut buffer = Buffer::cons(HEIGHT, WIDTH);
    let mut cube = Cube::cons(CUBE_WIDTH, Tri::cons(ROTX, ROTY, ROTZ));

    // ansi escape to clear terminal
    print!("\x1b[2J");
    // ansi escape to make cursor-line invisible for program
    print!("\x1b[?25l");

    loop {
        let mut renderer = RenderContext::cons(&cube, &mut buffer);
        renderer.render_cube();
        renderer.buffer.display();
        dump(renderer);
        cube.rotate();
        std::thread::sleep(std::time::Duration::from_millis(FRAME_DELAY));
    }
}

