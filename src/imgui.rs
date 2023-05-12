use crate::calculator::{Color, DeviceDisplay, Event, Keycode};

use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::{Baseline, Text, TextStyleBuilder},
};

use heapless::{spsc::Queue, String, Vec};

pub use embedded_graphics::mono_font::ascii::FONT_7X13 as NORMAL_FONT;

#[derive(Debug)]
pub struct Imgui {
    bounding_box: Rectangle,
    actions: Queue<Action, 8>,
    available_bounding_box: Rectangle,
    widgets: Vec<(Widget, FocusGrid), 32>,
    focused: usize,
}

impl Imgui {
    pub fn new(bounding_box: Rectangle) -> Self {
        Self {
            bounding_box,
            actions: Queue::new(),
            available_bounding_box: bounding_box,
            widgets: Vec::new(),
            focused: 0,
        }
    }
    pub fn new_frame(&mut self) {
        self.widgets = Vec::new();
        self.available_bounding_box = self.bounding_box;
    }

    pub fn render(&self, target: &mut DeviceDisplay) {
        let mut index = 0;
        for w in &self.widgets {
            let is_focused = if index == self.focused { true } else { false };
            w.0.render(target, is_focused);
            index += 1;
        }
    }

    pub fn on_event(&mut self, e: Event) {
        let mut next_event = Some(e);
        for w in &mut self.widgets {
            match next_event {
                None => (),
                Some(event) => next_event = w.0.on_event(event),
            }
        }
        match next_event {
            None => (),
            Some(event) => match event {
                Event::KeyDown(k) => match k {
                    Keycode::Right => {
                        if let Some(to_focus) = self.widgets.get(self.focused).unwrap().1.right {
                            self.focused = to_focus;
                        }
                    }
                    Keycode::Left => {
                        if let Some(to_focus) = self.widgets.get(self.focused).unwrap().1.left {
                            self.focused = to_focus;
                        }
                    }
                    Keycode::Up => {
                        if let Some(to_focus) = self.widgets.get(self.focused).unwrap().1.up {
                            self.focused = to_focus;
                        }
                    }
                    Keycode::Down => {
                        if let Some(to_focus) = self.widgets.get(self.focused).unwrap().1.down {
                            self.focused = to_focus;
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
        }
    }

    /// links two widget's focus
    pub fn focus_up_down(&mut self, up: usize, down: usize) {
        self.widgets.get_mut(up).unwrap().1.down = Some(down);
        self.widgets.get_mut(down).unwrap().1.up = Some(up);
    }
    /// links two widget's focus
    pub fn focus_left_right(&mut self, left: usize, right: usize) {
        self.widgets.get_mut(left).unwrap().1.right = Some(right);
        self.widgets.get_mut(right).unwrap().1.left = Some(left);
    }

    /// Returns the action back if it can't be added
    pub fn add_action(&mut self, action: Action) -> Result<(), Action> {
        self.actions.enqueue(action)
    }

    /// Add a button, always centered horizontally
    pub fn button(&mut self, label: &str, align: VerticalAlignment) -> usize {
        let character_style = MonoTextStyle::new(&NORMAL_FONT, Color::WHITE);
        let text = Text::new(label, Point::new(0, 0), character_style);

        let text_size = text.bounding_box().size;
        let external_size = Size::new(
            text_size.width + Button::MARGIN * 2 + Button::PADDING * 2,
            text_size.height + Button::MARGIN * 2 + Button::PADDING * 2,
        );

        let mut internal_bounding_box = Rectangle::new(
            Point::new(
                self.available_bounding_box.top_left.x
                    + (self.available_bounding_box.size.width as i32 - external_size.width as i32)
                        / 2,
                self.available_bounding_box.top_left.y,
            ),
            Size::new(
                text_size.width + Button::PADDING * 2,
                text_size.height + Button::PADDING * 2,
            ),
        );
        match align {
            VerticalAlignment::Top => (),
            VerticalAlignment::Center => {
                internal_bounding_box.top_left.y = self.available_bounding_box.top_left.y
                    + (self.available_bounding_box.size.height as i32
                        - external_size.height as i32)
                        / 2;
            }
            VerticalAlignment::Bottom => {
                internal_bounding_box.top_left.y = self.available_bounding_box.top_left.y
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
        internal_bounding_box.top_left.y += Button::MARGIN as i32;

        self.widgets
            .push((
                Widget::Button(Button {
                    bounding_box: internal_bounding_box,
                    text: String::from(label),
                }),
                FocusGrid::default(),
            ))
            .unwrap();
        return self.widgets.len() - 1;
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

#[derive(Debug)]
enum Widget {
    Button(Button),
}

impl Widget {
    pub fn render(&self, target: &mut DeviceDisplay, is_focused: bool) {
        match self {
            Widget::Button(w) => w.render(target, is_focused),
        }
    }
    /// returns the event back if it hasn't been used
    pub fn on_event(&mut self, e: Event) -> Option<Event> {
        match self {
            Widget::Button(_) => Some(e),
        }
    }
}

// Private structs

#[derive(Debug)]
struct FocusGrid {
    right: Option<usize>,
    left: Option<usize>,
    down: Option<usize>,
    up: Option<usize>,
}
impl Default for FocusGrid {
    fn default() -> Self {
        Self {
            right: None,
            left: None,
            down: None,
            up: None,
        }
    }
}

// Private widget structs

#[derive(Debug)]
struct Button {
    bounding_box: Rectangle,
    text: String<16>,
}

impl Button {
    const MARGIN: u32 = 2;
    const PADDING: u32 = 2;

    fn render(&self, target: &mut DeviceDisplay, is_focused: bool) {
        let mut bg_style_builder = PrimitiveStyleBuilder::new()
            .stroke_width(1)
            .fill_color(Color::CSS_DARK_GRAY);
        if is_focused {
            bg_style_builder = bg_style_builder.stroke_color(Color::CSS_BLUE_VIOLET);
        } else {
            bg_style_builder = bg_style_builder.stroke_color(Color::CSS_LIGHT_GRAY);
        }

        let bg_style = bg_style_builder.build();
        self.bounding_box
            .into_styled(bg_style)
            .draw(target)
            .unwrap();
        let text_position = Point::new(
            self.bounding_box.top_left.x + Self::PADDING as i32,
            self.bounding_box.top_left.y + Self::PADDING as i32,
        );
        let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();
        let character_style = MonoTextStyle::new(&NORMAL_FONT, Color::CSS_DARK_SLATE_GRAY); // TODO: this is duplicated from Imgui::button
        let text = Text::with_text_style(&self.text, text_position, character_style, text_style);
        text.draw(target).unwrap();
    }
}
