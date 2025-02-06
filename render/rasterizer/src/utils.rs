


use minifb::{Scale, Window, WindowOptions};

use crate::render_utils::Buffer;



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
