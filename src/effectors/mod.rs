pub mod checkout_effector;

use crate::facts::{event::Event, offset::Offset};

pub trait Effector {
    fn name(&self) -> &'static str;
    fn last_offset(&self) -> Offset;
    fn apply(&mut self, event: &Event);
}
