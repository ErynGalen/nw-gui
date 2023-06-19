//! Default widgets available to build a GUI.

use crate::calculator::{Color, DeviceDislay, Event, KeyCode};
use crate::gui::{storage::WidgetCollection, Widget};
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};

#[derive(Debug)]
pub struct RectWidget {
    pub bounging_box: Rectangle,
    pub fill_color: Color,
    pub border_color: Color,
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
        let mut focus_offset: (isize, isize) = (0, 0);
        match e {
            Event::KeyDown(KeyCode::Left) => focus_offset.0 = -1,
            Event::KeyDown(KeyCode::Right) => focus_offset.0 = 1,
            Event::KeyDown(KeyCode::Up) => focus_offset.1 = -1,
            Event::KeyDown(KeyCode::Down) => focus_offset.1 = 1,
            _ => {
                let selected_child = self.grid[self.selected.0][self.selected.1].unwrap();
                if let Some(selected_child) = self.children.get_mut(selected_child) {
                    selected_child.on_event(e);
                }
            }
        }
        if focus_offset.0 != 0 || focus_offset.1 != 0 {
            let mut new_selected = (self.selected.0 as isize, self.selected.1 as isize);
            loop {
                new_selected = (
                    new_selected.0 + focus_offset.0,
                    new_selected.1 + focus_offset.1,
                );
                if new_selected.0 < 0
                    || new_selected.0 >= X as isize
                    || new_selected.1 < 0
                    || new_selected.1 >= Y as isize
                {
                    break;
                }
                if self.grid[new_selected.0 as usize][new_selected.1 as usize].is_some() {
                    self.selected = (new_selected.0 as usize, new_selected.1 as usize);
                    break;
                }
            }
        }
    }
}
