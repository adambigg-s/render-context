


const CUBE_WIDTH: Float = 10.0;
const WIDTH: usize = 100;
const HEIGHT: usize = 40;
const DISTANCE: Float = 200.0;
const ROTX: Float = 0.05;
const ROTY: Float = 0.05;
const ROTZ: Float = 0.05;
const DEPTHSCALINGX: Float = 175.0;
const DEPTHSCALINGY: Float = 100.0;
const SLEEP_DURATION: u64 = 25;
const DELTA: Float = 1.0;



type Float = f32;
type Int = i32;

fn main() {
    let mut buffer = Buffer::cons(HEIGHT, WIDTH);
    let mut cube = Cube::cons(CUBE_WIDTH, 0.0, 0.0, 0.0, &mut buffer);

    print!("\x1b[2J");
    print!("\x1b[?25l");

    loop {
        cube.render_cube();
        cube.buffer.display();
        cube.rotate();
        print!("\x1b[H");
        print!("\x1b[2J");
        std::thread::sleep(std::time::Duration::from_millis(SLEEP_DURATION));
    }
}

struct Cube<'d> {
    width: Float,
    a: Float,
    b: Float,
    c: Float,
    buffer: &'d mut Buffer,
}

impl<'d> Cube<'d> {
    fn cons(width: Float, a: Float, b: Float, c: Float, buffer: &'d mut Buffer) -> Cube<'d> {
        Cube { width, a, b, c, buffer }
    }

    fn render_cube(&mut self) {
        // let width = self.width.round() as Int;
        // for x in -width..=width {
        //     for y in -width..width {
        //         let (x, y) = (x as Float, y as Float);
        //         self.surface(x, y, -self.width, int_to_chr(1));
        //         self.surface(self.width, y, x, int_to_chr(2));
        //         self.surface(-self.width, y, x, int_to_chr(3));
        //         self.surface(-x, y, self.width, int_to_chr(4));
        //         self.surface(x, -self.width, -y, int_to_chr(5));
        //         self.surface(x, self.width, y, int_to_chr(6));
        //     }
        // }
        let mut x = -self.width;
        while x < self.width {
            let mut y = -self.width;
            while y < self.width {
                self.surface(x, y, -self.width, int_to_chr(1));
                self.surface(self.width, y, x, int_to_chr(2));
                self.surface(-self.width, y, x, int_to_chr(3));
                self.surface(-x, y, self.width, int_to_chr(4));
                self.surface(x, -self.width, -y, int_to_chr(5));
                self.surface(x, self.width, y, int_to_chr(6));
                y += DELTA;
            }
            x += DELTA;
        }
    }

    fn surface(&mut self, x: Float, y: Float, z: Float, chr: char) {
        // let x = rotatex(x, y, z, self);
        // let y = rotatey(x, y, z, self);
        // let z = rotatez(x, y, z, self) + DISTANCE;

        let x = euler_rotate_u(x, y, z, self);
        let y = euler_rotate_v(x, y, z, self);
        let z = euler_rotate_w(x, y, z, self) + DISTANCE;

        let invz = 1.0 / z;
        let multx = DEPTHSCALINGX * invz;
        let multy = DEPTHSCALINGY * invz;
        let xp = (self.buffer.width / 2) as Float + multx * x;
        let yp = (self.buffer.height / 2) as Float - multy * y;

        if z <= 0.0 { return; }

        if let Some(idx) = self.buffer.idx(xp as usize, yp as usize) {
            if invz > self.buffer.zbuffer[idx] {
                self.buffer.zbuffer[idx] = invz;
                self.buffer.visual[idx] = chr;
            }
        }
    }

    fn rotate(&mut self) {
        self.a += ROTX;
        self.b += ROTY;
        self.c += ROTZ;
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
        let visual = vec![int_to_chr(0); width * height];
        let zbuffer = vec![0.0; width * height];
        Buffer { visual, zbuffer, height, width }
    }

    fn to_str(&self) -> String {
        let mut string = String::new();
        for idx in 0..self.visual.len() {
            if idx % WIDTH != 0 {
                string.push(self.visual[idx]);
                continue;
            }
            string.push('\n');
        }
        string
    }
    
    fn display(&mut self) {
        print!("\x1b[H");
        println!("{}", self.to_str());
        self.clear();
    }

    fn clear(&mut self) {
        self.visual.iter_mut().for_each(|ele| *ele = int_to_chr(0));
        self.zbuffer.iter_mut().for_each(|ele| *ele = 0.0);
    }

    fn idx(&self, x: usize, y: usize) -> Option<usize> {
        let newdex = y * self.width + x;
        if newdex < self.width * self.height {
            Some(newdex)
        } else {
            None
        }
    }
}

fn rotatex(i: Float, j: Float, k: Float, cube: &Cube) -> Float {
    let (a, b, c) = (cube.a, cube.b, cube.c);
    i * b.cos() * c.cos() + j * (a.sin() * b.sin() * c.cos() - a.cos() * c.sin())
        + k * (a.cos() * b.sin() * c.cos() + a.sin() * c.sin())
}

fn rotatey(i: Float, j: Float, k: Float, cube: &Cube) -> Float {
    let (a, b, c) = (cube.a, cube.b, cube.c);
    i * b.cos() * c.sin() + j * (a.sin() * b.sin() * c.sin() + a.cos() * c.cos())
        + k * (a.cos() * b.sin() * c.sin() - a.sin() * c.cos())
}

fn rotatez(i: Float, j: Float, k: Float, cube: &Cube) -> Float {
    let (a, b) = (cube.a, cube.b);
    i * -(b.sin()) + j * a.sin() * b.cos() + k * a.cos() * b.cos()
}

fn int_to_chr(int: u8) -> char {
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

fn euler_rotate_u(i: f32, j: f32, k: f32, cube: &Cube) -> f32 {
    let a = cube.a;
    let b = cube.b;
    let c = cube.c;
    i * b.cos() * c.cos()
        + j * (a.sin() * b.sin() * c.cos() - a.cos() * c.sin())
        + k * (a.cos() * b.sin() * c.cos() + a.sin() * c.sin())
}

fn euler_rotate_v(i: f32, j: f32, k: f32, cube: &Cube) -> f32 {
    let a = cube.a;
    let b = cube.b;
    let c = cube.c;
    i * b.cos() * c.sin()
        + j * (a.sin() * b.sin() * c.sin() + a.cos() * c.cos())
        + k * (a.cos() * b.sin() * c.sin() - a.sin() * c.cos())
}

fn euler_rotate_w(i: f32, j: f32, k: f32, cube: &Cube) -> f32 {
    let a = cube.a;
    let b = cube.b;
    i * (-b.sin()) + j * a.sin() * b.cos() + k * a.cos() * b.cos()
}

