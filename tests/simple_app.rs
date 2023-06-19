use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use nw_gui::{
    calculator::{Calculator, Color, Event, KeyCode},
    gui::{
        widgets::{Grid, ColorRect},
        Widget,
    },
};

use heapless::Vec;

#[test]
fn main() {
    let mut calc = Calculator::new().unwrap();
    calc.render();

    let children: Vec<ColorRect, 2> = Vec::new();

    let mut root: Grid<2, 1, _> = Grid::new(
        Rectangle::new(
            Point { x: 0, y: 0 },
            Size {
                width: 320,
                height: 240,
            },
        ),
        children,
    );

    root.add_child_at(
        ColorRect {
            bounging_box: Rectangle::default(),
            fill_color: Color::CSS_RED,
            border_color: Color::GREEN,
            border_width: 2,
        },
        (0, 0),
        (1, 1),
        4,
    )
    .unwrap();
    root.add_child_at(
        ColorRect {
            bounging_box: Rectangle::default(),
            fill_color: Color::CSS_RED,
            border_color: Color::GREEN,
            border_width: 2,
        },
        (1, 0),
        (1, 1),
        4,
    )
    .unwrap();

    let mut focused = true;
    'running: loop {
        root.render(calc.get_draw_target(), focused);
        calc.render();
        for e in calc.events() {
            match e {
                Event::HardQuit => break 'running,
                Event::KeyDown(KeyCode::Exe) => focused = !focused,
                other => root.on_event(other),
            }
        }
    }
}
