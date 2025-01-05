use crate::buffer::{color_tag, Buffer};
use crate::math::Vec3;
use crate::utility::Wall;
use crate::viewmodel::ViewModel;
use crate::Float;

pub struct Renderer<'d> {
    pub viewmodel: &'d ViewModel,
    pub buffer: &'d mut Buffer,
}

impl<'d> Renderer<'d> {
    pub fn cons(viewmodel: &'d ViewModel, buffer: &'d mut Buffer) -> Renderer<'d> {
        Renderer { viewmodel, buffer }
    }

    pub fn draw3d_point(&mut self, point: &Vec3<Float>) {
        let (screen_x, screen_y) = self.world_to_screen(point);
        self.buffer.draw_cluster(screen_x, screen_y, 3, color_tag(4));
    }

    pub fn world_to_screen(&mut self, point: &Vec3<Float>) -> (usize, usize) {
        let view: Vec3<Float> = *point - self.viewmodel.position;

        let mut world: Vec3<Float> = view.rotation_z(-self.viewmodel.rotation);
        world.z += (self.viewmodel.tilt as Float) * world.x / 32.0;

        let scale_factor = 200.0 / world.x;
        let screen_x = (-world.y * scale_factor + self.buffer.halfwidth() as Float) as usize;
        let screen_y = (world.z * scale_factor + self.buffer.halfheight() as Float) as usize;

        (screen_x, screen_y)
    }

    pub fn draw3d_wall(&mut self, wall: &Wall) {
        let p1 = wall.edge1;
        let p2 = wall.edge2;
        let p3 = wall.edge1 + Vec3::cons(0.0, 0.0, wall.height);
        let p4 = wall.edge2 + Vec3::cons(0.0, 0.0, wall.height);

        let (x1, y1) = self.world_to_screen(&p1);
        let (x2, y2) = self.world_to_screen(&p2);
        let (x3, y3) = self.world_to_screen(&p3);
        let (x4, y4) = self.world_to_screen(&p4);
        self.buffer.draw_line(x1, y1, x2, y2);
        self.buffer.draw_line(x3, y3, x4, y4);
    }
}

pub fn draw_view_2d(viewmodel: &ViewModel, buffer: &mut Buffer) {
    draw_point_2d(buffer, &viewmodel.position);

    let (sin, cos): (Float, Float) = viewmodel.rotation.sin_cos();
    let (sx, sy): (usize, usize) = (
        (viewmodel.position.x + buffer.halfwidth() as Float) as usize,
        (viewmodel.position.y + buffer.halfheight() as Float) as usize,
    );
    let (nx, ny): (usize, usize) = (
        (sx as Float + 10.0 * cos) as usize,
        (sy as Float + 10.0 * sin) as usize,
    );
    buffer.draw_line(sx, sy, nx, ny);
}

pub fn draw_point_2d(buffer: &mut Buffer, point: &Vec3<Float>) {
    let sx: usize = (point.x + buffer.halfwidth() as Float) as usize;
    let sy: usize = (point.y + buffer.halfheight() as Float) as usize;
    buffer.draw_cluster(sx, sy, 3, color_tag(2));
}

pub fn draw_wall_2d(buffer: &mut Buffer, wall: &Wall) {
    let sx1 = (wall.edge1.x + buffer.halfwidth() as Float) as usize;
    let sy1 = (wall.edge1.y + buffer.halfheight() as Float) as usize;
    let sx2 = (wall.edge2.x + buffer.halfwidth() as Float) as usize;
    let sy2 = (wall.edge2.y + buffer.halfheight() as Float) as usize;
    buffer.draw_line(sx1, sy1, sx2, sy2);
}
