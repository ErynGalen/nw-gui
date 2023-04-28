use crate::calculator::{Event, DeviceDisplay};

pub trait App {
    fn on_event(&mut self, e: Event);
    fn render(&self, target: &mut DeviceDisplay);
}