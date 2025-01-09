


use std::{error::Error, io::{stdout, Write}, time::Duration};

use crossterm::event::{self, Event, KeyCode};

use crate::ViewModel;



pub fn print_debug(viewmodel: &ViewModel) {
    print!("\x1b[H");
    print!("x: {:.2}, y: {:.2}, z: {:.2}, rot: {:.2}, tilt: {:.2}, speed: {:.2}",
        viewmodel.pos.x, viewmodel.pos.y, viewmodel.pos.z,
        viewmodel.rot, viewmodel.tilt, viewmodel.transspeed
    );
    stdout().flush().unwrap();
}

pub fn sleep(time: u64) {
    std::thread::sleep(std::time::Duration::from_millis(time));
}

pub fn get_user_input() -> Vec<char> {
    let mut inputs = Vec::new();
    if let Ok(true) = event::poll(Duration::from_millis(100)) {
        if let Ok(Event::Key(key_event)) = event::read() {
            match key_event.code {
                KeyCode::Char('w') => inputs.push('w'),
                KeyCode::Char('s') => inputs.push('s'),
                KeyCode::Char('W') => inputs.push('W'),
                KeyCode::Char('S') => inputs.push('S'),
                KeyCode::Char('a') => inputs.push('a'),
                KeyCode::Char('d') => inputs.push('d'),
                KeyCode::Char('q') => inputs.push('q'),
                KeyCode::Char('e') => inputs.push('e'),
                KeyCode::Char('p') => inputs.push('p'),
                KeyCode::Char('f') => inputs.push('f'),
                KeyCode::Char('r') => inputs.push('r'),
                KeyCode::Char('t') => inputs.push('t'),
                KeyCode::Char('g') => inputs.push('g'),
                KeyCode::Char('[') => inputs.push('['),
                KeyCode::Char(']') => inputs.push(']'),
                KeyCode::Char('1') => inputs.push('1'),
                KeyCode::Char('2') => inputs.push('2'),
                KeyCode::Char('3') => inputs.push('3'),
                KeyCode::Char('4') => inputs.push('4'),
                KeyCode::Char('5') => inputs.push('5'),
                KeyCode::Char('6') => inputs.push('6'),
                KeyCode::Char('7') => inputs.push('7'),
                KeyCode::Char('8') => inputs.push('8'),
                KeyCode::Char('9') => inputs.push('9'),
                KeyCode::Char('0') => inputs.push('0'),
                _ => {}
            }
        }
    }
    inputs
}

pub fn flash_error(error: Box<dyn Error>, time: u64) {
    println!("error: {}", error);
    sleep(time);
}

#[inline]
pub fn dump<Any>(_thing: Any) {}
