use core::marker::PhantomData;
use heapless::String;

pub struct TextBox<T, const N: usize> {
    content: String<N>,
    _context: PhantomData<T>,
}
