


use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};

use crate::{geometry::{Mesh, Tri}, math::{Vec2f, Vec3f}, render_utils::{Buffer, Camera}, renderer::Renderer};



/* keep all this out of main

it seems like if this stuff is put into main, even tiny changes during
testing makes the linker have to relink everything. the compilation time (even
for a single line change) is like 10x, so I just put everything in here so that
only this file has to be recompiled and everything linked from main stays linked
I have no idea if this is actually how linking works, this is just my theory but I
literally can't even compile on a RPI5 without pulling this into here */



pub fn make_window(buffer: &Buffer, fps: usize, scale: Scale) -> Window {
    let mut window = Window::new(
        "",
        buffer.width,
        buffer.height,
        WindowOptions {
            scale,
            ..Default::default()
        },
    )
    .unwrap();
    window.set_target_fps(fps);
    window
}

#[allow(unused_variables, unused_mut)]
pub fn make_mesh() -> Mesh {
    let tri1 = Tri::cons_pos(
        Vec3f::cons(0, -50, 20),
        Vec3f::cons(0, 50, 20),
        Vec3f::cons(0, 0, -50),
    );
    let tri2 = Tri::cons_pos(
        Vec3f::cons(0, 50, 20),
        Vec3f::cons(0, 50, -20),
        Vec3f::cons(0, 0, -50),
    );

    let mut mesh = Mesh::cons(vec![tri1], Vec3f::cons(0, -50, 0), None);
    mesh.tris.push(tri2);
    let mut mesh = Mesh::build_from_file("misc/icosahedron.vert", 55.);
    let mut mesh = Mesh::build_from_file_extended("ak47/ak47.obj", 1., Some("ak47/ak47.png"));
    let mut mesh = Mesh::build_from_file_extended("misc/eyeball.obj", 35., None);
    let mut mesh = Mesh::build_from_file_extended("portal/portal.obj", 55., Some("portal/portal_tex.jpg"));
    mesh.center = Vec3f::cons(0, 0, 0);
    mesh
}

pub fn handle_mutation_input(window: &Window, mesh: &mut Mesh, mouse: &mut Option<Vec2f>) {
    if !window.is_key_down(Key::R) {
        if window.is_key_down(Key::K) {
            let mut rotation = Vec3f::cons(0., 0., 0.1);
            rotation.inv_rot_zyx(mesh.rotation);
            mesh.rotate_z(rotation.z);
            mesh.rotate_y(rotation.y);
            mesh.rotate_x(rotation.x);
        }
        if window.is_key_down(Key::W) {
            mesh.rotate_y(0.02);
        }
        if window.is_key_down(Key::S) {
            mesh.rotate_y(-0.02);
        }
        if window.is_key_down(Key::A) {
            mesh.rotate_z(0.02);
        }
        if window.is_key_down(Key::D) {
            mesh.rotate_z(-0.02);
        }
        if window.is_key_down(Key::Q) {
            mesh.rotate_x(0.02);
        }
        if window.is_key_down(Key::E) {
            mesh.rotate_x(-0.02);
        }
    }
    else {
        mesh.rotate_x(0.01);
        mesh.rotate_y(0.005);
        mesh.rotate_z(0.01);
    }

    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
        if window.get_mouse_down(MouseButton::Left) {
            if let Some(past_pos) = mouse {
                let screen_dx = -(past_pos.x - x);
                let screen_dy = past_pos.y - y;

                mesh.rotate_y(screen_dy * 0.01);
                mesh.rotate_z(screen_dx * 0.01);
            }
        }
        *mouse = Some(Vec2f::cons(x, y));
    }
    else {
        *mouse = None;
    }
}

pub fn handle_renderer_input(window: &Window, mut renderer: Renderer) {
    if !window.is_key_down(Key::P) {
        renderer.render_mesh();
    }
    if window.is_key_down(Key::O) {
        renderer.render_wireframe();
    }
}

pub fn handle_camera_input(window: &Window, camera: &mut Camera) {
    if window.is_key_down(Key::Up) {
        camera.position.x += 0.5;
    }
    else if window.is_key_down(Key::Down) {
        camera.position.x -= 0.5;
    }
    else if window.is_key_down(Key::Right) {
        camera.rotation.z += 0.05;
    }
    else if window.is_key_down(Key::Left) {
        camera.rotation.z -= 0.05;
    }
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
