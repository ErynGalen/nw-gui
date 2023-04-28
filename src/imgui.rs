use crate::calculator::{Color, DeviceDisplay, Event};

use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle, StrokeAlignment},
    text::{Baseline, Text, TextStyleBuilder},
};

use heapless::{spsc::Queue, String, Vec};

pub use embedded_graphics::mono_font::ascii::FONT_7X13 as NORMAL_FONT;

#[derive(Debug)]
pub struct Imgui {
    bounding_box: Rectangle,
    actions: Queue<Action, 8>,
    available_bounding_box: Rectangle,
    widgets: Vec<Widget, 32>,
}

impl Imgui {
    pub fn new(bounding_box: Rectangle) -> Self {
        Self {
            bounding_box: bounding_box,
            actions: Queue::new(),
            available_bounding_box: bounding_box,
            widgets: Vec::new(),
        }
    }
    pub fn new_frame(&mut self) {
        self.widgets = Vec::new();
        self.available_bounding_box = self.bounding_box;
    }

    pub fn render(&self, target: &mut DeviceDisplay) {
        for w in &self.widgets {
            w.render(target);
        }
    }

    /// Returns the event back if it can't be added
    pub fn add_action(&mut self, action: Action) -> Result<(), Action> {
        self.actions.enqueue(action)
    }

    /// Add a button, always centered horizontally
    pub fn button(&mut self, label: &str, align: VerticalAlignment, focused: bool) {
        let character_style = MonoTextStyle::new(&NORMAL_FONT, Color::WHITE);
        let text = Text::new(label, Point::new(0, 0), character_style);

        let text_size = text.bounding_box().size;
        let external_size = Size::new(text_size.width + 4, text_size.height + 4);

        let mut text_bounding_box = Rectangle::new(
            Point::new(
                self.available_bounding_box.top_left.x
                    + (self.available_bounding_box.size.width as i32 - external_size.width as i32) / 2,
                self.available_bounding_box.top_left.y,
            ),
            text_size,
        );
        match align {
            VerticalAlignment::Top => (),
            VerticalAlignment::Center => {
                text_bounding_box.top_left.y = self.available_bounding_box.top_left.y
                    + (self.available_bounding_box.size.height as i32 - external_size.height as i32)
                        / 2;
            }
            VerticalAlignment::Bottom => {
                text_bounding_box.top_left.y = self.available_bounding_box.top_left.y
                    + self.available_bounding_box.size.height as i32
                    - external_size.height as i32;
            }
        };

        match align {
            VerticalAlignment::Bottom | VerticalAlignment::Top => {
                self.consume_height(align, external_size.height).unwrap()
            }
            VerticalAlignment::Center => todo!(),
        }
        text_bounding_box.top_left.y += 2;

        self.widgets
            .push(Widget::Button(Button {
                focused,
                bounding_box: text_bounding_box,
                text: String::from(label),
            }))
            .unwrap();
    }

    /// remove height from available bounding_box, at top or bottom only
    fn consume_height(&mut self, align: VerticalAlignment, height: u32) -> Result<(), ()> {
        let height = if height > self.available_bounding_box.size.height {
            self.available_bounding_box.size.height
        } else {
            height
        };
        match align {
            VerticalAlignment::Bottom => {
                self.available_bounding_box.size.height -= height;
                Ok(())
            }
            VerticalAlignment::Center => Err(()),
            VerticalAlignment::Top => {
                self.available_bounding_box.size.height -= height;
                self.available_bounding_box.top_left.y += height as i32;
                Ok(())
            }
        }
    }
}

pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}
#[derive(Debug)]
pub enum Action {
    Back,
    Ok,
}

// Private structs
#[derive(Debug)]
enum Widget {
    Button(Button),
}

impl Widget {
    pub fn render(&self, target: &mut DeviceDisplay) {
        match self {
            Widget::Button(w) => w.render(target),
        }
    }
}

#[derive(Debug)]
struct Button {
    focused: bool,
    bounding_box: Rectangle,
    text: String<16>,
}

impl Button {
    fn render(&self, target: &mut DeviceDisplay) {
        let mut bg_style_builder = PrimitiveStyleBuilder::new()
            .stroke_width(1)
            .fill_color(Color::CSS_DARK_GRAY);
        if self.focused {
            bg_style_builder = bg_style_builder.stroke_color(Color::CSS_BLUE_VIOLET);
        } else {
            bg_style_builder = bg_style_builder.stroke_color(Color::CSS_LIGHT_GRAY);
        }

        let bg_style = bg_style_builder.build();
        self.bounding_box
            .into_styled(bg_style)
            .draw(target)
            .unwrap();

        let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();
        let character_style = MonoTextStyle::new(&NORMAL_FONT, Color::CSS_DARK_SLATE_GRAY); // TODO: this is duplicated from Imgui::button
        let text = Text::with_text_style(
            &self.text,
            self.bounding_box.top_left,
            character_style,
            text_style,
        );
        text.draw(target).unwrap();
    }
}
