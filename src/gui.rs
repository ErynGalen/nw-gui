use heapless::{String, Vec};

use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};

use crate::numworks_display::{Color, DeviceDislay};

pub use embedded_graphics::mono_font::ascii::FONT_7X13 as NORMAL_FONT;

#[derive(Debug)]
pub enum Widget {
    Text(TextWidget),
}
impl Widget {
    pub fn render(&self, target: &mut DeviceDislay) {
        match self {
            Self::Text(w) => w.render(target),
        }
    }
}

pub const MAX_WIDGETS: usize = 32;
#[derive(Debug)]
pub struct Gui {
    widgets: Vec<Widget, MAX_WIDGETS>,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }
    pub fn add_widget(&mut self, w: Widget) -> Result<(), Widget> {
        if self.widgets.is_full() {
            return Err(w);
        } else {
            return self.widgets.push(w);
        }
    }
    pub fn render(&self, target: &mut DeviceDislay) {
        for w in &self.widgets {
            w.render(target);
        }
    }
}

#[derive(Debug)]
pub struct TextWidget {
    pub bounding_box: Rectangle,
    pub color: Color,
    pub bg_color: Color,
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
                + (text_size.height as i32) / 6, // text vertical anchor point is 2/3 of a character height
        );

        let style = PrimitiveStyle::with_fill(self.bg_color);
        // Rectangle::new(text_position, text_size)
        //     .into_styled(style)
        //     .draw(target)
        //     .unwrap();
        self.bounding_box.into_styled(style).draw(target).unwrap();

        let text = Text::new(&self.text, text_position, text_style);
        text.draw(target).unwrap();
    }

    // can fail if the string is longer than the widget can hold
    pub fn new(bounding_box: Rectangle, color: Color, bg_color: Color, text: &str) -> Option<Self> {
        let s = Self {
            bounding_box,
            color,
            bg_color,
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
