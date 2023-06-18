use heapless::String;

use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::Text,
};

use crate::calculator::{Color, DeviceDislay, Event, KeyCode};

pub mod widgets;

pub use embedded_graphics::mono_font::ascii::FONT_7X13 as NORMAL_FONT;

pub trait WidgetCollection {
    type Item: Widget;

    fn len(&self) -> usize;
    fn get(&self, n: usize) -> Option<&Self::Item>;
    fn get_mut(&mut self, n: usize) -> Option<&mut Self::Item>;
}

pub trait Widget {
    fn render(&self, target: &mut DeviceDislay);
    fn on_event(&mut self, e: Event);
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
impl Widget for RectWidget {
    fn on_event(&mut self, _e: Event) {}
    fn render(&self, target: &mut DeviceDislay) {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(self.fill_color)
            .stroke_color(self.border_color)
            .stroke_width(self.border_width)
            .build();
        self.bounging_box.into_styled(style).draw(target).unwrap();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Grid<const X: usize, const Y: usize, C: WidgetCollection> {
    bounding_box: Rectangle,
    grid: [[Option<usize>; Y]; X],
    selected: (usize, usize),
    children: C,
}
impl<const X: usize, const Y: usize, C: WidgetCollection> Grid<X, Y, C> {
    pub fn new(bounding_box: Rectangle, children: C) -> Self {
        Self {
            bounding_box,
            grid: [[None; Y]; X],
            selected: (0, 0),
            children,
        }
    }
}
impl<'a, const X: usize, const Y: usize, C: WidgetCollection> Widget for Grid<X, Y, C> {
    fn render(&self, target: &mut DeviceDislay) {
        for n in 0..self.children.len() {
            self.children.get(n).unwrap().render(target);
        }
    }
    fn on_event(&mut self, e: Event) {
        match e {
            Event::KeyDown(key) => {
                let mut new_selected = self.selected;
                match key {
                    KeyCode::Left => {
                        if new_selected.0 > 0 {
                            new_selected.0 -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if new_selected.0 < X {
                            new_selected.0 += 1;
                        }
                    }
                    KeyCode::Up => {
                        if new_selected.1 > 0 {
                            new_selected.1 -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if new_selected.1 < Y {
                            new_selected.1 += 1;
                        }
                    }
                    _ => {
                        let selected_child = self.grid[self.selected.0][self.selected.1].unwrap();
                        if let Some(selected_child) = self.children.get_mut(selected_child) {
                            selected_child.on_event(e);
                        }
                    }
                }
                if self.grid[new_selected.0][new_selected.1].is_some() {
                    self.selected = new_selected;
                }
            }
            _ => {
                let selected_child = self.grid[self.selected.0][self.selected.1].unwrap();
                if let Some(selected_child) = self.children.get_mut(selected_child) {
                    selected_child.on_event(e);
                }
            }
        }
    }
}
