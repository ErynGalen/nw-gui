use embedded_graphics::prelude::RgbColor;

use crate::{
    gui::{Gui, SquareWidget, Widget},
    numworks_display::{Color, NwDisplay},
};

pub fn nw_main() {
    let mut display = NwDisplay::get().unwrap();

    let mut gui = Gui::new();
    gui.add_widget(Widget::Square(SquareWidget::new(153, 31, Color::RED)))
        .unwrap();
    gui.add_widget(Widget::Square(SquareWidget::new(155, 16, Color::YELLOW)))
        .unwrap();
    gui.render(display.target());

    loop {
        display.render();
    }
}
