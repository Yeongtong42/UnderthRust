use super::List;
use crate::node::*;
use std::iter::{FusedIterator, Iterator};
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Iter<'a, T> {
    source: Option<NonNull<Node<T>>>,
    len: usize,
    phantom: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iter<'a, T> {
    pub(super) fn new(list: &'a List<T>) -> Self {
        Iter {
            source: list.head,
            len: list.len,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur_node) = self.source {
            self.source = Node::get_next_node(cur_node);
            self.len -= 1;
            unsafe { Some(cur_node.as_ref().get_data_ref()) }
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len)) // exact size
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(cur_node) = self.source {
            self.source = Node::get_prev_node(cur_node);
            self.len += 1;
            unsafe { Some(cur_node.as_ref().get_data_ref()) }
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}

impl<T> FusedIterator for Iter<'_, T> {}
