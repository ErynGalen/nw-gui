//! Types and traits to store collections of widgets used by the GUI.

use super::Widget;
use heapless::Vec;

use either::Either;

/// Types implementing this trait can be used as storage for many widgets.
///
/// Widget containers should use this trait to store their children.
/// # Example
/// ```
/// struct Container<C: WidgetCollection> {
///     children: C,
/// }
/// impl Container {
///     fn number_of_children(&self) -> usize {
///         self.children.len()
///     }
/// }
///
/// ```
pub trait WidgetCollection {
    type Item: Widget;

    /// Number of widgets in the collection.
    fn len(&self) -> usize;
    /// Read-only access to the n-th widget in the collection.
    fn get(&self, n: usize) -> Option<&Self::Item>;
    /// Mutable access to the n-th widget in the collection.
    fn get_mut(&mut self, n: usize) -> Option<&mut Self::Item>;
    /// Add a widget to the collection.
    /// Return the widget back if it can't be added.
    fn add_widget(&mut self, widget: Self::Item) -> Result<(), Self::Item>;
}

/// Vec of a uniform type can be used as a WidgetCollection.
impl<T: Widget, const N: usize> WidgetCollection for Vec<T, N> {
    type Item = T;

    // the double dereference to transform &Vec -> Vec -> &[T]
    // forces the use of the underlying methods
    fn len(&self) -> usize {
        (**self).len()
    }
    fn get(&self, n: usize) -> Option<&Self::Item> {
        (**self).get(n)
    }
    fn get_mut(&mut self, n: usize) -> Option<&mut Self::Item> {
        (**self).get_mut(n)
    }

    fn add_widget(&mut self, widget: Self::Item) -> Result<(), Self::Item> {
        self.push(widget)
    }
}

/// A Vec of Either can be used as a widget collection.
impl<L: Widget, R: Widget> Widget for Either<L, R> {
    fn on_event(&mut self, e: crate::calculator::Event) -> Option<crate::calculator::Event> {
        either::for_both!(self, w => w.on_event(e))
    }
    fn render(&self, target: &mut crate::calculator::DeviceDislay, focused: bool) {
        either::for_both!(self, w => w.render(target, focused))
    }
    fn set_bounding_box(&mut self, bounding_box: embedded_graphics::primitives::Rectangle) {
        either::for_both!(self, w => w.set_bounding_box(bounding_box))
    }
}
