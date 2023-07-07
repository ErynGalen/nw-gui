//! Themes for the GUI.
//!
//! A theme contains informations about how to render the widgets.

use embedded_graphics::prelude::*;

use crate::calculator::Color;

/// This struct holds all the informations about a specific theme.
#[derive(Debug, Clone)]
pub struct Theme {
    /// The width of the rectangle borders.
    pub rect_border: u32,

    /// The color of foreground elements, e.g. text or borders.
    pub foreground: Color,
    /// The foreground color of the widget currently focused.
    pub focused: Color,
    /// The foreground color of an 'active' widget.
    ///
    /// A widget is active when being interacted with, e.g. a button being pressed.
    pub active: Color,

    pub background: Color,
}
impl Default for Theme {
    /// Default theme.
    fn default() -> Self {
        Self {
            rect_border: 2,
            foreground: Color::CSS_WHITE_SMOKE,
            focused: Color::CSS_NAVAJO_WHITE,
            active: Color::CSS_SKY_BLUE,
            background: Color::CSS_DARK_GRAY,
        }
    }
}
