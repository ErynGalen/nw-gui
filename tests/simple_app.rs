use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use nw_gui::{
    calculator::{Calculator, Color, Event},
    gui::{Grid, RectWidget, Widget, WidgetCollection},
};

struct AppW(RectWidget);
impl WidgetCollection for AppW {
    type Item = RectWidget;
    fn len(&self) -> usize {
        1
    }
    fn get(&self, n: usize) -> Option<&Self::Item> {
        if n == 0 {
            Some(&self.0)
        } else {
            None
        }
    }
    fn get_mut(&mut self, n: usize) -> Option<&mut Self::Item> {
        if n == 0 {
            Some(&mut self.0)
        } else {
            None
        }
    }
}

#[test]
fn main() {
    let mut calc = Calculator::new().unwrap();

    let children = AppW(RectWidget {
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
    });
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
        for e in calc.events() {
            match e {
                Event::HardQuit => break 'running,
                other => root.on_event(other),
            }
        }
        break 'running;
    }
}
