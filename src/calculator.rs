use core::iter::Iterator;

use embedded_graphics::{pixelcolor::Rgb888, prelude::*};

use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};

// TODO: add cfg flags to support different Display drivers, event fetcher, ...

pub type DeviceDislay = SimulatorDisplay<Rgb888>;
pub type Color = Rgb888;

pub use embedded_graphics_simulator::sdl2::Keycode;

pub struct Calculator {
    display: DeviceDislay,
    window: Window,
}

impl Calculator {
    /// possibly fails if the calculator has already been requested
    pub fn new() -> Option<Self> {
        Some(Self {
            display: DeviceDislay::new(Size::new(320, 240)),
            window: Window::new("Numworks", &OutputSettings::default()),
        })
    }

    pub fn render(&mut self) {
        self.window.update(&mut self.display);
        //self.window.show_static(&mut self.display);
    }

    pub fn get_draw_target(&mut self) -> &mut DeviceDislay {
        &mut self.display
    }

    pub fn events(&mut self) -> impl Iterator<Item = Event> + '_ {
        self.window
            .events()
            .filter_map(|sdl_event| match sdl_event {
                SimulatorEvent::MouseButtonDown {
                    mouse_btn: _,
                    point: _,
                }
                | SimulatorEvent::MouseButtonUp {
                    mouse_btn: _,
                    point: _,
                } => None,
                SimulatorEvent::MouseMove { point: _ } => None,
                SimulatorEvent::MouseWheel {
                    scroll_delta: _,
                    direction: _,
                } => None,
                SimulatorEvent::Quit => Some(Event::HardQuit),
                SimulatorEvent::KeyDown {
                    keycode,
                    keymod: _,
                    repeat: _,
                } => Some(Event::KeyDown(keycode)),
                SimulatorEvent::KeyUp {
                    keycode,
                    keymod: _,
                    repeat: _,
                } => Some(Event::KeyUp(keycode)),
            })
    }
}

#[derive(Debug)]
pub enum Event {
    KeyDown(Keycode),
    KeyUp(Keycode),
    /// quit whatsoever
    HardQuit,
}
