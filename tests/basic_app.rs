use nw_gui::app::App;
use nw_gui::calculator::{Calculator, DeviceDisplay, Event};
use nw_gui::imgui::{Imgui, VerticalAlignment};

use embedded_graphics::{prelude::*, primitives::Rectangle};

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
    slider_value: i32,
    //running: bool,
}
impl ThisApp {
    fn new() -> Self {
        let mut app = Self {
            ui: Imgui::new(Rectangle::new(Point::new(0, 0), Size::new(320, 240))),
            slider_value: 0,
            //running: true,
        };
        app.create_layout();
        app
    }
}
impl App for ThisApp {
    fn on_event(&mut self, e: Event) {
        self.ui.on_event(e).unwrap();
        self.ui.new_frame(false);
        self.create_layout();
        self.ui.end_frame();
    }
    fn render(&self, target: &mut DeviceDisplay) {
        self.ui.render(target);
    }
}
impl ThisApp {
    fn create_layout(&mut self) {
        let (b1, _) = self.ui.button("button 1", VerticalAlignment::Top);
        let (b2, reset) = self.ui.button("reset", VerticalAlignment::Top);
        self.ui.focus_up_down(b1, b2);
        let s = self.ui.slider(&mut self.slider_value, -127, 128, VerticalAlignment::Top);
        self.ui.focus_up_down(b2, s);
        let (b3, _) = self.ui.button(&self.slider_value.to_string(), VerticalAlignment::Bottom);
        self.ui.focus_up_down(s, b3);
        if reset {
            self.slider_value = 0;
        }
    }
}
