use std::iter::Peekable;

pub struct CondIterator<'a, T, F>
    where T: 'a, T: Iterator {
    iter: &'a mut Peekable<T>,
    filter: F,
}

impl<'a, T: Iterator, F> Iterator for CondIterator<'a, T, F>
    where F: FnMut(&T::Item) -> bool {

    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.peek() {
            Some(val) => {
                match (self.filter)(val) {
                    true => self.iter.next(),
                    _ => None
                }
            },
            None => None
        }
    }
}

pub trait CondTake<'a, T>
    where T: Iterator {
    fn take_conditional<F>(self, filter: F) -> CondIterator<'a, T, F> where F: FnMut(&T::Item) -> bool;
}

impl<'a, T> CondTake<'a, T> for &'a mut Peekable<T> where T: Iterator {
    fn take_conditional<F>(self, filter: F) -> CondIterator<'a, T, F> {
        CondIterator::<T, F> {
            iter: self,
            filter,
        }
    }
}
