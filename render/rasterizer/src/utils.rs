


use minifb::{Scale, Window, WindowOptions};

use crate::render_utils::Buffer;



pub fn make_window(buffer: &Buffer, fps: usize, scale: Scale) -> Window {
    let mut window = Window::new(
        "",
        buffer.width,
        buffer.height,
        WindowOptions { scale, ..Default::default() }
    ).unwrap();
    window.set_target_fps(fps);
    window
}



// const OPTION: bool = false;

// fn gen_range(x: i32) -> Range<i32> {
//     #[ifdef(OPTION, true)]
//     {
//         return 0..x
//     }
//     #[else]
//     {
//         return 0..=x
//     }
//     #[endif]
// }
