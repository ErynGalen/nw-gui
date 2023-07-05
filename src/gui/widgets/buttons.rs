use super::ColorRect;
use crate::calculator::{Event, KeyCode};
use crate::gui::{Callback, Color, Widget, NORMAL_FONT};

use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::*,
    primitives::Rectangle,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use heapless::String;

/// A button widget.
///
/// `T` is the type of [`Widget::Context`].
#[derive(Debug, Clone)]
pub struct Button<T> {
    background: ColorRect<T>,
    text: String<16>,
    text_color: Color,
    pressed: bool,
    on_pressed: Callback<T>,
}
impl<T> Button<T> {
    /// Creates a new button.
    pub fn new(
        text: String<16>,
        text_color: Color,
        background_color: Color,
        border_width: u32,
        bounding_box: Rectangle,
        on_pressed: fn(&mut T),
    ) -> Self {
        Self {
            background: ColorRect::new(background_color, text_color, border_width, bounding_box),
            text,
            text_color,
            pressed: false,
            on_pressed: Callback(on_pressed),
        }
    }
    /// Modify the text displayed on the button.
    pub fn set_text(&mut self, text: String<16>) {
        self.text = text;
    }
}
impl<T> Widget for Button<T> {
    type Context = T;

    fn on_event(&mut self, e: Event, context: &mut T) -> Option<crate::calculator::Event> {
        // call self.closure
        match e {
            Event::KeyDown(KeyCode::Ok) | Event::KeyDown(KeyCode::Exe) => {
                self.pressed = true;
                (self.on_pressed.0)(context);
                None
            }
            Event::KeyUp(KeyCode::Ok) | Event::KeyUp(KeyCode::Exe) => {
                self.pressed = false;
                None
            }
            _ => Some(e),
        }
    }
    fn render(&self, target: &mut crate::calculator::DeviceDislay, focused: bool) {
        let text_color = if self.pressed { Color::RED } else { self.text_color };
        self.background.render(target, focused);
        let character_style = MonoTextStyleBuilder::new()
            .font(&NORMAL_FONT)
            .text_color(text_color)
            .build();
        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Middle)
            .build();
        Text::with_text_style(
            &self.text,
            self.background.get_bounding_box().center(),
            character_style,
            text_style,
        )
        .draw(target)
        .unwrap();
    }
    fn set_bounding_box(&mut self, bounding_box: embedded_graphics::primitives::Rectangle) {
        self.background.set_bounding_box(bounding_box);
    }
    fn get_bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        self.background.get_bounding_box()
    }
}
