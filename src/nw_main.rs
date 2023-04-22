use embedded_graphics::{prelude::*, primitives::Rectangle};

use crate::{
    calculator::{Calculator, Color, Event},
    gui::{Gui, TextWidget, Widget},
};

pub fn nw_main() {
    let mut calc = Calculator::new().unwrap();

    let mut gui = Gui::new();
    gui.add_widget(Widget::Text(
        TextWidget::new(
            Rectangle::new(Point::new(34, 35), Size::new(200, 20)),
            Color::BLACK,
            Color::CSS_LIGHT_GRAY,
            "Numworks: Centered text",
        )
        .unwrap(),
    ))
    .unwrap();
    gui.render(calc.get_draw_target());

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
