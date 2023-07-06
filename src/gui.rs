//! This module contains types and traits used to create a GUI.

use embedded_graphics::primitives::Rectangle;

use either::Either;

use crate::calculator::{Color, DeviceDislay, Event};

pub mod storage;
pub mod widgets;

/// This font should be used to render text.
pub use embedded_graphics::mono_font::ascii::FONT_7X13 as NORMAL_FONT;

/// The GUI is made of objects implementing [`Widget`].
pub trait Widget {
    /// Type of the data passed to [`on_event()`](Widget::on_event()). It can be used to store a state shared between widgets.
    type Context;

    /// The `render()` method draws the widget onto the given target.
    fn render(&self, target: &mut DeviceDislay);
    /// The `on_event()` method dispatches the given event to the widget,
    /// which may pass the event to its children.
    ///
    /// Return the event back if it hasn't been used.
    fn on_event(&mut self, e: Event, context: &mut Self::Context) -> Option<Event>;

    /// Get the bounding box of a widget.
    fn get_bounding_box(&self) -> Rectangle;
    /// Set the bounding box of a widget.
    fn set_bounding_box(&mut self, bounding_box: Rectangle);

    /// Focused state of the widget.
    ///
    /// `None` if the widget is unfocusable,
    /// `Some(true)` if the widget is focused, `Some(false)` if the widget isn't focused but can be focused.
    fn get_focus(&self) -> Option<bool>;

    /// Focus the widget.
    ///
    /// If `from_dir` is `None` then unfocus the widget.
    ///
    /// If `from_dir` is `Some()`, then it is the direction where the last focus was.
    /// For example, moving from the right of the screen to the left of the screen means `from_dir = Some(FocusFrom::Right)`.
    ///
    /// Return `Ok` on success, and `Err` if the widget cannot be focused *temporarily*.
    /// `Err` doesn't indicate that the widget can never be focused.
    fn set_focus(&mut self, from_dir: Option<FocusFrom>) -> Result<(), ()>;
}

/// A Vec of [`Either`](either::Either) can be used as a widget collection to store multiple widget types.
impl<T, L: Widget<Context = T>, R: Widget<Context = T>> Widget for Either<L, R> {
    type Context = T;

    fn on_event(
        &mut self,
        e: crate::calculator::Event,
        context: &mut Self::Context,
    ) -> Option<crate::calculator::Event> {
        either::for_both!(self, w => w.on_event(e, context))
    }
    fn render(&self, target: &mut crate::calculator::DeviceDislay) {
        either::for_both!(self, w => w.render(target))
    }
    fn set_bounding_box(&mut self, bounding_box: embedded_graphics::primitives::Rectangle) {
        either::for_both!(self, w => w.set_bounding_box(bounding_box))
    }
    fn get_bounding_box(&self) -> Rectangle {
        either::for_both!(self, w => w.get_bounding_box())
    }
    fn get_focus(&self) -> Option<bool> {
        either::for_both!(self, w => w.get_focus())
    }
    fn set_focus(&mut self, from_dir: Option<FocusFrom>) -> Result<(), ()> {
        either::for_both!(self, w => w.set_focus(from_dir))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FocusFrom {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub struct Callback<T>(fn(&mut T));
