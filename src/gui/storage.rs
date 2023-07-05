//! Types and traits to store collections of widgets used by the GUI.

use super::Widget;
use heapless::Vec;

/// Types implementing this trait can be used as uniform storage for [`Widget`]s.
///
/// This means that only one widget type can be stored.
/// In order to store multiple widget types, it's possible to use an enum.
/// You can use [`either::Either`] for example, for which [`Widget`]
/// is already implemented.
///
/// Widget containers should use this trait to store their children.
///
/// See [`super::Widget`] for more information.
/// # Example
/// ```
/// use nw_gui::gui::storage::WidgetCollection;
///
/// struct Container<C: WidgetCollection> {
///     children: C,
/// }
/// impl<C: WidgetCollection> Container<C> {
///     fn number_of_children(&self) -> usize {
///         self.children.len()
///     }
///     fn add_child(&mut self, child: C::Item) -> Result<(), C::Item> {
///         self.children.add_widget(child)
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

/// [`Vec`](heapless::Vec) can be used as a WidgetCollection.
impl<U: Widget, const N: usize> WidgetCollection for Vec<U, N> {
    type Item = U;

    // the double dereference transforming &Vec -> Vec -> &[T]
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
