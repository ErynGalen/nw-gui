use embedded_graphics::{pixelcolor::Rgb888, prelude::*};

use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

// TODO: add cfg flags to support different Display drivers

pub type DeviceDislay = SimulatorDisplay<Rgb888>;
pub type Color = Rgb888;

pub struct NwDisplay {
    display: DeviceDislay,
    window: Window,
}

impl NwDisplay {
    // possibly fails if the display has already been requested
    pub fn get() -> Option<Self> {
        Some(Self {
            display: DeviceDislay::new(Size::new(320, 240)),
            window: Window::new("Numworks", &OutputSettings::default()),
        })
    }

    pub fn render(&mut self) {
        self.window.update(&mut self.display);
        //self.window.show_static(&mut self.display);
    }

    pub fn target(&mut self) -> &mut DeviceDislay {
        &mut self.display
    }
}
