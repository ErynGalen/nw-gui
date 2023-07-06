use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use nw_gui::{
    app::App,
    calculator::{Calculator, Color, Event},
    gui::{
        widgets::{Button, ColorRect, SplitDirection, SplitLayout},
        Widget,
    },
};

use heapless::String;

#[test]
fn os_main() {
    let mut calc = Calculator::new().unwrap();
    calc.render();

    let mut app = SimpleApp::new();
    app.run(&mut calc);
}

struct SimpleApp {
    gui: SplitLayout<Button<SharedAppState>, SplitLayout<Button<SharedAppState>, ColorRect<SharedAppState>>>,
    state: SharedAppState,
}

impl App for SimpleApp {
    fn new() -> Self {
        let mut app = Self {
            //gui_root: Grid::new(Rectangle::new(Point::new(0, 0), Size::new(320, 240)), Vec::new()),
            gui: SplitLayout::new(
                Rectangle::new(Point::new(0, 0), Size::new(320, 240)),
                SplitDirection::Horizontal,
                0.5,
            ),
            state: SharedAppState {
                new_text: Some(String::from("1st!!!")),
            },
        };

        app.gui.attach_first(Button::new(
            String::from("Button!"),
            Color::GREEN,
            Color::CSS_GRAY,
            2,
            Rectangle::default(),
            |context| {
                println!("Pressed!");
                context.new_text = Some(String::from("2nd!"));
            },
        ));
        app.gui
            .attach_second(SplitLayout::new(Rectangle::default(), SplitDirection::Vertical, 0.2));
        app.gui.get_second_mut().unwrap().attach_first(Button::new(
            String::from("Button!"),
            Color::GREEN,
            Color::CSS_GRAY,
            2,
            Rectangle::default(),
            |context| {
                println!("Pressed 2!");
                context.new_text = Some(String::from("Boo"));
            },
        ));

        app
    }
    fn run(&mut self, calc: &mut Calculator) {
        'running: loop {
            self.gui.render(calc.get_draw_target());
            calc.render();
            for e in calc.events() {
                if let Some(e) = self.gui.on_event(e, &mut self.state) {
                    match e {
                        Event::HardQuit => break 'running,
                        _ => (),
                    }
                }
                // apply state changes
                // if let Some(Either::Left(w)) = self.gui.get_mut(0) {
                //     if let Some(new_text) = self.state.new_text.take() {
                //         w.set_text(new_text);
                //     }
                // }
            }
        }
    }
}
#[derive(Debug)]
struct SharedAppState {
    new_text: Option<String<16>>,
}
