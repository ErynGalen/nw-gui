use heapless::Vec;

use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};

use crate::numworks_display::{Color, DeviceDislay};

#[derive(Debug)]
pub enum Widget {
    Square(SquareWidget),
}
impl Widget {
    pub fn render(&self, target: &mut DeviceDislay) {
        match self {
            Self::Square(w) => w.render(target),
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
pub struct SquareWidget {
    x: i32,
    y: i32,
    color: Color,
    //text: String<16>,
}
impl SquareWidget {
    pub fn render(&self, target: &mut DeviceDislay) {
        let rect = Rectangle::new(Point::new(self.x, self.y), Size::new(20, 20));
        let style = PrimitiveStyle::with_stroke(self.color, 2);
        rect.into_styled(style).draw(target).unwrap();
    }
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        Self { x, y, color }
    }
}
