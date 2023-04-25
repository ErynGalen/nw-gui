use heapless::String;

use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::Text,
};

use crate::calculator::{Color, DeviceDislay};

pub use embedded_graphics::mono_font::ascii::FONT_7X13 as NORMAL_FONT;

#[derive(Debug)]
pub enum Widget {
    Text(TextWidget),
    Rect(RectWidget),
}
impl Widget {
    pub fn render(&self, target: &mut DeviceDislay) {
        match self {
            Self::Text(w) => w.render(target),
            Self::Rect(w) => w.render(target),
        }
    }
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

    /// can fail if the string is longer than the widget can hold
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

#[derive(Debug)]
pub struct RectWidget {
    pub bounging_box: Rectangle,
    pub fill_color: Color,
    pub border_color: Color,
    /// top, right, bottom, left
    pub border_width: u32,
}

impl RectWidget {
    pub fn render(&self, target: &mut DeviceDislay) {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(self.fill_color)
            .stroke_color(self.border_color)
            .stroke_width(self.border_width)
            .build();
        self.bounging_box.into_styled(style).draw(target).unwrap();
    }
}

pub struct Splitter {
    pub bounding_box: Rectangle,
    pub vertical: bool,
    pub ratio: f32,
}

impl Splitter {
    pub fn bounding_box_a(&self) -> Rectangle {
        let mut size = self.bounding_box.size;
        if self.vertical {
            let height = self.ratio * size.height as f32;
            size = Size::new(size.width, height as u32);
        } else {
            let width = self.ratio * size.width as f32;
            size = Size::new(width as u32, size.height);
        }
        Rectangle::new(self.bounding_box.top_left, size)
    }
    pub fn bounding_box_b(&self) -> Rectangle {
        let mut size = self.bounding_box.size;
        let mut top_left = self.bounding_box.top_left;
        if self.vertical {
            let removed_height = self.ratio * size.height as f32;
            size = Size::new(size.width, size.height - removed_height as u32);
            top_left = Point::new(top_left.x, top_left.y + removed_height as i32);
        } else {
            let removed_width = self.ratio * size.width as f32;
            size = Size::new(size.width - removed_width as u32, size.height);
            top_left = Point::new(top_left.x + removed_width as i32, top_left.y);
        }
        Rectangle::new(top_left, size)
    }
}
