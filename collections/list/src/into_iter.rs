//! # `List`를 위한 소유 반복자
//!
//! 이 반복자는 `List`의 `into_iter` 메소드에 의해 생성됩니다.

use super::List;
use crate::node::*;
use std::boxed::Box;
use std::iter::{ExactSizeIterator, FusedIterator, Iterator};
use std::ptr::NonNull;

/// # `List`를 위한 소유 반복자
///
/// 이 구조체는 `List`의 `into_iter` 메소드에 의해 생성됩니다.
/// 자세한 내용은 해당 문서를 참조하세요.
///
/// # 제네릭
///
/// * `T`: 반복자가 반환할 요소의 타입입니다.
///
/// # 필드
///
/// * `source`: 현재 노드를 가리키는 포인터입니다.
/// * `len`: 반복자에 남아있는 요소의 수입니다.
#[derive(Debug)]
pub struct IntoIter<T> {
    source: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> IntoIter<T> {
    /// # `List`로부터 새로운 `IntoIter`를 생성합니다.
    ///
    /// 이 함수는 `List`와 그 노드들의 소유권을 가져옵니다.
    /// 이중 해제를 방지하기 위해, 원래의 `List`는 비워집니다.
    ///
    /// # 인수
    ///
    /// * `list`: 소유권을 가져올 `List`입니다.
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

    /// # 반복자를 진행시키고 다음 값을 반환합니다.
    ///
    /// 노드를 소비하고 데이터를 반환합니다.
    ///
    /// # 반환값
    ///
    /// 다음 요소를 담은 `Option<T>`입니다.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur_node) = self.source {
            self.source = Node::get_next_node(cur_node);
            self.len -= 1;
            // consume node here
            unsafe { Some(Box::from_raw(cur_node.as_ptr()).get_data()) }
        } else {
            None
        }
    }

    /// # 반복자의 정확한 크기를 반환합니다.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len)) // exact size
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> FusedIterator for IntoIter<T> {}
