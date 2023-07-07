use core::marker::PhantomData;

use crate::calculator::{DeviceDislay, Event};
use crate::gui::theme::Theme;
use crate::gui::{FocusFrom, Widget};
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};

/// A colored rectangle.
///
/// The colors of the rectagle are defined by the theme the rectanble is rendered with.
#[derive(Debug, Clone, Copy)]
pub struct ColorRect<T> {
    bounding_box: Rectangle,
    _context: PhantomData<T>,
}
impl<T> ColorRect<T> {
    pub fn new(bounding_box: Rectangle) -> Self {
        Self {
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
    fn render(&self, target: &mut DeviceDislay, theme: &Theme) {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(theme.background)
            .stroke_color(theme.foreground)
            .stroke_width(theme.rect_border)
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
