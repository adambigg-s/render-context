#![cfg_attr(rustfmt, rustfmt_skip)]



use crate::{Float, Int};
use crate::render_utils::{Buffer, Camera, Color};
use crate::math::{Vec2i, Vec3f};
use crate::geometry::{BarycentricSystem, Mesh, PolyData, RefFrame, Tri};



#[allow(dead_code)]
pub struct Renderer<'d> {
    buffer: &'d mut Buffer,
    mesh: &'d Mesh,
    camera: &'d Camera,
    lighting_vec: Vec3f,
    scale: Float,
    overdraw_percent: Float,
    minimum_lighting: Float,
}

impl<'d> Renderer<'d> {
    pub fn cons(buffer: &'d mut Buffer, mesh: &'d Mesh, camera: &'d Camera, fov: Float) -> Renderer<'d> {
        let mut lighting_vec = Vec3f::cons(-3, 1, -4);
        lighting_vec.normalize();
        let scale = buffer.get_half_width() / (fov / 2.).to_degrees().tan();
        let overdraw_percent = 0.20;
        let minimum_lighting = 0.15;

        Renderer { buffer, mesh, camera, lighting_vec, scale, overdraw_percent, minimum_lighting }
    }

    pub fn render_mesh(&mut self) {
        self.mesh.tris.iter().for_each(|tri| {
            self.render_triangle(tri);
        });
    }

    pub fn render_wireframe(&mut self) {
        self.mesh.tris.iter().for_each(|tri| {
            if let Some(polydata) = self.initialize_triangle_render(tri) {
                let mut color = Color::cons(0, 255, 255);
                color.attenuate(polydata.lighting);
                self.draw_line_screen(polydata.tri.a.pos, polydata.tri.b.pos, color);
                self.draw_line_screen(polydata.tri.a.pos, polydata.tri.c.pos, color);
                self.draw_line_screen(polydata.tri.c.pos, polydata.tri.b.pos, color);
            }
        });
    }

    pub fn render_refframe(&mut self, frame: &RefFrame) {
        let mut frame = *frame;

        let mut x_arm = Vec3f::cons(frame.length, 0., 0.);
        let mut y_arm = Vec3f::cons(0., frame.length, 0.);
        let mut z_arm = Vec3f::cons(0., 0., frame.length);

        x_arm.rot_zyx(-self.camera.rotation);
        y_arm.rot_zyx(-self.camera.rotation);
        z_arm.rot_zyx(-self.camera.rotation);

        x_arm.rot_zyx(self.mesh.rotation);
        y_arm.rot_zyx(self.mesh.rotation);
        z_arm.rot_zyx(self.mesh.rotation);

        frame.translate(-self.camera.position);
        x_arm += frame.center;
        y_arm += frame.center;
        z_arm += frame.center;

        self.draw_line_world(frame.center, x_arm, Color::cons(255, 0, 0));
        self.draw_line_world(frame.center, y_arm, Color::cons(0, 255, 0));
        self.draw_line_world(frame.center, z_arm, Color::cons(0, 0, 255));
    }

    fn render_triangle(&mut self, tri: &Tri) {
        if let Some(polydata) = self.initialize_triangle_render(tri) {
            let (a, b, c) = (polydata.tri.a.pos, polydata.tri.b.pos, polydata.tri.c.pos);
            if polydata.tri.lumped_left() {
                self.trace_and_fill(&polydata, a, c, a, b);
                self.trace_and_fill(&polydata, c, a, c, b);
            }
            else {
                self.trace_and_fill(&polydata, a, b, a, c);
                self.trace_and_fill(&polydata, c, b, c, a);
            }
        }
    }

    fn initialize_triangle_render(&mut self, tri: &Tri) -> Option<PolyData> {
        let mut triangle: Tri = *tri;
        
        // super super needs to be changed! haven't done lighting yet and this is a
        // major bottleneck at the current moment. def a better way to do this, maybe have
        // PolyData hold two norms world and viewframe and can be stored during poly calcs
        // so norm doesn't have to be done like 3 times for no reason
        let mut world_norm = triangle.get_normal();
        world_norm.rot_zyx(self.mesh.rotation);
        let lighting = self.lighting_vec.inner_prod(&world_norm).max(self.minimum_lighting);
        
        self.transform_tri(&mut triangle);

        let norm = triangle.get_normal();
        if norm.x > self.overdraw_percent {
            return None;
        }

        self.transform_to_screen(&mut triangle);
        if triangle.behind_view() {
            return None;
        }
        
        triangle.sort_verts_vertical();
        
        Some(PolyData::cons(triangle, norm, lighting))
    }

    fn trace_and_fill(&mut self, poly: &PolyData, e1s: Vec3f, e1e: Vec3f, e2s: Vec3f, e2e: Vec3f) {
        let mut e1 = EdgeTracer::cons(e1s, e1e);
        let mut e2 = EdgeTracer::cons(e2s, e2e);
        let barycentric = BarycentricSystem::cons(&poly.tri);
        while let (Some(p1), Some(p2)) = (e1.step_constant(), e2.step_constant()) {
            self.fill_edge_trace(&p1, &p2, poly, &barycentric);
        }
    }

    fn transform_to_screen(&self, triangle: &mut Tri) {
        triangle.a.pos = self.view_to_screen(&triangle.a.pos);
        triangle.b.pos = self.view_to_screen(&triangle.b.pos);
        triangle.c.pos = self.view_to_screen(&triangle.c.pos);
        triangle.a.pos.clamp_xy(0., self.buffer.get_width(), 0., self.buffer.get_height());
        triangle.b.pos.clamp_xy(0., self.buffer.get_width(), 0., self.buffer.get_height());
        triangle.c.pos.clamp_xy(0., self.buffer.get_width(), 0., self.buffer.get_height());
    }

    fn transform_tri(&mut self, triangle: &mut Tri) {
        triangle.rot_xyz(self.mesh.rotation);
        triangle.translate(-self.camera.position);
        triangle.translate(self.mesh.center);
        triangle.rot_zyx(-self.camera.rotation);
    }

    fn fill_edge_trace(
        &mut self, starting: &Vec2i, ending: &Vec2i, poly: &PolyData, bary: &BarycentricSystem)
    {
        {
            debug_assert!(starting.y == ending.y);
        }

        let y = starting.y;
        for x in starting.x..=ending.x {
            if !self.buffer.inbounds(x as usize, y as usize) { return; }

            let mut color = Color::default();
            let coords = bary.get_coords(x, y);
            let texture_x = poly.tri.interpolate_tex_u(&coords);
            let texture_y = poly.tri.interpolate_tex_v(&coords);

            if let Some(texture) = &self.mesh.texture {
                color = texture.get_texture(texture_x, texture_y);
            }
            else {
                color.red = poly.tri.get_red_ordered_vec().inner_prod(&coords);
                color.green = poly.tri.get_green_ordered_vec().inner_prod(&coords);
                color.blue = poly.tri.get_blue_ordered_vec().inner_prod(&coords);
            }

            color.attenuate(poly.lighting);
            let depth = poly.tri.interpolate_depth_nonlinear(coords);

            self.buffer.set(x as usize, y as usize, color, depth);
        }
    }

    fn draw_line_screen(&mut self, p1: Vec3f, p2: Vec3f, color: Color) {
        let mut edge = EdgeTracer::cons(p1, p2);
        while let Some(point) = edge.step_once() {
            if !self.buffer.inbounds(point.x as usize, point.y as usize) { return; }

            self.buffer.set(point.x as usize, point.y as usize, color, 1.);
        }
    }

    fn draw_line_world(&mut self, p1: Vec3f, p2: Vec3f, color: Color) {
        let p1 = self.view_to_screen(&p1);
        let p2 = self.view_to_screen(&p2);
        self.draw_line_screen(p1, p2, color);
    }

    fn view_to_screen(&self, target: &Vec3f) -> Vec3f {
        let scrx = target.y / target.x * self.scale + self.buffer.get_half_width();
        let scry = -target.z / target.x * self.scale + self.buffer.get_half_height();

        Vec3f::cons(scrx, scry, target.x)
    }
}



pub struct EdgeTracer {
    current: Vec2i,
    target: Vec2i,
    steps: Vec2i,
    deltas: Vec2i,
    error: Int,
}

impl EdgeTracer {
    pub fn cons(start: Vec3f, end: Vec3f) -> EdgeTracer {
        let current = Vec2i::cons(start.x.ceil() as Int, start.y.ceil() as Int);
        let target = Vec2i::cons(end.x.ceil() as Int, end.y.ceil() as Int);
        let dx: i32 = (target.x - current.x).abs();
        let dy: i32 = -(target.y - current.y).abs();
        let int_step_x: i32 = if current.x < target.x { 1 } else { -1 };
        let int_step_y: i32 = if current.y < target.y { 1 } else { -1 };
        let error: i32 = dx + dy;
        
        EdgeTracer {
            current, target, steps: Vec2i::cons(int_step_x, int_step_y), deltas: Vec2i::cons(dx, dy), error
        }
    }

    pub fn step_once(&mut self) -> Option<Vec2i> {
        let twice_error: i32 = 2 * self.error;
        if twice_error >= self.deltas.y {
            if self.current.x == self.target.x { return None; }
            self.error += self.deltas.y;
            self.current.x += self.steps.x;
        }
        else if twice_error <= self.deltas.x {
            if self.current.y == self.target.y { return None; }
            self.error += self.deltas.x;
            self.current.y += self.steps.y
        }
        Some(self.current)
    }

    pub fn step_constant(&mut self) -> Option<Vec2i> {
        let startingy = self.current.y;
        while self.current.y == startingy {
            self.step_once()?;
        }
        Some(self.current)
    }
}
