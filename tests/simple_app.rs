use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use nw_gui::{
    calculator::{Calculator, Color, Event, KeyCode},
    gui::{
        widgets::{ColorRect, Grid},
        Widget,
    },
};

use heapless::Vec;
use either::Either;

#[test]
fn main() {
    let mut calc = Calculator::new().unwrap();
    calc.render();

    let children: Vec<Either<ColorRect, Grid<1, 2, Vec<ColorRect, 2>>>, 2> = Vec::new();

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
        Either::Left(ColorRect {
            bounging_box: Rectangle::default(),
            fill_color: Color::CSS_RED,
            border_color: Color::GREEN,
            border_width: 2,
        }),
        (0, 0),
        (1, 1),
        4,
    )
    .unwrap();

    let right_id = root
        .add_child_at(
            Either::Right(Grid::new(Rectangle::default(), Vec::new())),
            (1, 0),
            (1, 1),
            0,
        )
        .unwrap();

    if let Some(Either::Right(right)) = root.get_mut(right_id) {
        right
            .add_child_at(
                ColorRect {
                    border_color: Color::CYAN,
                    border_width: 2,
                    bounging_box: Rectangle::default(),
                    fill_color: Color::CSS_PINK,
                },
                (0, 0),
                (1, 1),
                4,
            )
            .unwrap();
        right
            .add_child_at(
                ColorRect {
                    border_color: Color::CSS_DEEP_PINK,
                    border_width: 2,
                    bounging_box: Rectangle::default(),
                    fill_color: Color::CSS_PALE_GOLDENROD,
                },
                (0, 1),
                (1, 1),
                4,
            )
            .unwrap();
    } else {
        panic!();
    }

    let mut focused = true;
    'running: loop {
        root.render(calc.get_draw_target(), focused);
        calc.render();
        for e in calc.events() {
            if let Some(e) = root.on_event(e) {
                match e {
                    Event::HardQuit => break 'running,
                    Event::KeyDown(KeyCode::Exe) => focused = !focused,
                    _ => (),
                }
            }
        }
    }
}
