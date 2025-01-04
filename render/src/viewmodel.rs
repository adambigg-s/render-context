use crate::math::{tau, Vec3};
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
            rotation: Float::default(),
            tilt: Int::default(),
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
            self.rotation += tau();
        } else if self.rotation > tau() {
            self.rotation -= tau();
        }
    }

    pub fn tilt(&mut self, delta: Int) {
        self.tilt += delta;
        self.tilt = self.tilt.clamp(-70, 70);
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
