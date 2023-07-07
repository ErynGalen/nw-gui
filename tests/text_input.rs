use heapless::Vec;
use nw_gui::calculator::{Event, KeyCode};
use nw_gui::gui::text::{TextInput, TextInputState};
#[test]
fn print_text_input() {
    let mut context = TextInputState::new();
    let mut keys: Vec<KeyCode, 10> = Vec::new();
    keys.push(KeyCode::Num6).unwrap();
    keys.push(KeyCode::Shift).unwrap();
    keys.push(KeyCode::Imaginary).unwrap();
    keys.push(KeyCode::Alpha).unwrap();
    keys.push(KeyCode::Shift).unwrap();
    keys.push(KeyCode::Alpha).unwrap();
    keys.push(KeyCode::Division).unwrap();
    keys.push(KeyCode::Cosine).unwrap();
    keys.push(KeyCode::Alpha).unwrap();
    keys.push(KeyCode::Square).unwrap();

    let mut result: Vec<TextInput, 10> = Vec::new();
    for key in keys {
        let input = context.text_from_event(&Event::KeyDown(key));
        result.push(input).unwrap();
    }
    let mut expected: Vec<TextInput, 10> = Vec::new();

    expected.push(TextInput::Text("6")).unwrap();
    expected.push(TextInput::None).unwrap();
    expected.push(TextInput::Text("}")).unwrap();
    expected.push(TextInput::None).unwrap();
    expected.push(TextInput::None).unwrap();
    expected.push(TextInput::None).unwrap();
    expected.push(TextInput::Text("V")).unwrap();
    expected.push(TextInput::Text("H")).unwrap();
    expected.push(TextInput::None).unwrap();
    expected.push(TextInput::Text("^2")).unwrap();

    assert_eq!(result, expected);
}
