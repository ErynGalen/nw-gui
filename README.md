# nw-gui
Attempt at making a UI for Rustworks (https://github.com/nw-rs/rustworks)

The current goal is to create a GUI toolkit as well as an input event system.

### Current state
- The input event system uses directly the `Keycode` struct from SDL2
- The widget system doesn't support event dispatching, which it should