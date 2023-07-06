use crate::calculator::{Event, KeyCode};
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
                       //TODO: add margin/padding
}
impl<S, T: Widget<Context = S>, U: Widget<Context = S>> Widget for SplitLayout<T, U> {
    type Context = S;

    fn get_bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        self.bounding_box
    }
    fn set_bounding_box(&mut self, bounding_box: Rectangle) {
        self.bounding_box = bounding_box;
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
                    match self.focus_child(next_focus, from_dir) {
                        Ok(()) => None,
                        Err(()) => {
                            if should_attempt_other_side {
                                let other_side = match next_focus {
                                    Side::First => Side::Second,
                                    Side::Second => Side::First,
                                };
                                match self.focus_child(other_side, from_dir) {
                                    Ok(()) => None,
                                    Err(()) => Some(Event::KeyDown(key)),
                                }
                            } else {
                                Some(Event::KeyDown(key))
                            }
                        }
                    }
                } else {
                    Some(Event::KeyDown(key))
                }
            }
            Some(ev) => Some(ev),
        }
    }
    fn render(&self, target: &mut crate::calculator::DeviceDislay) {
        match self.focused {
            Some(Side::First) => {
                if let Some(ref second) = self.second {
                    second.render(target);
                }
                if let Some(ref first) = self.first {
                    first.render(target);
                }
            }
            Some(Side::Second) | None => {
                if let Some(ref first) = self.first {
                    first.render(target);
                }
                if let Some(ref second) = self.second {
                    second.render(target);
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
                self.focus_child(focus_side, from_dir)?
            }
            (None, SplitDirection::Vertical) => {
                let focus_side = match from_dir {
                    FocusFrom::Down => Side::Second,
                    _ => Side::First,
                };
                self.focus_child(focus_side, from_dir)?
            }
            (Some(side), _) => self.focus_child(side, from_dir)?, // if we were already focused, keep that focus
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
    /// `split_factor` is clampted between 0.0 and 1.0.
    pub fn new(bounding_box: Rectangle, split_direction: SplitDirection, split_factor: f32) -> Self {
        Self {
            bounding_box,
            direction: split_direction,
            first: None,
            second: None,
            focused: None,
            split_factor: split_factor.clamp(0.0, 1.0),
        }
    }

    /// Attach the first widget to the `SplitLayout`.
    ///
    /// The bounding box of the widget is automatically set to fit according to the parametres of the split.
    pub fn attach_first(&mut self, mut widget: T) {
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
        widget.set_bounding_box(Rectangle::new(self.bounding_box.top_left, size));
        self.first = Some(widget);
    }
    /// Attach the second widget to the `SplitLayout`.
    ///
    /// The bounding box of the widget is automatically set to fit according to the parametres of the split.
    pub fn attach_second(&mut self, mut widget: U) {
        let other_size = match self.direction {
            SplitDirection::Horizontal => {
                Size::new((self.bounding_box.size.width as f32 * self.split_factor) as u32, 0)
            }
            SplitDirection::Vertical => Size::new(0, (self.bounding_box.size.height as f32 * self.split_factor) as u32),
        };
        let top_left = self.bounding_box.top_left + other_size;

        widget.set_bounding_box(Rectangle::new(top_left, self.bounding_box.size - other_size));
        self.second = Some(widget);
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

    fn focus_child(&mut self, side: Side, from_dir: FocusFrom) -> Result<(), ()> {
        match side {
            Side::First => {
                if let Some(ref mut first) = self.first {
                    first.set_focus(Some(from_dir))?;
                    self.focused = Some(Side::First);
                } else {
                    return Err(());
                }
                if let Some(ref mut second) = self.second {
                    second.set_focus(None).unwrap_or_default();
                }
            }
            Side::Second => {
                if let Some(ref mut second) = self.second {
                    second.set_focus(Some(from_dir))?;
                    self.focused = Some(Side::Second);
                } else {
                    return Err(());
                }
                if let Some(ref mut first) = self.first {
                    first.set_focus(None).unwrap_or_default();
                }
            }
        }
        Ok(())
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
