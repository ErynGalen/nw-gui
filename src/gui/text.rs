//! Types helping to process text.
//! 
//! # Example
//! ```
//! use nw_gui::gui::text::TextInputContext;
//! use nw_gui::gui::text::TextInput;
//! use nw_gui::calculator::{Event, KeyCode};
//! 
//! use heapless::Vec;
//! let mut context = TextInputContext::new();
//! let mut keys: Vec<KeyCode, 5> = Vec::new();
//! keys.push(KeyCode::Num6);
//! keys.push(KeyCode::Shift);
//! keys.push(KeyCode::Imaginary);
//! keys.push(KeyCode::Alpha);
//! keys.push(KeyCode::Division);
//! 
//! let mut result: Vec<TextInput, 5> = Vec::new();
//! for key in keys {
//!     let input = context.text_from_event(Event::KeyDown(key));
//!     result.push(input).unwrap();
//! }
//! 
//! let mut expected: Vec<TextInput, 5> = Vec::new();
//!
//! expected.push(TextInput::Text("6")).unwrap();
//! expected.push(TextInput::None).unwrap();
//! expected.push(TextInput::Text("}")).unwrap();
//! expected.push(TextInput::None).unwrap();
//! expected.push(TextInput::Text("v")).unwrap();
//! 
//! assert_eq!(result, expected);
//! ```

use crate::calculator::{Event, KeyCode};

/// State of the alpha key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaState {
    /// Alpha isn't active.
    No,
    /// Alpha is in small letters mode. The inner bool is whether the state is held or not.
    Small(bool),
    /// Alpha is in big letters mode. The inner bool is whether the state is held or not.
    Big(bool),
}

/// Type returned after paring an event into a text or an action.
/// 
/// See [`TextInputContext::text_from_event`] for more details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextInput {
    /// An [`Action`].
    Action(Action),
    /// Backspace.
    Backspace,
    /// Raw text.
    Text(&'static str),
    /// The event either mutated the [`TextInputContext`] or did nothing.
    None,
}
/// Actions resulting of a key press.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Ans,
    Back,
    Clear,
    Copy,
    Cut,
    /// The inner bool is whether Shift was active.
    Down(bool),
    Exe,
    Home,
    /// The inner bool is whether Shift was active.
    Left(bool),
    Ok,
    Paste,
    /// The inner bool is whether Shift was active.
    Right(bool),
    Toolbox,
    /// The inner bool is whether Shift was active.
    Up(bool),
    Var,
    Xnt,
}

/// Context used to parse events.
/// 
/// Represents the state of the Shift and Alpha keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextInputContext {
    /// Whether Shift is active.
    shift: bool,
    /// State of Alpha: see [`AlphaState`] for details.
    alpha: AlphaState,
}
impl TextInputContext {
    /// Create a new context, with Shift an Alpha inactive.
    pub fn new() -> Self {
        Self {
            alpha: AlphaState::No,
            shift: false,
        }
    }
    /// Use the context to process the given event.
    /// 
    /// This may modifiy the context, f.e. when pressing Shift or Alpha.
    pub fn text_from_event(&mut self, e: Event) -> TextInput {
        if let Event::KeyDown(key) = e {
            match key {
                KeyCode::Alpha => match self.alpha {
                    AlphaState::No => {
                        if self.shift {
                            self.shift = false;
                            self.alpha = AlphaState::Big(false);
                        } else {
                            self.alpha = AlphaState::Small(false);
                        }
                        TextInput::None
                    }
                    AlphaState::Small(lock) => {
                        if self.shift {
                            unreachable!("pressing shift while holding alpha should toggle alpha hold");
                        } else {
                            if lock {
                                self.alpha = AlphaState::No;
                            } else {
                                self.alpha = AlphaState::Small(true);
                            }
                            TextInput::None
                        }
                    }
                    AlphaState::Big(lock) => {
                        if self.shift {
                            unreachable!("pressing shift while holding alpha should toggle alpha hold");
                        } else {
                            if lock {
                                self.alpha = AlphaState::No;
                            } else {
                                self.alpha = AlphaState::Big(true);
                            }
                            TextInput::None
                        }
                    }
                },
                KeyCode::Shift => match self.alpha {
                    AlphaState::No => {
                        self.shift = !self.shift;
                        TextInput::None
                    }
                    AlphaState::Big(lock) => {
                        self.shift = false;
                        self.alpha = AlphaState::Small(lock);
                        TextInput::None
                    }
                    AlphaState::Small(lock) => {
                        self.shift = false;
                        self.alpha = AlphaState::Big(lock);
                        TextInput::None
                    }
                },
                KeyCode::Ans => self.full_key(
                    TextInput::Action(Action::Ans),
                    TextInput::None,
                    TextInput::Text("@"),
                    TextInput::Text("@"),
                ),
                KeyCode::Back => self.full_key(
                    TextInput::Action(Action::Back),
                    TextInput::None,
                    TextInput::Action(Action::Back),
                    TextInput::Action(Action::Back),
                ),
                KeyCode::Backspace => self.full_key(
                    TextInput::Backspace,
                    TextInput::Action(Action::Clear),
                    TextInput::Text("%"),
                    TextInput::Text("%"),
                ),
                KeyCode::Comma => self.basic_key(",", "_", "e", "E"),
                KeyCode::Cosine => self.basic_key("cos(", "acos(", "h", "H"),
                KeyCode::Division => self.shift_action_key("/", TextInput::None, "v", "V"),
                KeyCode::Dot => self.shift_action_key(".", TextInput::None, "!", "!"),
                KeyCode::Down => self.full_key(
                    TextInput::Action(Action::Down(false)),
                    TextInput::Action(Action::Down(true)),
                    TextInput::None,
                    TextInput::None,
                ),
                KeyCode::Up => self.full_key(
                    TextInput::Action(Action::Up(false)),
                    TextInput::Action(Action::Up(true)),
                    TextInput::None,
                    TextInput::None,
                ),
                KeyCode::Left => self.full_key(
                    TextInput::Action(Action::Left(false)),
                    TextInput::Action(Action::Left(true)),
                    TextInput::None,
                    TextInput::None,
                ),
                KeyCode::Right => self.full_key(
                    TextInput::Action(Action::Right(false)),
                    TextInput::Action(Action::Right(true)),
                    TextInput::None,
                    TextInput::None,
                ),
                KeyCode::Ee => self.shift_action_key("*10^", TextInput::None, "\\", "\\"),
                KeyCode::Exe => self.full_key(
                    TextInput::Action(Action::Exe),
                    TextInput::None,
                    TextInput::Action(Action::Exe),
                    TextInput::Action(Action::Exe),
                ),
                KeyCode::Ok => self.full_key(
                    TextInput::Action(Action::Ok),
                    TextInput::None,
                    TextInput::Action(Action::Ok),
                    TextInput::Action(Action::Ok),
                ),
                KeyCode::Home => self.full_key(
                    TextInput::Action(Action::Home),
                    TextInput::Action(Action::Home),
                    TextInput::Action(Action::Home),
                    TextInput::Action(Action::Home),
                ),
                KeyCode::Exp => self.basic_key("e^", "[", "a", "A"),
                KeyCode::Imaginary => self.basic_key("i", "}", "d", "D"),
                KeyCode::LeftParen => self.basic_key("(", "()", "p", "P"),
                KeyCode::RightParen => self.basic_key(")", ")", "q", "Q"),
                KeyCode::Ln => self.basic_key("ln(", "]", "b", "B"),
                KeyCode::Log => self.basic_key("log(", "{", "c", "C"),
                KeyCode::Minus => self.shift_action_key("-", TextInput::None, " ", " "),
                KeyCode::Multiplication => self.shift_action_key("*", TextInput::None, "u", "U"),
                KeyCode::Num0 => self.shift_action_key("0", TextInput::None, "?", "?"),
                KeyCode::Num1 => self.shift_action_key("1", TextInput::None, "w", "W"),
                KeyCode::Num2 => self.shift_action_key("2", TextInput::None, "x", "X"),
                KeyCode::Num3 => self.shift_action_key("3", TextInput::None, "y", "Y"),
                KeyCode::Num4 => self.shift_action_key("4", TextInput::None, "r", "R"),
                KeyCode::Num5 => self.shift_action_key("5", TextInput::None, "s", "S"),
                KeyCode::Num6 => self.shift_action_key("6", TextInput::None, "t", "T"),
                KeyCode::Num7 => self.shift_action_key("7", TextInput::None, "m", "M"),
                KeyCode::Num8 => self.shift_action_key("8", TextInput::None, "n", "N"),
                KeyCode::Num9 => self.shift_action_key("9", TextInput::None, "o", "O"),
                KeyCode::OnOff => TextInput::None,
                KeyCode::Pi => self.basic_key("pi", "=", "j", "J"),
                KeyCode::Plus => self.shift_action_key("+", TextInput::None, "z", "Z"),
                KeyCode::Power => self.basic_key("^", "->", "f", "F"),
                KeyCode::Sine => self.basic_key("sin(", "asin(", "g", "G"),
                KeyCode::Tangent => self.basic_key("tan(", "atan(", "i", "I"),
                KeyCode::Sqrt => self.basic_key("sqrt(", "<", "k", "K"),
                KeyCode::Square => self.basic_key("^2", ">", "l", "L"),
                KeyCode::Toolbox => self.full_key(
                    TextInput::Action(Action::Toolbox),
                    TextInput::Action(Action::Paste),
                    TextInput::Text("\""),
                    TextInput::Text("'"),
                ),
                KeyCode::Var => self.full_key(
                    TextInput::Action(Action::Var),
                    TextInput::Action(Action::Copy),
                    TextInput::Text(";"),
                    TextInput::Text(";"),
                ),
                KeyCode::Xnt => self.full_key(
                    TextInput::Action(Action::Xnt),
                    TextInput::Action(Action::Cut),
                    TextInput::Text(":"),
                    TextInput::Text(":"),
                ),
            }
        } else {
            TextInput::None
        }
    }

    // Primitives to help managing the state.

    fn basic_key(
        &mut self,
        normal: &'static str,
        shift: &'static str,
        alpha: &'static str,
        big_alpha: &'static str,
    ) -> TextInput {
        self.full_key(
            TextInput::Text(normal),
            TextInput::Text(shift),
            TextInput::Text(alpha),
            TextInput::Text(big_alpha),
        )
    }
    fn shift_action_key(
        &mut self,
        normal: &'static str,
        shift: TextInput,
        alpha: &'static str,
        big_alpha: &'static str,
    ) -> TextInput {
        self.full_key(
            TextInput::Text(normal),
            shift,
            TextInput::Text(alpha),
            TextInput::Text(big_alpha),
        )
    }
    fn full_key(&mut self, normal: TextInput, shift: TextInput, alpha: TextInput, big_alpha: TextInput) -> TextInput {
        match self.alpha {
            AlphaState::No => {
                if self.shift {
                    self.shift = false;
                    shift
                } else {
                    normal
                }
            }
            AlphaState::Big(lock) => {
                if !lock {
                    self.alpha = AlphaState::No;
                }
                big_alpha
            }
            AlphaState::Small(lock) => {
                if !lock {
                    self.alpha = AlphaState::No;
                }
                alpha
            }
        }
    }
}
