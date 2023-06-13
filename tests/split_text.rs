use embedded_graphics::{prelude::*, primitives::Rectangle};

use heapless::Vec;

use nw_gui::{
    calculator::{Calculator, Color, Event, KeyCode},
    gui::{Splitter, TextWidget, Widget},
};

#[test]
fn main() {
    let mut calc = Calculator::new().unwrap();

    let mut horizontal_splitter = Splitter {
        bounding_box: Rectangle::new(Point::new(0, 0), Size::new(320, 240)),
        vertical: false,
        ratio: 0.6,
    };
    let mut left_vertical_splitter = Splitter {
        bounding_box: horizontal_splitter.bounding_box_a(),
        vertical: true,
        ratio: 0.4,
    };

    let t1 = TextWidget::new(
        left_vertical_splitter.bounding_box_a(),
        Color::BLACK,
        "Numworks: Centered text",
    )
    .unwrap();

    let t2 = TextWidget::new(
        horizontal_splitter.bounding_box_b(),
        Color::RED,
        "I'm red text!",
    )
    .unwrap();

    let t3 = TextWidget::new(
        left_vertical_splitter.bounding_box_b(),
        Color::YELLOW,
        "Big text box",
    )
    .unwrap();

    let mut widgets: Vec<Widget, 8> = Vec::new();
    widgets.push(Widget::Text(t1)).unwrap();
    widgets.push(Widget::Text(t2)).unwrap();
    widgets.push(Widget::Text(t3)).unwrap();

    let mut horizontal_ratio = horizontal_splitter.ratio;
    let mut vertical_ratio = left_vertical_splitter.ratio;

    'running: loop {
        for w in &widgets {
            w.render(calc.get_draw_target());
        }
        calc.render();
        for e in calc.events() {
            //eprintln!("{:?}", e);
            match e {
                Event::HardQuit => break 'running,
                Event::KeyDown(k) => match k {
                    KeyCode::Right => horizontal_ratio = horizontal_splitter.ratio + 0.1,
                    KeyCode::Left => horizontal_ratio = horizontal_splitter.ratio - 0.1,
                    KeyCode::Up => vertical_ratio = left_vertical_splitter.ratio - 0.1,
                    KeyCode::Down => vertical_ratio = left_vertical_splitter.ratio + 0.1,
                    _ => (),
                },
                _ => (),
            }
            horizontal_ratio = horizontal_ratio.clamp(0.0, 1.0);
            vertical_ratio = vertical_ratio.clamp(0.0, 1.0);
        }
        horizontal_splitter.ratio += (horizontal_ratio - horizontal_splitter.ratio) * 0.5;
        left_vertical_splitter.ratio += (vertical_ratio - left_vertical_splitter.ratio) * 0.5;
        left_vertical_splitter.bounding_box = horizontal_splitter.bounding_box_a();
        if let Widget::Text(ref mut t) = widgets[0] {
            t.bounding_box = left_vertical_splitter.bounding_box_a();
        }
        if let Widget::Text(ref mut t) = widgets[1] {
            t.bounding_box = horizontal_splitter.bounding_box_b();
        }
        if let Widget::Text(ref mut t) = widgets[2] {
            t.bounding_box = left_vertical_splitter.bounding_box_b();
        }
    }
}
