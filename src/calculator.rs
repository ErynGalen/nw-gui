//! This module contains types and traits associated with the calculator as a whole.

use core::iter::Iterator;

use embedded_graphics::{pixelcolor::Rgb888, prelude::*};

use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};

use embedded_graphics_simulator::sdl2;

// TODO: add cfg flags to support different Display drivers, event fetcher, ...

/// This type represent the display of the calculator.
/// Everything is drawn on it.
pub type DeviceDislay = SimulatorDisplay<Rgb888>;
/// This type represents the colors available on the calculator display.
/// It should be the only type used for colors.
pub type Color = Rgb888;

/// This struct represents the calculator, as seen by an application.
/// The OS may pass a `Calculator` object to the applications.
///
/// # Panics
/// `Calculator::render()` must be called before `Calculator::events()`,
/// otherwise `Calculator::events()` will panic.
pub struct Calculator {
    display: DeviceDislay,
    window: Window,
}

impl Calculator {
    /// Request the calculator.
    /// May fail if the calculator has already been requested.
    pub fn new() -> Option<Self> {
        Some(Self {
            display: DeviceDislay::new(Size::new(320, 240)),
            window: Window::new("Numworks", &OutputSettings::default()),
        })
    }

    /// Render the buffer to the physical display.
    pub fn render(&mut self) {
        self.window.update(&mut self.display);
        //self.window.show_static(&mut self.display);
    }

    /// Return a `DeviceDisplay` representing the display buffer,
    /// on which applications should draw their UI.
    pub fn get_draw_target(&mut self) -> &mut DeviceDislay {
        &mut self.display
    }

    /// Iterator on events recieved by the OS at a certain point.
    pub fn events(&mut self) -> impl Iterator<Item = Event> + '_ {
        self.window
            .events()
            .filter_map(|sdl_event| match sdl_event {
                SimulatorEvent::MouseButtonDown { .. }
                | SimulatorEvent::MouseButtonUp { .. }
                | SimulatorEvent::MouseMove { .. }
                | SimulatorEvent::MouseWheel { .. } => None,
                SimulatorEvent::Quit => Some(Event::HardQuit),
                SimulatorEvent::KeyDown {
                    keycode,
                    keymod: _,
                    repeat: _,
                } => {
                    if let Some(key) = KeyCode::try_from_sdl2(keycode) {
                        Some(Event::KeyDown(key))
                    } else {
                        None
                    }
                }
                SimulatorEvent::KeyUp {
                    keycode,
                    keymod: _,
                    repeat: _,
                } => {
                    if let Some(key) = KeyCode::try_from_sdl2(keycode) {
                        Some(Event::KeyUp(key))
                    } else {
                        None
                    }
                }
            })
    }
}

/// This struct holds an event produced by the OS.
#[derive(Debug)]
pub enum Event {
    /// A key has been pressed.
    KeyDown(KeyCode),
    /// A key has been released.
    KeyUp(KeyCode),
    /// Quit whatsoever, may be handled by the OS directly,
    /// so applications may or may not recieve it.
    HardQuit,
}

/// Raw key codes representing each key on the physical keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum KeyCode {
    Left,
    Up,
    Down,
    Right,
    Ok,
    Back,
    Home,
    OnOff,
    Shift,
    Alpha,
    Xnt,
    Var,
    Toolbox,
    Backspace,
    Exp,
    Ln,
    Log,
    Imaginary,
    Comma,
    Power,
    Sine,
    Cosine,
    Tangent,
    Pi,
    Sqrt,
    Square,
    Num7,
    Num8,
    Num9,
    LeftParen,
    RightParen,
    Num4,
    Num5,
    Num6,
    Multiplication,
    Division,
    Num1,
    Num2,
    Num3,
    Plus,
    Minus,
    Num0,
    Dot,
    Ee,
    Ans,
    Exe,
}
impl KeyCode {
    pub fn try_from_sdl2(key: sdl2::Keycode) -> Option<Self> {
        match key {
            sdl2::Keycode::Left => Some(Self::Left),
            sdl2::Keycode::Up => Some(Self::Up),
            sdl2::Keycode::Down => Some(Self::Down),
            sdl2::Keycode::Right => Some(Self::Right),
            // Ok,
            sdl2::Keycode::Escape => Some(Self::Back),
            sdl2::Keycode::Home => Some(Self::Home),
            // OnOff,
            // Shift,
            // Alpha,
            // Xnt,
            // Var,
            // Toolbox,
            sdl2::Keycode::Backspace => Some(Self::Backspace),
            // Exp,
            // Ln,
            // Log,
            // Imaginary,
            sdl2::Keycode::Comma => Some(Self::Comma),
            // Power,
            // Sine,
            // Cosine,
            // Tangent,
            // Pi,
            // Sqrt,
            // Square,
            sdl2::Keycode::Num7 => Some(Self::Num7),
            sdl2::Keycode::Num8 => Some(Self::Num8),
            sdl2::Keycode::Num9 => Some(Self::Num9),
            // LeftParen,
            // RightParen,
            sdl2::Keycode::Num4 => Some(Self::Num4),
            sdl2::Keycode::Num5 => Some(Self::Num5),
            sdl2::Keycode::Num6 => Some(Self::Num6),
            // Multiplication,
            // Division,
            sdl2::Keycode::Num1 => Some(Self::Num1),
            sdl2::Keycode::Num2 => Some(Self::Num2),
            sdl2::Keycode::Num3 => Some(Self::Num3),
            // Plus,
            // Minus,
            // Num0,
            // Dot,
            // Ee,
            // Ans,
            sdl2::Keycode::Return => Some(Self::Exe),
            _ => None,
        }
    }
}
