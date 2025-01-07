


use std::{io::{stdout, Write}, time::Duration};

use crossterm::event::{self, Event, KeyCode};

use crate::ViewModel;



pub fn print_debug(viewmodel: &ViewModel) {
    print!("\x1b[H");
    println!("x: {:.2}, y: {:.2}, z: {:.2}, rot: {:.2}, tilt: {:.2}",
        viewmodel.pos.x, viewmodel.pos.y, viewmodel.pos.z, viewmodel.rot, viewmodel.tilt
    );
    stdout().flush().unwrap();
}

pub fn sleep(time: u64) {
    std::thread::sleep(std::time::Duration::from_millis(time));
}

pub fn get_user_input() -> Vec<char> {
    let mut inputs = Vec::new();
    if let Ok(true) = event::poll(Duration::from_millis(1)) {
        if let Ok(Event::Key(key_event)) = event::read() {
            match key_event.code {
                KeyCode::Char('w') => inputs.push('w'),
                KeyCode::Char('s') => inputs.push('s'),
                KeyCode::Char('a') => inputs.push('a'),
                KeyCode::Char('d') => inputs.push('d'),
                KeyCode::Char('q') => inputs.push('q'),
                KeyCode::Char('e') => inputs.push('e'),
                KeyCode::Char('p') => inputs.push('p'),
                KeyCode::Char('f') => inputs.push('f'),
                KeyCode::Char('r') => inputs.push('r'),
                KeyCode::Char('t') => inputs.push('t'),
                KeyCode::Char('g') => inputs.push('g'),
                _ => {}
            }
        }
    }
    inputs
}
