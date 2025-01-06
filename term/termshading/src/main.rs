


use std::{thread::sleep, time::Duration};

use crossterm::event::{self, Event, KeyCode};



type Float = f32;

fn main() {
    loop {
        let inputs = get_user_input();

        for input in &inputs {
            println!("pressed: {}", input);
        }

        if inputs.contains(&'q') {
            break;
        }

        sleep(Duration::from_millis(10));
    }
}

struct ViewModel {
    pos: Vec3,
    rot: Float,
    tilt: Float,
}

impl ViewModel {
    fn new() -> ViewModel {
        ViewModel { pos: Vec3::cons(0.0, 0.0, 0.0), rot: 0.0, tilt: 0.0 }
    }
}

struct Vec3 {
    x: Float, y: Float, z: Float,
}

impl Vec3 {
    fn cons<T>(x: T, y: T, z: T) -> Vec3
    where T: Into<Float> {
        Vec3 { x: x.into(), y: y.into(), z: z.into() }
    }
}

fn get_user_input() -> Vec<char> {
    let mut inputs = Vec::new();
    if let Ok(true) = event::poll(Duration::from_millis(10)) {
        if let Ok(Event::Key(key_event)) = event::read() {
            match key_event.code {
                KeyCode::Char('w') => inputs.push('w'),
                KeyCode::Char('s') => inputs.push('s'),
                KeyCode::Char('a') => inputs.push('a'),
                KeyCode::Char('d') => inputs.push('d'),
                KeyCode::Char('q') => inputs.push('q'),
                _ => {}
            }
        }
    }
    inputs
}
