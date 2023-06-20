use crate::calculator::{Color, DeviceDislay, Event};
use crate::gui::Widget;
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};

/// A colored rectangle, with an outline of a different color.
#[derive(Debug)]
pub struct ColorRect {
    bounding_box: Rectangle,
    fill_color: Color,
    border_color: Color,
    border_width: u32,
}
impl ColorRect {
    pub fn new(
        fill_color: Color,
        border_color: Color,
        border_width: u32,
        bounding_box: Rectangle,
    ) -> Self {
        Self {
            fill_color,
            border_color,
            border_width,
            bounding_box,
        }
    }
}
impl Widget for ColorRect {
    fn on_event(&mut self, e: Event) -> Option<Event> {
        Some(e)
    }
    fn render(&self, target: &mut DeviceDislay, focused: bool) {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(self.fill_color)
            .stroke_color(if focused {
                self.border_color
            } else {
                Color::CSS_GRAY
            })
            .stroke_width(self.border_width)
            .build();
        self.bounding_box.into_styled(style).draw(target).unwrap();
    }
    fn set_bounding_box(&mut self, bounding_box: Rectangle) {
        self.bounding_box = bounding_box;
    }
}
