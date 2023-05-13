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
    next_event: Option<Event>,
}

impl Imgui {
    pub fn new(bounding_box: Rectangle) -> Self {
        Self {
            bounding_box,
            actions: Queue::new(),
            available_bounding_box: bounding_box,
            widgets: Vec::new(),
            focused: 0,
            next_event: None,
        }
    }
    pub fn new_frame(&mut self, reset_focus: bool) {
        self.widgets = Vec::new();
        self.available_bounding_box = self.bounding_box;
        if reset_focus {
            self.focused = 0;
        }
    }
    /// Triggers focus changes, and all events managed by Imgui directly
    pub fn end_frame(&mut self) {
        match self.next_event {
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
        self.next_event = None; // next_event was either used or not useful
    }

    pub fn render(&self, target: &mut DeviceDisplay) {
        target.clear(Color::CSS_DARK_GRAY).unwrap();
        let mut index = 0;
        for w in &self.widgets {
            let is_focused = if index == self.focused { true } else { false };
            w.0.render(target, is_focused);
            index += 1;
        }
    }

    /// A call to `on_event()` should be followed by the creation of a new frame,
    /// because it's on new frame creation that the event is processed.
    ///
    /// Returns `Err(e)` if another event is already there to be processed.
    pub fn on_event(&mut self, e: Event) -> Result<(), Event> {
        match self.next_event {
            None => self.next_event = Some(e),
            Some(_) => return Err(e),
        }
        Ok(())
    }
    fn do_event_for_last(&mut self) {
        if self.widgets.len() - 1 != self.focused {
            return;
        }
        let next_event = match &self.next_event {
            Some(e) => self.widgets.last_mut().unwrap().0.on_event(*e),
            None => return,
        };
        self.next_event = next_event;
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

    // create widgets

    /// Add a button, always centered horizontally
    pub fn button(&mut self, label: &str, align: VerticalAlignment) -> usize {
        let character_style = MonoTextStyle::new(&NORMAL_FONT, Color::WHITE);
        let text = Text::new(label, Point::new(0, 0), character_style);

        let text_size = text.bounding_box().size;
        let external_size = Size::new(
            text_size.width + Widget::MARGIN * 2 + Button::PADDING * 2,
            text_size.height + Widget::MARGIN * 2 + Button::PADDING * 2,
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
                self.take_height(align, external_size.height).unwrap();
            }
            VerticalAlignment::Center => todo!(),
        }
        internal_bounding_box.top_left.y += Widget::MARGIN as i32;

        self.widgets
            .push((
                Widget::Button(Button {
                    bounding_box: internal_bounding_box,
                    text: String::from(label),
                }),
                FocusGrid::default(),
            ))
            .unwrap();
        self.do_event_for_last();
        return self.widgets.len() - 1;
    }
    pub fn slider(
        &mut self,
        value: &mut i32,
        min: i32,
        max: i32,
        align: VerticalAlignment,
    ) -> usize {
        let bounding_box = match align {
            VerticalAlignment::Bottom | VerticalAlignment::Top => self
                .take_height_with_margins(align, Slider::HEIGHT + Widget::MARGIN * 2)
                .unwrap(),
            VerticalAlignment::Center => todo!(),
        };
        let (min, max) = if min < max { (min, max) } else { (max, min) };
        *value = (*value).clamp(min, max);
        self.widgets
            .push((
                Widget::Slider(Slider {
                    bounding_box,
                    value: *value,
                    min,
                    max,
                }),
                FocusGrid::default(),
            ))
            .unwrap();
        self.do_event_for_last();
        if let Widget::Slider(s) = &self.widgets.last().unwrap().0 {
            *value = s.value;
        } else {
            panic!("Last widget should've been a slider");
        }
        return self.widgets.len() - 1;
    }

    /// Remove height from available bounding_box, at top or bottom only,
    /// and returns the bounding box that has been removed
    fn take_height(&mut self, align: VerticalAlignment, height: u32) -> Result<Rectangle, ()> {
        let height = if height > self.available_bounding_box.size.height {
            self.available_bounding_box.size.height
        } else {
            height
        };
        match align {
            VerticalAlignment::Bottom => {
                let bb = Rectangle::new(
                    Point::new(
                        self.available_bounding_box.top_left.x,
                        self.available_bounding_box.bottom_right().unwrap().y - height as i32,
                    ),
                    Size::new(self.available_bounding_box.size.width, height),
                );
                self.available_bounding_box.size.height -= height;
                Ok(bb)
            }
            VerticalAlignment::Center => Err(()),
            VerticalAlignment::Top => {
                let bb = Rectangle::new(
                    self.available_bounding_box.top_left,
                    Size::new(self.available_bounding_box.size.width, height),
                );
                self.available_bounding_box.size.height -= height;
                self.available_bounding_box.top_left.y += height as i32;
                Ok(bb)
            }
        }
    }
    fn take_height_with_margins(
        &mut self,
        align: VerticalAlignment,
        height: u32,
    ) -> Result<Rectangle, ()> {
        let res = self.take_height(align, height + Widget::MARGIN * 2);
        match res {
            Err(_) => res,
            Ok(bb) => Ok(Rectangle::new(
                bb.top_left + Size::new_equal(Widget::MARGIN),
                bb.size - Size::new_equal(Widget::MARGIN) * 2,
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}
#[derive(Debug, Clone, Copy)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}
#[derive(Debug, Clone, Copy)]
pub enum Action {
    Back,
    Ok,
}

#[derive(Debug)]
enum Widget {
    Button(Button),
    Slider(Slider),
}

impl Widget {
    const MARGIN: u32 = 2;

    pub fn render(&self, target: &mut DeviceDisplay, is_focused: bool) {
        match self {
            Widget::Button(w) => w.render(target, is_focused),
            Widget::Slider(w) => w.render(target, is_focused),
        }
    }
    /// returns the event back if it hasn't been used
    pub fn on_event(&mut self, e: Event) -> Option<Event> {
        match self {
            Widget::Button(_) => Some(e),
            Widget::Slider(w) => w.on_event(e),
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

#[derive(Debug)]
struct Slider {
    bounding_box: Rectangle,
    value: i32,
    min: i32,
    max: i32,
}
impl Slider {
    const BAR_HEIGHT: u32 = 5;
    const HEIGHT: u32 = 15;

    fn render(&self, target: &mut DeviceDisplay, is_focused: bool) {
        let mut bg_style_builder = PrimitiveStyleBuilder::new()
            .stroke_width(1)
            .fill_color(Color::CSS_DARK_GRAY);
        if is_focused {
            bg_style_builder = bg_style_builder.stroke_color(Color::CSS_BLUE_VIOLET);
        } else {
            bg_style_builder = bg_style_builder.stroke_color(Color::CSS_LIGHT_GRAY);
        }
        self.bounding_box
            .into_styled(bg_style_builder.build())
            .draw(target)
            .unwrap();

        let center = self.bounding_box.top_left + self.bounding_box.size / 2;
        let bar_size = Size::new(
            self.bounding_box.size.width - (Self::HEIGHT - Self::BAR_HEIGHT),
            Self::BAR_HEIGHT,
        );
        let bar_bb = Rectangle::new(center - bar_size / 2, bar_size);
        let bar_style = PrimitiveStyleBuilder::new()
            .fill_color(Color::CSS_SKY_BLUE)
            .build();
        bar_bb.into_styled(bar_style).draw(target).unwrap();

        let marker_size = Size::new_equal(Self::BAR_HEIGHT);
        let x_offset = (self.value - self.min) * (bar_bb.size.width - marker_size.width) as i32
            / (self.max - self.min);
        let marker_bb = Rectangle::new(
            Point::new(bar_bb.top_left.x + x_offset, bar_bb.top_left.y),
            marker_size,
        );
        let marker_style = PrimitiveStyleBuilder::new()
            .fill_color(Color::CSS_INDIAN_RED)
            .build();
        marker_bb.into_styled(marker_style).draw(target).unwrap();
    }
    fn on_event(&mut self, e: Event) -> Option<Event> {
        match e {
            Event::KeyDown(k) => match k {
                Keycode::Right => self.value += 1,
                Keycode::Left => self.value -= 1,
                _ => return Some(e),
            },
            _ => return Some(e),
        }
        self.value = self.value.clamp(self.min, self.max);
        return None;
    }
}
