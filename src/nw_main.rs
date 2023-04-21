use embedded_graphics::{prelude::*, primitives::Rectangle};

use crate::{
    gui::{Gui, TextWidget, Widget},
    numworks_display::{Color, NwDisplay},
};

pub fn nw_main() {
    let mut display = NwDisplay::get().unwrap();

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
    gui.render(display.target());

    loop {
        display.render();
    }
}
