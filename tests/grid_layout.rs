use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use nw_gui::{
    app::App,
    calculator::{Calculator, Color, Event},
    gui::{
        widgets::{ColorRect, Grid},
        Widget,
    },
};

use heapless::Vec;

#[test]
fn os_main() {
    let mut calc = Calculator::new().unwrap();
    calc.render();

    let mut app = SimpleApp::new();
    app.run(&mut calc);
}

#[derive(Debug)]
struct SimpleApp {
    grid: Grid<5, 3, Vec<ColorRect<()>, 3>>,
}
impl App for SimpleApp {
    fn new() -> Self {
        let mut app = Self {
            grid: Grid::new(
                Rectangle::new(Point::new(0, 0), Size::new(320, 240)),
                Vec::new(),
            ),
        };
        app.grid
            .add_child_at(
                ColorRect::new(Color::GREEN, Color::WHITE, 2, Rectangle::default()),
                (3, 0),
                (2, 1),
                4,
            )
            .unwrap();
        app.grid
            .add_child_at(
                ColorRect::new(Color::BLUE, Color::WHITE, 2, Rectangle::default()),
                (0, 1),
                (3, 1),
                4,
            )
            .unwrap();
        app.grid
            .add_child_at(
                ColorRect::new(Color::RED, Color::WHITE, 2, Rectangle::default()),
                (1, 2),
                (3, 1),
                4,
            )
            .unwrap();
        app
    }
    fn run(&mut self, calc: &mut Calculator) {
        'running: loop {
            self.grid.render(calc.get_draw_target(), true);
            calc.render();
            for e in calc.events() {
                if let Some(e) = self.grid.on_event(e, &mut ()) {
                    match e {
                        Event::HardQuit => break 'running,
                        _ => (),
                    }
                }
            }
        }
    }
}
