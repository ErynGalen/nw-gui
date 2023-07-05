use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use nw_gui::{
    app::App,
    calculator::{Calculator, Color, Event, KeyCode},
    gui::{
        widgets::{Button, ColorRect, Grid},
        Widget,
    },
};

use either::Either;
use heapless::{String, Vec};

#[test]
fn os_main() {
    let mut calc = Calculator::new().unwrap();
    calc.render();

    let mut app = SimpleApp::new();
    app.run(&mut calc);
}

struct SimpleApp {
    gui_root: Grid<
        2,
        1,
        Vec<Either<Button<SharedAppState>, Grid<1, 2, Vec<ColorRect<SharedAppState>, 2>>>, 2>,
    >,
    state: SharedAppState,
}

impl App for SimpleApp {
    fn new() -> Self {
        let mut app = Self {
            gui_root: Grid::new(
                Rectangle::new(Point::new(0, 0), Size::new(320, 240)),
                Vec::new(),
            ),
            state: SharedAppState {
                new_text: Some(String::from("1st!!!")),
            },
        };
        app.gui_root
            .add_child_at(
                Either::Left(Button::new(
                    String::from("Button!"),
                    Color::GREEN,
                    Rectangle::default(),
                    Color::CSS_GRAY,
                    2,
                    |context| {
                        println!("Pressed!");
                        context.new_text = Some(String::from("2nd!"));
                    },
                )),
                (0, 0),
                (1, 1),
                4,
            )
            .unwrap();

        let right_id = app
            .gui_root
            .add_child_at(
                Either::Right(Grid::new(Rectangle::default(), Vec::new())),
                (1, 0),
                (1, 1),
                0,
            )
            .unwrap();

        if let Some(Either::Right(right)) = app.gui_root.get_mut(right_id) {
            right
                .add_child_at(
                    ColorRect::new(Color::CSS_PINK, Color::CYAN, 2, Rectangle::default()),
                    (0, 0),
                    (1, 1),
                    4,
                )
                .unwrap();
            right
                .add_child_at(
                    ColorRect::new(
                        Color::CSS_PALE_GOLDENROD,
                        Color::CSS_DEEP_PINK,
                        2,
                        Rectangle::default(),
                    ),
                    (0, 1),
                    (1, 1),
                    4,
                )
                .unwrap();
        } else {
            panic!();
        }
        app
    }
    fn run(&mut self, calc: &mut Calculator) {
        let mut focused = true;
        'running: loop {
            self.gui_root.render(calc.get_draw_target(), focused);
            calc.render();
            for e in calc.events() {
                if let Some(e) = self.gui_root.on_event(e, &mut self.state) {
                    match e {
                        Event::HardQuit => break 'running,
                        Event::KeyDown(KeyCode::Home) => focused = !focused,
                        _ => (),
                    }
                }
                // apply state changes
                if let Some(Either::Left(w)) = self.gui_root.get_mut(0) {
                    if let Some(new_text) = self.state.new_text.take() {
                        w.set_text(new_text);
                    }
                }
            }
        }
    }
}
#[derive(Debug)]
struct SharedAppState {
    new_text: Option<String<16>>,
}
