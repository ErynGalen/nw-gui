use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use nw_gui::{
    calculator::{Calculator, Color, Event},
    gui::{
        widgets::{Grid, RectWidget},
        Widget,
    },
};

use heapless::Vec;

#[test]
fn main() {
    let mut calc = Calculator::new().unwrap();
    calc.render();

    let mut children: Vec<RectWidget, 1> = Vec::new();
    children
        .push(RectWidget {
            bounging_box: Rectangle::new(
                Point { x: 0, y: 0 },
                Size {
                    width: 20,
                    height: 20,
                },
            ),
            fill_color: Color::CSS_RED,
            border_color: Color::GREEN,
            border_width: 2,
        })
        .unwrap();
    let mut root: Grid<2, 1, _> = Grid::new(
        Rectangle::new(
            Point { x: 0, y: 0 },
            Size {
                width: 20,
                height: 20,
            },
        ),
        children,
    );

    'running: loop {
        root.render(calc.get_draw_target());
        calc.render();
        for e in calc.events() {
            match e {
                Event::HardQuit => break 'running,
                other => root.on_event(other),
            }
        }
    }
}
