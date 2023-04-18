#![no_std]

mod numworks_display;

mod gui;

mod nw_main;
use nw_main::nw_main;

fn main() {
    // setup some things here...

    // main() of the calculator
    nw_main();
}
