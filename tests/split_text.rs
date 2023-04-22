use embedded_graphics::{prelude::*, primitives::Rectangle};

use nw_gui::{
    calculator::{Calculator, Color, Event},
    gui::{Splitter, TextWidget, Widget},
};

#[test]
fn main() {
    let mut calc = Calculator::new().unwrap();

    let horizontal_splitter = Splitter {
        bounding_box: Rectangle::new(Point::new(0, 0), Size::new(320, 240)),
        vertical: false,
        ratio: 0.6,
    };
    let left_vertical_splitter = Splitter {
        bounding_box: horizontal_splitter.bounding_box_a(),
        vertical: true,
        ratio: 0.4,
    };

    let t1 = Widget::Text(
        TextWidget::new(
            left_vertical_splitter.bounding_box_a(),
            Color::BLACK,
            Color::CSS_LIGHT_GRAY,
            "Numworks: Centered text",
        )
        .unwrap(),
    );

    let t2 = TextWidget::new(
        horizontal_splitter.bounding_box_b(),
        Color::RED,
        Color::WHITE,
        "I'm red text!",
    ).unwrap();

    let t3 = TextWidget::new(
        left_vertical_splitter.bounding_box_b(),
        Color::YELLOW,
        Color::CSS_SEA_GREEN,
        "Big text box",
    ).unwrap();

    t1.render(calc.get_draw_target());
    t2.render(calc.get_draw_target());
    t3.render(calc.get_draw_target());

    'running: loop {
        calc.render();
        for e in calc.events() {
            //eprintln!("{:?}", e);
            match e {
                Event::HardQuit => break 'running,
                _ => (),
            }
        }
    }
}
