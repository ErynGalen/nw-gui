use heapless::{String, Vec};

use embedded_graphics::{
    mono_font::{ascii::FONT_5X7, MonoTextStyle},
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};

use crate::numworks_display::{Color, DeviceDislay};

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
    x: i32,
    y: i32,
    color: Color,
    bg_color: Color,
    text: String<16>,
}
impl TextWidget {
    pub fn render(&self, target: &mut DeviceDislay) {
        let text_style = MonoTextStyle::new(&FONT_5X7, self.color);
        let text = Text::new(&self.text, Point::new(self.x, self.y), text_style);

        let bounding_box = text.bounding_box();
        let bounding_box = Rectangle::new(
            Point::new(bounding_box.top_left.x - 3, bounding_box.top_left.y - 3),
            Size::new(
                bounding_box.size.width + 6,
                bounding_box.size.height + 6,
            ),
        );

        let style = PrimitiveStyle::with_stroke(self.bg_color, 2);
        bounding_box.into_styled(style).draw(target).unwrap();

        text.draw(target).unwrap();
    }
    pub fn new(x: i32, y: i32, color: Color, bg_color: Color, text: &str) -> Self {
        Self {
            x,
            y,
            color,
            bg_color,
            text: String::from(text),
        }
    }
}
