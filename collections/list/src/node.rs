//! # 연결 리스트의 노드
//!
//! 이 모듈은 `List`에서 사용되는 `Node` 구조체와 관련 함수들을 포함합니다.

use std::ptr::NonNull;

/// # 연결 리스트의 노드
///
/// `Node`는 이중 연결 리스트의 개별 요소를 나타냅니다.
///
/// # 제네릭
///
/// * `T`: 노드에 저장될 데이터의 타입입니다.
///
/// # 필드
///
/// * `data`: 노드에 저장되는 데이터입니다.
/// * `next_node`: 다음 노드를 가리키는 포인터입니다.
/// * `prev_node`: 이전 노드를 가리키는 포인터입니다.
pub struct Node<T> {
    data: T,
    next_node: Option<NonNull<Node<T>>>,
    prev_node: Option<NonNull<Node<T>>>,
}

// public
impl<T> Node<T> {
    /// # 새 노드를 생성합니다.
    ///
    /// # 인수
    ///
    /// * `data`: 노드에 저장될 데이터입니다.
    ///
    /// # 반환값
    ///
    /// `data`를 포함하는 새로운 `Node`입니다.
    pub fn new(data: T) -> Self {
        Node {
            data,
            next_node: None,
            prev_node: None,
        }
    }

    /// # 노드를 소비하고 데이터를 반환합니다.
    ///
    /// 이 함수는 노드의 연결을 끊고, 노드가 소유한 데이터를 반환합니다.
    ///
    /// # 반환값
    ///
    /// 노드가 가지고 있던 데이터입니다.
    pub fn get_data(self) -> T {
        let mut temp = self;
        Node::splice_left_right(&mut temp);
        temp.data
    }

    /// # 노드의 데이터에 대한 참조를 반환합니다.
    ///
    /// # 반환값
    ///
    /// 노드의 데이터에 대한 참조입니다.
    pub fn get_data_ref(&self) -> &T {
        &self.data
    }

    /// # 노드의 데이터에 대한 가변 참조를 반환합니다.
    ///
    /// # 반환값
    ///
    /// 노드의 데이터에 대한 가변 참조입니다.
    pub fn get_data_mut_ref(&mut self) -> &mut T {
        &mut self.data
    }

    /// # 두 노드를 연결합니다.
    ///
    /// `left_node`의 `next_node`를 `right_node`로 설정하고,
    /// `right_node`의 `prev_node`를 `left_node`로 설정합니다.
    ///
    /// # 안전하지 않은 코드
    ///
    /// 이 함수는 `NonNull` 포인터를 역참조하므로 `unsafe` 블록이 필요합니다.
    ///
    /// # 인수
    ///
    /// * `left_node`: 연결할 왼쪽 노드입니다.
    /// * `right_node`: 연결할 오른쪽 노드입니다.
    pub fn link_nodes(mut left_node: NonNull<Node<T>>, mut right_node: NonNull<Node<T>>) {
        unsafe {
            left_node.as_mut().next_node = Some(right_node);
            right_node.as_mut().prev_node = Some(left_node);
        }
    }

    /// # 연결된 노드 앞에 단일 노드를 삽입합니다.
    ///
    /// `single_node`를 `chained_node` 앞에 삽입합니다.
    ///
    /// # 안전하지 않은 코드
    ///
    /// 이 함수는 `NonNull` 포인터를 역참조하므로 `unsafe` 블록이 필요합니다.
    ///
    /// # 인수
    ///
    /// * `single_node`: 삽입할 단일 노드입니다.
    /// * `chained_node`: `single_node`가 삽입될 위치의 노드입니다.
    pub fn insert_before(single_node: NonNull<Node<T>>, mut chained_node: NonNull<Node<T>>) {
        // link prev of chain
        unsafe {
            if chained_node.as_mut().prev_node.is_some() {
                Node::link_nodes(chained_node.as_mut().prev_node.unwrap(), single_node);
            }
        }
        Node::link_nodes(single_node, chained_node);
    }

    /// # 다음 노드를 반환합니다.
    ///
    /// # 안전하지 않은 코드
    ///
    /// 이 함수는 `NonNull` 포인터를 역참조하므로 `unsafe` 블록이 필요합니다.
    ///
    /// # 인수
    ///
    /// * `cur_node`: 현재 노드입니다.
    ///
    /// # 반환값
    ///
    /// 다음 노드를 가리키는 `Option<NonNull<Node<T>>>`입니다.
    pub fn get_next_node(cur_node: NonNull<Node<T>>) -> Option<NonNull<Node<T>>> {
        unsafe { cur_node.as_ref().next_node }
    }

    /// # 이전 노드를 반환합니다.
    ///
    /// # 안전하지 않은 코드
    ///
    /// 이 함수는 `NonNull` 포인터를 역참조하므로 `unsafe` 블록이 필요합니다.
    ///
    /// # 인수
    ///
    /// * `cur_node`: 현재 노드입니다.
    ///
    /// # 반환값
    ///
    /// 이전 노드를 가리키는 `Option<NonNull<Node<T>>>`입니다.
    pub fn get_prev_node(cur_node: NonNull<Node<T>>) -> Option<NonNull<Node<T>>> {
        unsafe { cur_node.as_ref().prev_node }
    }

    /// # 다음 노드와의 연결을 끊습니다.
    ///
    /// # 안전하지 않은 코드
    ///
    /// 이 함수는 `NonNull` 포인터를 역참조하므로 `unsafe` 블록이 필요합니다.
    ///
    /// # 인수
    ///
    /// * `node`: 연결을 끊을 노드입니다.
    pub fn unlink_next(mut node: NonNull<Node<T>>) {
        unsafe {
            node.as_mut().next_node = None;
        }
    }

    /// # 이전 노드와의 연결을 끊습니다.
    ///
    /// # 안전하지 않은 코드
    ///
    /// 이 함수는 `NonNull` 포인터를 역참조하므로 `unsafe` 블록이 필요합니다.
    ///
    /// # 인수
    ///
    /// * `node`: 연결을 끊을 노드입니다.
    pub fn unlink_prev(mut node: NonNull<Node<T>>) {
        unsafe {
            node.as_mut().prev_node = None;
        }
    }

    /// # 리스트에서 노드를 잘라냅니다.
    ///
    /// 이 노드의 이전 노드와 다음 노드를 직접 연결하여, 이 노드를 리스트에서 제거합니다.
    fn splice_left_right(&mut self) {
        // link prev and next of self
        if self.next_node.is_some() && self.prev_node.is_some() {
            Node::link_nodes(self.prev_node.unwrap(), self.next_node.unwrap());
        } else if self.next_node.is_some() {
            Node::unlink_prev(self.next_node.unwrap());
        } else if self.prev_node.is_some() {
            Node::unlink_next(self.prev_node.unwrap());
        }
    }
}

// partial eq
impl<T> PartialEq for Node<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

// eq
impl<T> Eq for Node<T>
where
    Node<T>: PartialEq,
    T: Eq,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_construct() {
        let node = Node::<i32>::new(44);
        assert_eq!(44, node.data);
    }
    #[test]
    fn test_node_link() {
        let node1 = Box::new(Node::<i32>::new(42));
        let node2 = Box::new(Node::<i32>::new(43));

        Node::link_nodes(
            NonNull::new(Box::leak(node1) as *mut Node<i32>).unwrap(),
            NonNull::new(Box::leak(node2) as *mut Node<i32>).unwrap(),
        );
    }
    #[test]
    fn test_node_get() {
        let node = Node::new(44);
        assert_eq!(44, Node::get_data(node));
    }
}
