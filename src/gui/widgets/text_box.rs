use core::marker::PhantomData;
use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::*,
    primitives::Rectangle,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use heapless::String;

use super::ColorRect;
use crate::gui::text::{Action, TextInput, TextInputContext};
use crate::{
    calculator::Event,
    gui::{Widget, NORMAL_FONT},
};

#[derive(Debug, Clone)]
struct EditionInfo {
    focused: bool,
    in_edition: bool, // whether the text is being edited
}

pub struct TextBox<T, const N: usize> {
    background: ColorRect<T>,
    pub value: String<N>,
    edition: Option<EditionInfo>,
    _context: PhantomData<T>,
}
impl<T, const N: usize> TextBox<T, N> {
    pub fn new(bounding_box: Rectangle, editable: bool) -> Self {
        Self {
            _context: PhantomData,
            background: ColorRect::new(bounding_box),
            value: String::new(),
            edition: if editable {
                Some(EditionInfo {
                    focused: false,
                    in_edition: false,
                })
            } else {
                None
            },
        }
    }
}
impl<T: TextInputContext, const N: usize> Widget for TextBox<T, N> {
    type Context = T;

    fn get_bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        self.background.get_bounding_box()
    }
    fn set_bounding_box(&mut self, bounding_box: embedded_graphics::primitives::Rectangle) {
        self.background.set_bounding_box(bounding_box);
    }
    fn get_focus(&self) -> Option<bool> {
        self.edition.as_ref().map(|edition| edition.focused)
    }
    fn set_focus(&mut self, from_dir: Option<crate::gui::FocusFrom>) -> Result<(), ()> {
        if let Some(ref mut edition) = self.edition {
            if from_dir.is_some() {
                edition.focused = true;
            } else {
                edition.focused = false;
                edition.in_edition = false;
            }
            Ok(())
        } else {
            Err(())
        }
    }
    fn on_event(&mut self, e: crate::calculator::Event, context: &mut Self::Context) -> Option<Event> {
        if let Some(ref mut edition) = self.edition {
            let input = context.get_context().text_from_event(&e);
            let remaining_event: Option<Event> = match input {
                TextInput::Text(text) => {
                    if !edition.in_edition {
                        edition.in_edition = true;
                        self.value.clear();
                    }
                    self.value.push_str(text).unwrap_or_default();
                    None
                }
                TextInput::Backspace => {
                    if edition.in_edition {
                        self.value.pop();
                    } else {
                        edition.in_edition = true;
                        self.value.clear();
                    }
                    None
                }
                TextInput::Action(action) => match action {
                    Action::Back => {
                        edition.in_edition = false;
                        None
                    }
                    Action::Clear => {
                        self.value.clear();
                        None
                    }
                    Action::Ok | Action::Exe => {
                        edition.in_edition = !edition.in_edition;
                        None
                    }
                    _ => Some(e),
                },
                TextInput::None => Some(e),
            };
            remaining_event
        } else {
            Some(e)
        }
    }
    fn render(&self, target: &mut crate::calculator::DeviceDislay, theme: &crate::gui::theme::Theme) {
        let (editable, focused, in_edition) = if let Some(ref edition) = self.edition {
            (true, edition.focused, edition.in_edition)
        } else {
            (false, false, false)
        };
        let mut background_theme = theme.clone();
        background_theme.foreground = if in_edition {
            theme.active
        } else if focused {
            theme.focused
        } else {
            theme.foreground
        };
        if !editable {
            background_theme.rect_border = 0;
        }
        self.background.render(target, &background_theme);
        let character_style = MonoTextStyleBuilder::new()
            .font(&NORMAL_FONT)
            .text_color(theme.foreground);
        let character_style = if in_edition {
            character_style.underline_with_color(theme.active).build()
        } else {
            character_style.build()
        };
        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Middle)
            .build();

        Text::with_text_style(
            &self.value,
            self.background.get_bounding_box().center(),
            character_style,
            text_style,
        )
        .draw(target)
        .unwrap();
    }
}
