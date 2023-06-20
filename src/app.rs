//! This module defines the interface between an application and the OS

use crate::calculator::Calculator;

/// Trait that must be implemented by applications to be run by the OS.
pub trait App {
    fn new() -> Self;
    fn run(&mut self, calc: &mut Calculator);
}