use crate::calculator::{Event, KeyCode};
use crate::gui::theme::Theme;
use crate::gui::{FocusFrom, Widget};

use embedded_graphics::{prelude::*, primitives::Rectangle};

/// Split the layout in two, horizontally or vertically.
#[derive(Debug, Clone, Copy)]
pub struct SplitLayout<T: Widget, U: Widget> {
    direction: SplitDirection,
    first: Option<T>,
    second: Option<U>,
    focused: Option<Side>,
    bounding_box: Rectangle,
    split_factor: f32, // 0 = the second widget has all the bounding box, 1 = the first widget has all the bounding box
    first_margins: (u32, u32), // horizontal, vertical
    second_margins: (u32, u32),
}
impl<S, T: Widget<Context = S>, U: Widget<Context = S>> Widget for SplitLayout<T, U> {
    type Context = S;

    fn get_bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        self.bounding_box
    }
    fn set_bounding_box(&mut self, bounding_box: Rectangle) {
        self.bounding_box = bounding_box;
        self.set_children_bounding_box();
    }
    fn on_event(&mut self, e: Event, context: &mut Self::Context) -> Option<Event> {
        let remaining_event = match self.focused {
            Some(Side::First) => {
                if let Some(ref mut w) = self.first {
                    w.on_event(e, context)
                } else {
                    Some(e)
                }
            }
            Some(Side::Second) => {
                if let Some(ref mut w) = self.second {
                    w.on_event(e, context)
                } else {
                    Some(e)
                }
            }
            None => Some(e),
        };
        match remaining_event {
            None => None,
            Some(Event::KeyDown(key)) => {
                let mut to_focus: Option<(Side, FocusFrom)> = None;
                match (key, self.direction, self.focused) {
                    (KeyCode::Left, SplitDirection::Horizontal, Some(Side::Second) | None) => {
                        to_focus = Some((Side::First, FocusFrom::Right))
                    }
                    (KeyCode::Right, SplitDirection::Horizontal, Some(Side::First) | None) => {
                        to_focus = Some((Side::Second, FocusFrom::Left))
                    }
                    (KeyCode::Up, SplitDirection::Vertical, Some(Side::Second) | None) => {
                        to_focus = Some((Side::First, FocusFrom::Down))
                    }
                    (KeyCode::Down, SplitDirection::Vertical, Some(Side::First) | None) => {
                        to_focus = Some((Side::Second, FocusFrom::Up))
                    }
                    _ => (), // pass event to the parent
                };
                if let Some((next_focus, from_dir)) = to_focus {
                    let should_attempt_other_side = self.focused.is_none();
                    match self.focus_child(next_focus, from_dir, should_attempt_other_side) {
                        Ok(()) => None,
                        Err(()) => Some(Event::KeyDown(key)),
                    }
                } else {
                    Some(Event::KeyDown(key))
                }
            }
            Some(ev) => Some(ev),
        }
    }
    fn render(&self, target: &mut crate::calculator::DeviceDislay, theme: &Theme) {
        match self.focused {
            Some(Side::First) => {
                if let Some(ref second) = self.second {
                    second.render(target, theme);
                }
                if let Some(ref first) = self.first {
                    first.render(target, theme);
                }
            }
            Some(Side::Second) | None => {
                if let Some(ref first) = self.first {
                    first.render(target, theme);
                }
                if let Some(ref second) = self.second {
                    second.render(target, theme);
                }
            }
        }
    }
    fn get_focus(&self) -> Option<bool> {
        Some(self.focused.is_some())
    }
    fn set_focus(&mut self, from_dir: Option<FocusFrom>) -> Result<(), ()> {
        if from_dir.is_none() {
            self.focused = None;
            let mut is_ok = false; // Ok(()) as long as one of the side is Ok
            if let Some(ref mut first) = self.first {
                match first.set_focus(None) {
                    Ok(()) => is_ok = true,
                    Err(()) => (),
                }
            }
            if let Some(ref mut second) = self.second {
                match second.set_focus(None) {
                    Ok(()) => is_ok = true,
                    Err(()) => (),
                }
            }
            if is_ok {
                return Ok(());
            } else {
                return Err(());
            }
        }
        let from_dir = from_dir.unwrap(); // we already checked that `from_dir` is not `None`
        match (self.focused, self.direction) {
            (None, SplitDirection::Horizontal) => {
                let focus_side = match from_dir {
                    FocusFrom::Right => Side::Second,
                    _ => Side::First,
                };
                self.focus_child(focus_side, from_dir, true)?
            }
            (None, SplitDirection::Vertical) => {
                let focus_side = match from_dir {
                    FocusFrom::Down => Side::Second,
                    _ => Side::First,
                };
                self.focus_child(focus_side, from_dir, true)?
            }
            (Some(side), _) => self.focus_child(side, from_dir, true)?, // if we were already focused, keep that focus
        };
        Ok(())
    }
}
impl<T: Widget, U: Widget> SplitLayout<T, U> {
    /// Create a new `SplitLayout`.
    ///
    /// The given bounding box is split in two according to the split direction and split factor.
    /// A small split factor means that the first widget is smaller,
    /// and a large split factor means that the second widget is smaller.
    ///
    /// `split_factor` is clamped between 0 and 1.
    pub fn new(bounding_box: Rectangle, split_direction: SplitDirection, split_factor: f32) -> Self {
        Self {
            bounding_box,
            direction: split_direction,
            first: None,
            second: None,
            focused: None,
            split_factor: split_factor.clamp(0.0, 1.0),
            first_margins: (0, 0),
            second_margins: (0, 0),
        }
    }

    /// Attach the first widget to the `SplitLayout`.
    ///
    /// `margins` are (horizontal, vertical). The margins are applied at each side of the widget.
    ///
    /// The bounding box of the widget is automatically set to fit according to the parametres of the split.
    pub fn attach_first(&mut self, widget: T, margins: (u32, u32)) {
        self.first = Some(widget);
        self.first_margins = margins;
        self.set_children_bounding_box();
    }
    /// Attach the second widget to the `SplitLayout`.
    ///
    /// `margins` are (horizontal, vertical). The margins are applied at each side of the widget.
    ///
    /// The bounding box of the widget is automatically set to fit according to the parametres of the split.
    pub fn attach_second(&mut self, widget: U, margins: (u32, u32)) {
        self.second = Some(widget);
        self.second_margins = margins;
        self.set_children_bounding_box();
    }

    /// Read access to the first widget.
    pub fn get_first(&self) -> Option<&T> {
        self.first.as_ref()
    }
    /// Read access to the second widget.
    pub fn get_second(&self) -> Option<&U> {
        self.second.as_ref()
    }
    /// Mutable access to the first widget.
    pub fn get_first_mut(&mut self) -> Option<&mut T> {
        self.first.as_mut()
    }
    /// Mutable access to the second widget.
    pub fn get_second_mut(&mut self) -> Option<&mut U> {
        self.second.as_mut()
    }

    fn focus_child(&mut self, side: Side, from_dir: FocusFrom, attempt_other_side: bool) -> Result<(), ()> {
        match side {
            Side::First => {
                if let Some(ref mut first) = self.first {
                    match first.set_focus(Some(from_dir)) {
                        Ok(()) => self.focused = Some(Side::First),
                        Err(()) => {
                            if attempt_other_side {
                                return self.focus_child(side.other(), from_dir, false);
                            }
                            return Err(());
                        }
                    };
                } else {
                    if attempt_other_side {
                        return self.focus_child(side.other(), from_dir, false);
                    }
                    return Err(());
                }
                if let Some(ref mut second) = self.second {
                    second.set_focus(None).unwrap_or_default();
                }
            }
            Side::Second => {
                if let Some(ref mut second) = self.second {
                    match second.set_focus(Some(from_dir)) {
                        Ok(()) => self.focused = Some(Side::Second),
                        Err(()) => {
                            if attempt_other_side {
                                return self.focus_child(side.other(), from_dir, false);
                            }
                            return Err(());
                        }
                    }
                } else {
                    if attempt_other_side {
                        return self.focus_child(side.other(), from_dir, false);
                    }
                    return Err(());
                }
                if let Some(ref mut first) = self.first {
                    first.set_focus(None).unwrap_or_default();
                }
            }
        }
        Ok(())
    }

    fn set_children_bounding_box(&mut self) {
        if let Some(ref mut first) = self.first {
            let size = match self.direction {
                SplitDirection::Horizontal => Size::new(
                    (self.bounding_box.size.width as f32 * self.split_factor) as u32,
                    self.bounding_box.size.height,
                ),
                SplitDirection::Vertical => Size::new(
                    self.bounding_box.size.width,
                    (self.bounding_box.size.height as f32 * self.split_factor) as u32,
                ),
            };
            let mut margins = Size::new(self.first_margins.0, self.first_margins.1);
            if margins.width * 2 > size.width {
                margins.width = size.width / 2;
            }
            if margins.height * 2 > size.height {
                margins.height = size.height / 2;
            }
            first.set_bounding_box(Rectangle::new(self.bounding_box.top_left + margins, size - margins * 2));
        }
        if let Some(ref mut second) = self.second {
            let other_size = match self.direction {
                SplitDirection::Horizontal => {
                    Size::new((self.bounding_box.size.width as f32 * self.split_factor) as u32, 0)
                }
                SplitDirection::Vertical => {
                    Size::new(0, (self.bounding_box.size.height as f32 * self.split_factor) as u32)
                }
            };
            let top_left = self.bounding_box.top_left + other_size;
            let size = self.bounding_box.size - other_size;
            let mut margins = Size::new(self.second_margins.0, self.second_margins.1);
            if margins.width * 2 > size.width {
                margins.width = size.width / 2;
            }
            if margins.height * 2 > size.height {
                margins.height = size.height / 2;
            }
            second.set_bounding_box(Rectangle::new(top_left + margins, size - margins * 2));
        }
    }
}

/// Direction in which a [`SplitLayout`] is split.
#[derive(Debug, Clone, Copy)]
pub enum SplitDirection {
    /// Left | Right
    Horizontal,
    /// Up / Down
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Side {
    First,
    Second,
}
impl Side {
    fn other(self) -> Self {
        match self {
            Self::First => Self::Second,
            Self::Second => Self::First,
        }
    }
}
