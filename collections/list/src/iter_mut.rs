use super::List;
use crate::node::*;
use std::iter::Iterator;
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
            let result = unsafe { Some(cur_node.as_mut().get_data_mut_ref()) };
            self.source = Node::get_next_node(cur_node);
            self.len -= 1;
            result
        } else {
            None
        }
    }
}
