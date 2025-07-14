use super::List;
use crate::node::*;
use std::boxed::Box;
use std::iter::{ExactSizeIterator, FusedIterator, Iterator};
use std::ptr::NonNull;

#[derive(Debug)]
pub struct IntoIter<T> {
    source: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> IntoIter<T> {
    pub(super) fn new(list: List<T>) -> Self {
        // move ownership of nodes to the iterator
        let result = IntoIter {
            source: list.head,
            len: list.len,
        };
        // to prevent double free, make list empty
        let mut list = list;
        list.head = None;
        list.tail = None;
        list.len = 0;
        result
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur_node) = self.source {
            self.source = Node::get_next_node(cur_node);
            self.len -= 1;
            unsafe { Some(Box::from_raw(cur_node.as_ptr()).get_data()) } // consume node here
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len)) // exact size
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> FusedIterator for IntoIter<T> {}
