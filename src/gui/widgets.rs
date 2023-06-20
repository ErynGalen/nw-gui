//! Default widgets available to build a GUI.

use crate::calculator::{Color, DeviceDislay, Event, KeyCode};
use crate::gui::{storage::WidgetCollection, Widget};
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};

/// A colored rectanble, with an outline of a different color.
#[derive(Debug)]
pub struct ColorRect {
    bounding_box: Rectangle,
    fill_color: Color,
    border_color: Color,
    border_width: u32,
}
impl ColorRect {
    pub fn new(
        fill_color: Color,
        border_color: Color,
        border_width: u32,
        bounding_box: Rectangle,
    ) -> Self {
        Self {
            fill_color,
            border_color,
            border_width,
            bounding_box,
        }
    }
}
impl Widget for ColorRect {
    fn on_event(&mut self, e: Event) -> Option<Event> {
        Some(e)
    }
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
        self.bounding_box.into_styled(style).draw(target).unwrap();
    }
    fn set_bounding_box(&mut self, bounding_box: Rectangle) {
        self.bounding_box = bounding_box;
    }
}

/// A widget grid.
/// The grid has `X` cells horizontally, and `Y` cells vertically.
///
/// Its children are stored in a collection of type `C`.
///
/// The bounding box of the children is calculated when they're added.
#[derive(Debug, Clone, Copy)]
pub struct Grid<const X: usize, const Y: usize, C: WidgetCollection> {
    bounding_box: Rectangle,
    grid: [[Option<usize>; Y]; X],
    selected: (usize, usize),
    children: C,
}
impl<const X: usize, const Y: usize, C: WidgetCollection> Grid<X, Y, C> {
    /// Create a new grid filling the specified bounding box.
    ///
    /// `children` is a the collection used to store the children of the grid.
    /// It should be empty.
    ///
    /// # Panics
    /// The function panics if `children` isn't empty, i.e. if `children.len() > 0`.
    pub fn new(bounding_box: Rectangle, children: C) -> Self {
        if children.len() > 0 {
            panic!("Can't create a grid from a non-empty collection.");
        }
        Self {
            bounding_box,
            grid: [[None; Y]; X],
            selected: (0, 0),
            children,
        }
    }
    /// Add a child widget to the grid.
    ///
    /// * `position` is the top-left cell of the requested position in grid.
    /// * `size` is the number of cells (horizontally, vertically) requested.
    /// * `padding` is the padding (in the CSS sense, both horizontally and vertically) in pixels.
    ///
    /// If the child is successfully added, then its id is returned,
    /// otherwise the child is returned back.
    ///
    /// # Example
    /// ```
    /// use embedded_graphics::{prelude::*, primitives::Rectangle};
    /// use nw_gui::gui::widgets::{ColorRect, Grid};
    /// use nw_gui::calculator::Color;
    /// use heapless::Vec;
    /// // This layout:
    /// // +----+----+----+----+----+
    /// // |    |    |    |  GREEN  |
    /// // +----+----+----+----+----+
    /// // |     BLUE     |    |    |
    /// // +----+----+----+----+----+
    /// // |    |     RED      |    |
    /// // +----+----+----+----+----+
    /// // can be contructed as follows:
    /// let mut grid: Grid<5, 3, Vec<ColorRect, 3>> = Grid::new(
    ///     Rectangle::new(Point::new(0, 0), Size::new(320, 240)),
    ///     Vec::new(),
    /// );
    /// grid.add_child_at(
    ///     ColorRect::new(Color::GREEN, Color::WHITE, 2, Rectangle::default()),
    ///     (0, 0),
    ///     (2, 1),
    ///     4,
    /// ).unwrap();
    /// grid.add_child_at(
    ///     ColorRect::new(Color::BLUE, Color::WHITE, 2, Rectangle::default()),
    ///     (0, 1),
    ///     (3, 1),
    ///     4,
    /// ).unwrap();
    /// grid.add_child_at(
    ///     ColorRect::new(Color::RED, Color::WHITE, 2, Rectangle::default()),
    ///     (1, 2),
    ///     (3, 1),
    ///     4,
    /// ).unwrap();
    /// ```
    pub fn add_child_at(
        &mut self,
        mut child: C::Item,
        position: (usize, usize),
        size: (usize, usize),
        padding: u32,
    ) -> Result<usize, C::Item> {
        let cell_size = (
            self.bounding_box.size.width / X as u32,
            self.bounding_box.size.height / Y as u32,
        );
        if position.0 >= X || position.1 >= Y || position.0 + size.0 > X || position.1 + size.1 > Y
        {
            return Err(child);
        }

        let mut bb = Rectangle::new(
            Point::new(
                position.0 as i32 * cell_size.0 as i32,
                position.1 as i32 * cell_size.1 as i32,
            ) + self.bounding_box.top_left,
            Size::new(size.0 as u32 * cell_size.0, size.1 as u32 * cell_size.1),
        );
        let horizontal_padding = padding.clamp(0, bb.size.width / 2);
        let vertical_padding = padding.clamp(0, bb.size.height / 2);
        bb.top_left.x += horizontal_padding as i32;
        bb.top_left.y += vertical_padding as i32;
        bb.size.width -= 2 * horizontal_padding;
        bb.size.height -= 2 * horizontal_padding;
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

        Ok(child_n)
    }

    /// Read-only acces to a child
    pub fn get(&self, n: usize) -> Option<&C::Item> {
        self.children.get(n)
    }
    /// Mutable acces to a child
    pub fn get_mut(&mut self, n: usize) -> Option<&mut C::Item> {
        self.children.get_mut(n)
    }
}
impl<'a, const X: usize, const Y: usize, C: WidgetCollection> Widget for Grid<X, Y, C> {
    fn render(&self, target: &mut DeviceDislay, focused: bool) {
        let mut selected_child: Option<usize> = None;
        for n in 0..self.children.len() {
            let child_focused = self.grid[self.selected.0][self.selected.1]
                .and_then(|selected_child| Some(n == selected_child))
                .unwrap_or(false);
            if child_focused {
                selected_child = Some(n);
            } else {
                self.children.get(n).unwrap().render(target, false);
            }
        }
        if let Some(selected_child) = selected_child {
            self.children
                .get(selected_child)
                .unwrap()
                .render(target, focused);
        }
    }
    fn on_event(&mut self, e: Event) -> Option<Event> {
        let mut remaining_event: Option<Event> = Some(e);
        if let Some(selected_index) = self.grid[self.selected.0][self.selected.1] {
            if let Some(selected_child) = self.children.get_mut(selected_index) {
                remaining_event = selected_child.on_event(remaining_event.unwrap());
            }
        }
        if let Some(e) = remaining_event {
            let mut focus_offset: (isize, isize) = (0, 0);
            match e {
                Event::KeyDown(KeyCode::Left) => focus_offset.0 = -1,
                Event::KeyDown(KeyCode::Right) => focus_offset.0 = 1,
                Event::KeyDown(KeyCode::Up) => focus_offset.1 = -1,
                Event::KeyDown(KeyCode::Down) => focus_offset.1 = 1,
                _ => return Some(e),
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
                        // if the user moves outside the grid, they may want to move out of the grid,
                        // so let the parent widget handle the event
                        return Some(e);
                    }
                    if self.grid[new_selected.0 as usize][new_selected.1 as usize].is_some() {
                        self.selected = (new_selected.0 as usize, new_selected.1 as usize);
                        return None;
                    }
                }
            }
        }
        None
    }
    fn set_bounding_box(&mut self, bounding_box: Rectangle) {
        // TODO: resize dynamically all the children
        self.bounding_box = bounding_box;
    }
}
