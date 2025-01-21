


use std::f32::consts::{PI, TAU};

use minifb::Key;

use crate::math::Vec3;
use crate::{Float, Int};



pub struct ViewModel {
    pub position: Vec3<Float>,
    pub rotation: Float,
    pub tilt: Int,
}

impl ViewModel {
    pub fn cons(position: Vec3<Float>) -> ViewModel {
        ViewModel {
            position,
            rotation: 0.0,
            tilt: 0,
        }
    }

    pub fn move_forward(&mut self, direction: Float, speed: Float) {
        self.position.x += direction * speed * self.rotation.cos();
        self.position.y += direction * speed * self.rotation.sin();
    }

    pub fn move_lateral(&mut self, direction: Float, speed: Float) {
        self.position.x -= direction * speed * self.rotation.sin();
        self.position.y += direction * speed * self.rotation.cos();
    }

    pub fn rotate(&mut self, delta: Float) {
        self.rotation += delta;
        if self.rotation < 0.0 {
            self.rotation += TAU;
        }
        else if self.rotation > TAU {
            self.rotation -= TAU;
        }
    }

    pub fn tilt(&mut self, delta: Int) {
        self.tilt += delta;
        self.tilt = self.tilt.clamp(-70, 70);
    }

    pub fn get_movement(&mut self, keys: Vec<Key>) {
        keys.iter().for_each(|key| {
            match key {
                Key::Q => self.rotate(1.0 / PI / 3.0),
                Key::E => self.rotate(-1.0 / PI / 3.0),
                Key::W => self.move_forward(1.0, 5.0),
                Key::S => self.move_forward(-1.0, 5.0),
                Key::A => self.move_lateral(1.0, 5.0),
                Key::D => self.move_lateral(-1.0, 5.0),
                Key::R => self.tilt(-1),
                Key::F => self.tilt(1),
                _ => {}
            };
        });
    }    
}



#[cfg(test)]
mod test {
    #[test]
    fn float_cast_usize() {
        assert!(-13.04 as usize == 0);
    }

    #[test]
    fn negint_cast_usize() {
        assert!(34_i32 as usize == 34);
        assert!(-34_i32 as usize > 34);
    }
}
