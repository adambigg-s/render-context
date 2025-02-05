


use minifb::{Key, Scale, Window, WindowOptions};

use crate::render_utils::Buffer;
use crate::geometry::Mesh;



pub fn handle_inputs(window: &Window, mesh: &mut Mesh) {
    if !window.is_key_down(Key::R) {
        if window.is_key_down(Key::W) {
            mesh.rotatex(0.02);
        }
        if window.is_key_down(Key::A) {
            mesh.rotatey(0.02);
        }
        if window.is_key_down(Key::Q) {
            mesh.rotatez(0.02);
        }
        if window.is_key_down(Key::S) {
            mesh.rotatex(-0.02);
        }
        if window.is_key_down(Key::D) {
            mesh.rotatey(-0.02);
        }
        if window.is_key_down(Key::E) {
            mesh.rotatez(-0.02);
        }
    }
}

pub fn make_window(buffer: &Buffer, fps: usize) -> Window {
    let mut window = Window::new(
        "software rendering mesh",
        buffer.width,
        buffer.height,
        WindowOptions { scale: Scale::X2, ..Default::default() }
    ).unwrap();
    window.set_target_fps(fps);
    window
}
