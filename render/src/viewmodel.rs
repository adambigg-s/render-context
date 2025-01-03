


use crate::Color;
use crate::buffer::{color_tag, Buffer};
use crate::{Float, Int};
use crate::math::{tau, Vec3};



pub struct ViewModel
{
    pub position: Vec3<Float>,
    pub rotation: Float,
    pub tilt: Int,
}

impl ViewModel
{
    pub fn cons(position: Vec3<Float>) -> ViewModel
    {
        ViewModel { position, rotation: Float::default(), tilt: Int::default() }
    }

    pub fn move_forward(&mut self, direction: Float, speed: Float)
    {
        self.position.x += direction * speed * self.rotation.cos();
        self.position.y += direction * speed * self.rotation.sin();
    }

    pub fn move_lateral(&mut self, direction: Float, speed: Float)
    {
        self.position.x -= direction * speed * self.rotation.sin();
        self.position.y += direction * speed * self.rotation.cos();
    }
    
    pub fn rotate(&mut self, delta: Float)
    {
        self.rotation += delta;
        if self.rotation < 0.0 {
            self.rotation += tau();
        }
        else if self.rotation > tau() {
            self.rotation -= tau();
        }
    }
    
    pub fn tilt(&mut self, delta: Int)
    {
        self.tilt += delta;
        self.tilt = self.tilt.clamp(-70, 70);
    }
}

pub struct Wall
{
    pub edge1: Vec3<Float>,
    pub edge2: Vec3<Float>,
    pub height: Float,
}

impl Wall
{
    pub fn cons(edge1: Vec3<Float>, edge2: Vec3<Float>, height: Float) -> Wall
    {
        Wall { edge1, edge2, height }
    }
}

pub fn draw3d_point(viewmodel: &ViewModel, buffer: &mut Buffer, point: &Vec3<Float>)
{
    let view: Vec3<Float> = *point - viewmodel.position;

    let mut world: Vec3<Float> = view.rotation_z(viewmodel.rotation);
    world.z += (viewmodel.tilt as Float) * world.x / 32.0;
    
    let scale_factor = 100.0 / world.x;
    let screen_x = world.y * scale_factor + buffer.halfwidth() as Float;
    let screen_y = world.z * scale_factor + buffer.halfheight() as Float;

    let (sx, sy) = (screen_x as usize, screen_y as usize);

    draw_cluster(sx, sy, 2, color_tag(3), buffer);
}

pub fn draw3d_wall(viewmodel: &ViewModel, buffer: &mut Buffer, wall: &Wall)
{
    let p1 = wall.edge1;
    let p2 = wall.edge2;
    let p3 = wall.edge1 + Vec3::cons(0.0, 0.0, wall.height);
    let p4 = wall.edge2 + Vec3::cons(0.0, 0.0, wall.height);
    
    let points = [p1, p2, p3, p4];
    for point in &points {
        draw3d_point(viewmodel, buffer, point);
    }
}


pub fn draw_point(point: &Vec3<Float>, buffer: &mut Buffer)
{
    let sx: usize = (point.x + buffer.halfwidth() as Float) as usize;
    let sy: usize = (point.y + buffer.halfheight() as Float) as usize;

    draw_cluster(sx, sy, 3, color_tag(2), buffer);
}

pub fn draw_view(viewmodel: &ViewModel, buffer: &mut Buffer)
{
    let sx: usize = (viewmodel.position.x + buffer.halfwidth() as Float) as usize;
    let sy: usize = (viewmodel.position.y + buffer.halfheight() as Float) as usize;
    draw_cluster(sx, sy, 3, color_tag(1), buffer);

    let (sin, cos): (Float, Float) = viewmodel.rotation.sin_cos();
    (0..10).for_each(|len| {
        let nx: usize = (sx as Float + len as Float * cos) as usize;
        let ny: usize = (sy as Float + len as Float * sin) as usize;
        if buffer.inbounds(nx, ny) {
            buffer.place_pixel(nx, ny, color_tag(2));
        }
    });
}

pub fn draw_cluster(x: usize, y: usize, size: isize, color: Color, buffer: &mut Buffer)
{
    let range: isize = size / 2;
    (-range..=range).for_each(|dy| {
        (-range..=range).for_each(|dx| {
            let (sx, sy): (usize, usize) = idx_usize_isize(x, y, dx, dy);
            if buffer.inbounds(sx, sy) {
                buffer.place_pixel(sx, sy, color);
            }
        })
    })
}

pub fn idx_usize_isize(x: usize, y: usize, dx: isize, dy: isize) -> (usize, usize)
{
    ((x as isize + dx) as usize, (y as isize + dy) as usize)
}



#[cfg(test)]
mod test
{
    #[test]
    fn float_cast_usize()
    {
        assert!(-13.04 as usize == 0);
    }

    #[test]
    fn negint_cast_usize()
    {
        assert!(34_i32 as usize == 34);
        assert!(-34_i32 as usize > 34);
    }
}
