use core::marker::PhantomData;

use crate::calculator::{Color, DeviceDislay, Event};
use crate::gui::{FocusFrom, Widget};
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};

/// A colored rectangle, with an outline of a different color.
#[derive(Debug, Clone, Copy)]
pub struct ColorRect<T> {
    bounding_box: Rectangle,
    pub fill_color: Color,
    pub border_color: Color,
    pub border_width: u32,
    _context: PhantomData<T>,
}
impl<T> ColorRect<T> {
    pub fn new(fill_color: Color, border_color: Color, border_width: u32, bounding_box: Rectangle) -> Self {
        Self {
            fill_color,
            border_color,
            border_width,
            bounding_box,
            _context: PhantomData,
        }
    }
}
impl<T> Widget for ColorRect<T> {
    type Context = T;

    fn on_event(&mut self, e: Event, _context: &mut Self::Context) -> Option<Event> {
        Some(e)
    }
    fn render(&self, target: &mut DeviceDislay) {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(self.fill_color)
            .stroke_color(self.border_color)
            .stroke_width(self.border_width)
            .build();
        self.bounding_box.into_styled(style).draw(target).unwrap();
    }
    fn set_bounding_box(&mut self, bounding_box: Rectangle) {
        self.bounding_box = bounding_box;
    }
    fn get_bounding_box(&self) -> Rectangle {
        self.bounding_box
    }
    fn get_focus(&self) -> Option<bool> {
        None
    }
    fn set_focus(&mut self, _: Option<FocusFrom>) -> Result<(), ()> {
        Err(())
    }
}
