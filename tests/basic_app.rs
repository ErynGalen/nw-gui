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
    //running: bool,
}
impl ThisApp {
    fn new() -> Self {
        let mut app = Self {
            ui: Imgui::new(Rectangle::new(Point::new(0, 0), Size::new(320, 240))),
            //running: true,
        };
        let b1 = app.ui.button("line 1", VerticalAlignment::Top);
        let b2 = app.ui.button("line 2", VerticalAlignment::Top);
        let b3 = app.ui.button("bottom -1", VerticalAlignment::Bottom);
        app.ui.focus_up_down(b1, b2);
        app.ui.focus_up_down(b2, b3);
        app
    }
}
impl App for ThisApp {
    fn on_event(&mut self, e: Event) {
        self.ui.on_event(e);
        self.ui.new_frame();
        let b1 = self.ui.button("line 1", VerticalAlignment::Top);
        let b2 = self.ui.button("line 2", VerticalAlignment::Top);
        let b3 = self.ui.button("bottom -1", VerticalAlignment::Bottom);
        self.ui.focus_up_down(b1, b2);
        self.ui.focus_up_down(b2, b3);
    }
    fn render(&self, target: &mut DeviceDisplay) {
        self.ui.render(target);
    }
}
