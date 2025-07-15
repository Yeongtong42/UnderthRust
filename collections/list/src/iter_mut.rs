//! # `List`의 항목에 대한 가변 반복자
//!
//! 이 반복자는 `List`의 `iter_mut` 메소드에 의해 생성됩니다.

use super::List;
use crate::node::*;
use std::iter::{ExactSizeIterator, FusedIterator, Iterator};
use std::marker::PhantomData;
use std::ptr::NonNull;

/// # `List`의 항목에 대한 가변 반복자
///
/// 이 구조체는 `List`의 `iter_mut` 메소드에 의해 생성됩니다.
/// 자세한 내용은 해당 문서를 참조하세요.
///
/// # 제네릭
///
/// * `'a`: 반복자의 라이프타임입니다.
/// * `T`: 반복자가 반환할 요소의 타입입니다.
///
/// # 필드
///
/// * `source`: 현재 노드를 가리키는 포인터입니다.
/// * `len`: 반복자에 남아있는 요소의 수입니다.
/// * `phantom`: 라이프타임 `'a`를 사용하기 위한 팬텀 데이터입니다.
#[derive(Debug)]
pub struct IterMut<'a, T> {
    source: Option<NonNull<Node<T>>>,
    len: usize,
    phantom: PhantomData<&'a mut Node<T>>,
}

impl<'a, T> IterMut<'a, T> {
    /// # `List`로부터 새로운 `IterMut`를 생성합니다.
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

    /// # 반복자를 진행시키고 다음 값을 반환합니다.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut cur_node) = self.source {
            self.source = Node::get_next_node(cur_node);
            self.len -= 1;
            unsafe { Some(cur_node.as_mut().get_data_mut_ref()) }
        } else {
            None
        }
    }

    /// # 반복자의 정확한 크기를 반환합니다.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len)) // exact size
    }
}

impl<T> DoubleEndedIterator for IterMut<'_, T> {
    /// # 반복자를 뒤에서부터 진행시키고 다음 값을 반환합니다.
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
