use crate::numworks_display::NwDisplay;

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
};

pub fn nw_main() {
    let mut display = NwDisplay::get().unwrap();

    let line_style = PrimitiveStyle::with_stroke(Rgb888::YELLOW, 3);

    let mut x = 0;
    while x < 320 {
        x += 1;
        let line = Line::new(Point::new(x, 0), Point::new(100, 100));
        line.into_styled(line_style).draw(display.target()).unwrap();

        display.render();
    }
}
