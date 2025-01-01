


use std::ops::AddAssign;



pub struct Vec3<T>
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
{
    pub fn cons(x: T, y: T, z: T) -> Vec3<T>
    {
        Vec3 { x, y, z }
    }
}

impl<T> AddAssign for Vec3<T>
where T: AddAssign
{
    fn add_assign(&mut self, other: Self)
    {
        self.x += other.x; self.y += other.y; self.z += other.z;
    }
}
