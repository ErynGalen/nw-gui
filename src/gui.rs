//! This module contains types and  traits used to create a GUI.

use heapless::String;

use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::Rectangle,
    text::Text,
};

use crate::calculator::{Color, DeviceDislay, Event};

pub mod storage;
pub mod widgets;

/// This font should be used to render text.
pub use embedded_graphics::mono_font::ascii::FONT_7X13 as NORMAL_FONT;

/// The GUI is made of objects implementing the `Widget` trait.
pub trait Widget {
    /// The `render()` method draws the widget onto the given target.
    fn render(&self, target: &mut DeviceDislay, focused: bool);
    /// The `on_event()` method dispatches the given event to the widget,
    /// which may pass the event to its children.
    ///
    /// Return the event back if it hasn't been used.
    fn on_event(&mut self, e: Event) -> Option<Event>;

    /// Set the bounding box of a widget
    fn set_bounding_box(&mut self, bounding_box: Rectangle);
}
#[derive(Debug)]
pub struct TextWidget {
    pub bounding_box: Rectangle,
    pub color: Color,
    pub text: String<32>,
}
impl TextWidget {
    pub fn render(&self, target: &mut DeviceDislay) {
        let text_style = MonoTextStyle::new(&NORMAL_FONT, self.color);
        let text = Text::new(&self.text, Point::new(0, 0), text_style);

        let text_size = text.bounding_box().size;
        let text_position = Point::new(
            self.bounding_box.top_left.x
                + (self.bounding_box.size.width as i32 - text_size.width as i32) / 2,
            self.bounding_box.top_left.y
                + (self.bounding_box.size.height as i32) / 2
                + (text_size.height as i32) / 6, // text vertical anchor point is at 2/3 of a character height
        );

        let text = Text::new(&self.text, text_position, text_style);
        text.draw(target).unwrap();
    }

    /// Can fail if the string is longer than the widget can hold.
    pub fn new(bounding_box: Rectangle, color: Color, text: &str) -> Option<Self> {
        let s = Self {
            bounding_box,
            color,
            text: String::new(),
        };
        if text.len() > s.text.capacity() {
            return None;
        }
        Some(Self {
            text: String::from(text),
            ..s
        })
    }
}
