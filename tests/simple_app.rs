use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use nw_gui::{
    app::App,
    calculator::{Calculator, Color, Event},
    gui::{
        text::{TextInputContext, TextInputState},
        theme::Theme,
        widgets::{Button, SplitDirection, SplitLayout, TextBox},
        Widget,
    },
};

use heapless::String;

#[test]
fn os_main() {
    let mut calc = Calculator::new().unwrap();
    calc.render();

    let mut app = SimpleApp::new();
    app.run(&mut calc);
}

struct SimpleApp {
    gui: SplitLayout<
        SplitLayout<TextBox<SharedAppState, 16>, Button<SharedAppState>>,
        SplitLayout<Button<SharedAppState>, TextBox<SharedAppState, 32>>,
    >,
    state: SharedAppState,
    theme: Theme,
}

impl App for SimpleApp {
    fn new() -> Self {
        let mut app = Self {
            gui: SplitLayout::new(
                Rectangle::new(Point::new(0, 0), Size::new(320, 240)),
                SplitDirection::Horizontal,
                0.5,
            ),
            state: SharedAppState {
                new_color: None,
                input_state: TextInputState::new(),
            },
            theme: Theme::default(),
        };

        app.gui.attach_first(
            SplitLayout::new(Rectangle::default(), SplitDirection::Vertical, 0.1),
            (0, 0),
        );
        app.gui
            .get_first_mut()
            .unwrap()
            .attach_first(TextBox::new(Rectangle::default(), false), (2, 2));
        app.gui.get_first_mut().unwrap().get_first_mut().unwrap().value = "Mods:      ".into();
        app.gui.get_first_mut().unwrap().attach_second(
            Button::new(String::from("Button!"), Rectangle::default(), |context| {
                println!("Pressed!");
                context.new_color = Some(Color::CSS_DARK_MAGENTA);
            }),
            (5, 20),
        );
        app.gui.attach_second(
            SplitLayout::new(Rectangle::default(), SplitDirection::Vertical, 0.2),
            (0, 0),
        );
        app.gui.get_second_mut().unwrap().attach_first(
            Button::new(String::from("Button!"), Rectangle::default(), |context| {
                println!("Pressed 2!");
                context.new_color = Some(Color::CSS_DARK_ORCHID);
            }),
            (10, 5),
        );
        app.gui
            .get_second_mut()
            .unwrap()
            .attach_second(TextBox::new(Rectangle::default(), true), (3, 3));

        app
    }
    fn run(&mut self, calc: &mut Calculator) {
        'running: loop {
            self.gui.render(calc.get_draw_target(), &self.theme);
            calc.render();
            for e in calc.events() {
                if let Some(e) = self.gui.on_event(e, &mut self.state) {
                    match e {
                        Event::HardQuit => break 'running,
                        _ => (),
                    }
                }
                // apply state changes
                if let Some(color) = self.state.new_color {
                    self.theme.background = color;
                }
                let mut input_state_string: String<16> = String::from("Mods: ");
                if self.state.input_state.get_shift() {
                    input_state_string.push_str(" S ").unwrap();
                } else {
                    input_state_string.push_str("   ").unwrap();
                }
                input_state_string
                    .push_str(match self.state.input_state.get_alpha() {
                        nw_gui::gui::text::AlphaState::Big(lock) => {
                            if lock {
                                "@A"
                            } else {
                                " A"
                            }
                        }
                        nw_gui::gui::text::AlphaState::No => "  ",
                        nw_gui::gui::text::AlphaState::Small(lock) => {
                            if lock {
                                "@a"
                            } else {
                                " a"
                            }
                        }
                    })
                    .unwrap();
                self.gui.get_first_mut().unwrap().get_first_mut().unwrap().value = input_state_string;
            }
        }
    }
}
#[derive(Debug)]
struct SharedAppState {
    new_color: Option<Color>,
    input_state: TextInputState,
}
impl TextInputContext for SharedAppState {
    fn get_context(&mut self) -> &mut TextInputState {
        &mut self.input_state
    }
}
