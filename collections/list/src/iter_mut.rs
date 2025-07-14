use super::List;
use crate::node::*;
use std::iter::{ExactSizeIterator, FusedIterator, Iterator};
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct IterMut<'a, T> {
    source: Option<NonNull<Node<T>>>,
    len: usize,
    phantom: PhantomData<&'a mut Node<T>>,
}

impl<'a, T> IterMut<'a, T> {
    pub(super) fn new(list: &'a mut List<T>) -> Self {
        IterMut {
            source: list.head,
            len: list.len,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut cur_node) = self.source {
            self.source = Node::get_next_node(cur_node);
            self.len -= 1;
            unsafe { Some(cur_node.as_mut().get_data_mut_ref()) }
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len)) // exact size
    }
}

impl<T> DoubleEndedIterator for IterMut<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(mut cur_node) = self.source {
            self.source = Node::get_prev_node(cur_node);
            self.len += 1;
            unsafe { Some(cur_node.as_mut().get_data_mut_ref()) }
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {}

impl<T> FusedIterator for IterMut<'_, T> {}
