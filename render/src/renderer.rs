


use crate::viewmodel::{delay, ViewModel};
use crate::buffer::Buffer;



pub struct Renderer<'d>
{
    pub viewmodel: &'d ViewModel,
    pub buffer: &'d mut Buffer,
}

impl<'d> Renderer<'d>
{
    pub fn cons(viewmodel: &'d ViewModel, buffer: &'d mut Buffer) -> Renderer<'d>
    {
        Renderer { viewmodel, buffer }
    }

    pub fn delay_update(&self)
    {
        delay(50);
    }
}
