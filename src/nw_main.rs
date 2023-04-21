use embedded_graphics::prelude::RgbColor;

use crate::{
    gui::{Gui, TextWidget, Widget},
    numworks_display::{Color, NwDisplay},
};

pub fn nw_main() {
    let mut display = NwDisplay::get().unwrap();

    let mut gui = Gui::new();
    gui.add_widget(Widget::Text(TextWidget::new(153, 31, Color::RED, Color::YELLOW, "hello!")))
        .unwrap();
    gui.add_widget(Widget::Text(TextWidget::new(155, 100, Color::YELLOW, Color::GREEN, "how are you?")))
        .unwrap();
    gui.render(display.target());

    loop {
        display.render();
    }
}
