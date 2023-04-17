use embedded_graphics::{pixelcolor::Rgb888, prelude::*};

use embedded_graphics_simulator::{SimulatorDisplay, Window, OutputSettings};

pub struct NwDisplay {
    // TODO: display should be a generic draw target
    display: SimulatorDisplay<Rgb888>,
    window: Window,
}

impl NwDisplay {
    // possibly fails if the display has already been requested
    pub fn get() -> Option<Self> {
        Some(Self {
            display: SimulatorDisplay::<Rgb888>::new(Size::new(320, 240)),
            window: Window::new("Numworks", &OutputSettings::default()),
        })
    }

    pub fn render(&mut self) {
        self.window.update(&mut self.display);
    }

    pub fn target(&mut self) -> &mut SimulatorDisplay<Rgb888> {
        &mut self.display
    }
}