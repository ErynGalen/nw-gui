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
    fn render(&self, target: &mut DeviceDislay, focused: bool) {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(self.fill_color)
            .stroke_color(if focused {
                self.border_color
            } else {
                Color::CSS_GRAY
            })
            .stroke_width(self.border_width)
            .build();
        self.bounging_box.into_styled(style).draw(target).unwrap();
    }
    fn set_bounding_box(&mut self, bounding_box: Rectangle) {
        self.bounging_box = bounding_box;
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
    /// Add a child widget to the grid.
    ///
    /// `position` is the top-left cell of the requested position in grid.
    /// `size` is the number of cells (horizontally, vertically) requested.
    ///
    /// The child is returned back if it can't be added.
    pub fn add_child_at(
        &mut self,
        mut child: C::Item,
        position: (usize, usize),
        size: (usize, usize),
    ) -> Result<(), C::Item> {
        let cell_size = (
            self.bounding_box.size.width / X as u32,
            self.bounding_box.size.height / Y as u32,
        );
        if position.0 >= X || position.1 >= Y || position.0 + size.0 > X || position.1 + size.1 > Y
        {
            return Err(child);
        }

        let bb = Rectangle::new(
            Point::new(
                position.0 as i32 * cell_size.0 as i32,
                position.1 as i32 * cell_size.1 as i32,
            ) + self.bounding_box.top_left,
            Size::new(size.0 as u32 * cell_size.0, size.1 as u32 * cell_size.1),
        );
        child.set_bounding_box(bb);

        self.children.add_widget(child)?;

        // reserve cells in the grid
        let child_n = self.children.len() - 1;
        for x_offset in 0..size.0 {
            for y_offset in 0..size.1 {
                // safe because we already checked that the requested position and size are contained in the grid
                self.grid[x_offset + position.0][y_offset + position.1] = Some(child_n);
            }
        }

        Ok(())
    }
}
impl<'a, const X: usize, const Y: usize, C: WidgetCollection> Widget for Grid<X, Y, C> {
    fn render(&self, target: &mut DeviceDislay, focused: bool) {
        for n in 0..self.children.len() {
            let child_focused = self.grid[self.selected.0][self.selected.1]
                .and_then(|selected_child| Some(n == selected_child))
                .unwrap_or(false);
            self.children
                .get(n)
                .unwrap()
                .render(target, child_focused && focused);
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
                if let Some(selected_child) = self.grid[self.selected.0][self.selected.1] {
                    if let Some(selected_child) = self.children.get_mut(selected_child) {
                        selected_child.on_event(e);
                    }
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
    fn set_bounding_box(&mut self, bounding_box: Rectangle) {
        self.bounding_box = bounding_box;
    }
}
