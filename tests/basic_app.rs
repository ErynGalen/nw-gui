use nw_gui::app::App;
use nw_gui::calculator::{Calculator, DeviceDisplay, Event, Keycode};
use nw_gui::imgui::{Imgui, VerticalAlignment};

use embedded_graphics::{prelude::*, primitives::Rectangle};
use heapless::Vec;

#[test]
fn os_main() {
    let mut calc = Calculator::new().unwrap();
    let mut app = ThisApp::new();
    'os_loop: loop {
        app.render(calc.get_draw_target());
        calc.render();
        for e in calc.events() {
            match e {
                Event::HardQuit => break 'os_loop,
                _ => app.on_event(e),
            }
        }
    }
}

struct ThisApp {
    ui: Imgui,
    running: bool,
    current_focus: u32,
}
impl ThisApp {
    fn new() -> Self {
        let mut app = Self {
            ui: Imgui::new(Rectangle::new(Point::new(0, 0), Size::new(320, 240))),
            running: true,
            current_focus: 0,
        };
        app.ui.button("line 1", VerticalAlignment::Top, true);
        app.ui.button("line 2", VerticalAlignment::Top, false);
        app.ui.button("bottom -1", VerticalAlignment::Bottom, false);
        app
    }
}
impl App for ThisApp {
    fn on_event(&mut self, e: Event) {
        match e {
            Event::KeyDown(k) => match k {
                Keycode::Up => {
                    if self.current_focus > 0 {
                        self.current_focus -= 1
                    }
                }
                Keycode::Down => {
                    if self.current_focus < 7 {
                        self.current_focus += 1
                    }
                }
                Keycode::Q => self.running = false,
                _ => (),
            },
            _ => (),
        }
        self.ui.new_frame();
        let mut focus: Vec<bool, 8> = Vec::from_slice(&[false; 8]).unwrap();
        if self.current_focus < 8 {
            *focus.get_mut(self.current_focus as usize).unwrap() = true;
        }
        self.ui.button("line 1", VerticalAlignment::Top, focus[0]);
        self.ui.button("line 2", VerticalAlignment::Top, focus[1]);
        self.ui
            .button("bottom -1", VerticalAlignment::Bottom, focus[2]);
    }
    fn render(&self, target: &mut DeviceDisplay) {
        self.ui.render(target);
    }
}
