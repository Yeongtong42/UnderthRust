//! A doubly-linked list implementation.

mod into_iter;
mod iter;
mod iter_mut;
mod node;

use crate::into_iter::*;
use crate::iter::*;
use crate::iter_mut::*;
use crate::node::*;
use std::default::Default;
use std::ptr::NonNull;
use std::result::Result;

/// # 이중 연결 리스트
///
/// `List`는 이중 연결 리스트를 나타냅니다.
///
/// # 제네릭
///
/// * `T`: 리스트에 저장될 요소의 타입입니다.
///
/// # 필드
///
/// * `len`: 리스트의 길이입니다.
/// * `head`: 리스트의 첫 번째 노드를 가리키는 포인터입니다.
/// * `tail`: 리스트의 마지막 노드를 가리키는 포인터입니다.
#[derive(Debug)]
pub struct List<T> {
    len: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

// private methods
impl<T> List<T> {
    /// # 새 노드를 힙에 할당합니다.
    ///
    /// 이 함수는 `data`를 포함하는 새로운 `Node`를 힙에 할당하고,
    /// 그 노드를 가리키는 `NonNull` 포인터를 반환합니다.
    ///
    /// # 인수
    ///
    /// * `data`: 노드에 저장될 데이터입니다.
    ///
    /// # 반환값
    ///
    /// 새로 할당된 노드를 가리키는 `NonNull` 포인터입니다.
    fn alloc_node(data: T) -> NonNull<Node<T>> {
        let boxed_new_node = Box::new(Node::<T>::new(data));
        NonNull::new(Box::into_raw(boxed_new_node)).unwrap()
    }

    /// # 지정된 위치의 노드에 대한 포인터를 반환합니다.
    ///
    /// 이 함수는 리스트의 `pos` 위치에 있는 노드를 가리키는 `NonNull` 포인터를 반환합니다.
    /// `pos`가 범위를 벗어나면 `None`을 반환합니다.
    ///
    /// # 인수
    ///
    /// * `pos`: 찾고자 하는 노드의 위치입니다.
    ///
    /// # 반환값
    ///
    /// `pos` 위치의 노드를 가리키는 `Option<NonNull<Node<T>>>`입니다.
    fn at(&self, pos: usize) -> Option<NonNull<Node<T>>> {
        if pos >= self.len() {
            return None;
        }
        let mut target_node = self.head.unwrap();
        for _ in 0..pos {
            // O(N)
            target_node = Node::get_next_node(target_node).unwrap();
        }
        Some(target_node)
    }

    /// # 리스트가 비어 있을 때 노드를 삽입합니다.
    ///
    /// 이 함수는 리스트가 비어 있을 때 `node`를 첫 번째이자 마지막 노드로 설정합니다.
    ///
    /// # 인수
    ///
    /// * `node`: 삽입할 노드입니다.
    fn insert_when_empty(&mut self, node: NonNull<Node<T>>) {
        self.head = Some(node);
        self.tail = Some(node);
        self.len = 1;
    }

    /// # 리스트의 마지막 요소를 제거하고 반환합니다.
    ///
    /// 이 함수는 리스트에 요소가 하나만 있을 때 그 요소를 제거하고 데이터를 반환합니다.
    /// 리스트가 비어 있으면 에러를 반환합니다.
    ///
    /// # 반환값
    ///
    /// 제거된 요소의 데이터를 담은 `Result<T, &str>`입니다.
    fn pop_last(&mut self) -> Result<T, &str> {
        let result = if let Some(node) = self.head {
            unsafe { Ok((*Box::from_raw(node.as_ptr())).get_data()) }
        } else {
            Err("Error : pos out of range.")
        };
        self.head = None;
        self.tail = None;
        self.len = 0;
        result
    }
}

// public methods
impl<T> List<T> {
    /// # 비어 있는 `List`를 생성합니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let list = List::<i32>::new();
    /// assert!(list.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            len: 0usize,
            head: None,
            tail: None,
        }
    }

    /// # 슬라이스에서 `List`를 생성합니다.
    ///
    /// # 제네릭
    ///
    /// * `T`: `Clone` 트레이트를 구현해야 합니다.
    ///
    /// # 인수
    ///
    /// * `slice`: `List`를 생성하는 데 사용될 슬라이스입니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let list = List::from_slice(&[1, 2, 3]);
    /// assert_eq!(list.len(), 3);
    /// ```
    pub fn from_slice(slice: &[T]) -> Self
    where
        T: Clone,
    {
        let mut list = List::<T>::new();
        for data in slice {
            list.push_back(data.clone());
        }
        list
    }

    /// # 리스트의 요소 수를 반환합니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let mut list = List::new();
    /// list.push_back(1);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// # 리스트가 비어 있는지 확인합니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let list = List::<i32>::new();
    /// assert!(list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0usize
    }

    /// # 리스트의 맨 앞에 요소를 추가합니다.
    ///
    /// # 인수
    ///
    /// * `data`: 추가할 요소입니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let mut list = List::new();
    /// list.push_front(1);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn push_front(&mut self, data: T) {
        let new_node = List::alloc_node(data);
        if self.is_empty() {
            List::insert_when_empty(self, new_node);
        } else {
            Node::link_nodes(new_node, self.head.unwrap());
            self.head = Some(new_node);
            self.len += 1;
        }
    }

    /// # 리스트의 맨 뒤에 요소를 추가합니다.
    ///
    /// # 인수
    ///
    /// * `data`: 추가할 요소입니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let mut list = List::new();
    /// list.push_back(1);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn push_back(&mut self, data: T) {
        let new_node = List::alloc_node(data);
        if self.is_empty() {
            List::insert_when_empty(self, new_node);
        } else {
            Node::link_nodes(self.tail.unwrap(), new_node);
            self.tail = Some(new_node);
            self.len += 1;
        }
    }

    /// # 지정된 위치에 요소를 삽입합니다.
    ///
    /// # 인수
    ///
    /// * `data`: 삽입할 요소입니다.
    /// * `pos`: 요소를 삽입할 위치입니다.
    ///
    /// # 반환값
    ///
    /// 삽입에 성공하면 `Ok(())`를, `pos`가 범위를 벗어나면 `Err`를 반환합니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let mut list = List::from_slice(&[1, 3]);
    /// list.insert_at(2, 1).unwrap();
    /// assert_eq!(list.len(), 3);
    /// ```
    pub fn insert_at(&mut self, data: T, pos: usize) -> Result<(), &str> {
        // push back
        match pos {
            0 => {
                self.push_front(data);
                Ok(())
            }
            other => {
                if other == self.len {
                    self.push_back(data);
                    return Ok(());
                }
                if let Some(target_node) = self.at(pos) {
                    let new_node = List::alloc_node(data);
                    Node::insert_before(new_node, target_node);
                    self.len += 1;
                    Ok(())
                } else {
                    Err("Error : out of range.")
                }
            }
        }
    }

    /// # 리스트의 첫 번째 요소를 제거하고 반환합니다.
    ///
    /// 리스트가 비어 있으면 `Err`를 반환합니다.
    ///
    /// # 반환값
    ///
    /// 제거된 요소의 데이터를 담은 `Result<T, &str>`입니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let mut list = List::from_slice(&[1, 2]);
    /// assert_eq!(list.pop_front(), Ok(1));
    /// assert_eq!(list.pop_front(), Ok(2));
    /// assert!(list.pop_front().is_err());
    /// ```
    pub fn pop_front(&mut self) -> Result<T, &str> {
        if self.len == 0 {
            Err("Error : out of range.")
        } else if self.len == 1 {
            self.pop_last()
        } else {
            let new_head = Node::get_next_node(self.head.unwrap());
            let result = unsafe { Ok((*Box::from_raw(self.head.unwrap().as_ptr())).get_data()) };
            self.head = new_head;
            self.len -= 1;
            result
        }
    }

    /// # 리스트의 마지막 요소를 제거하고 반환합니다.
    ///
    /// 리스트가 비어 있으면 `Err`를 반환합니다.
    ///
    /// # 반환값
    ///
    /// 제거된 요소의 데이터를 담은 `Result<T, &str>`입니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let mut list = List::from_slice(&[1, 2]);
    /// assert_eq!(list.pop_back(), Ok(2));
    /// assert_eq!(list.pop_back(), Ok(1));
    /// assert!(list.pop_back().is_err());
    /// ```
    pub fn pop_back(&mut self) -> Result<T, &str> {
        if self.len == 0 {
            Err("Error : out of range.")
        } else if self.len == 1 {
            self.pop_last()
        } else {
            let new_tail = Node::get_prev_node(self.tail.unwrap());
            let result = unsafe { Ok((*Box::from_raw(self.tail.unwrap().as_ptr())).get_data()) };
            self.tail = new_tail;
            self.len -= 1;
            result
        }
    }

    /// # 지정된 위치의 요소를 제거하고 반환합니다.
    ///
    /// `pos`가 범위를 벗어나면 `Err`를 반환합니다.
    ///
    /// # 인수
    ///
    /// * `pos`: 제거할 요소의 위치입니다.
    ///
    /// # 반환값
    ///
    /// 제거된 요소의 데이터를 담은 `Result<T, &str>`입니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let mut list = List::from_slice(&[1, 2, 3]);
    /// assert_eq!(list.get(1), Ok(2));
    /// assert_eq!(list.len(), 2);
    /// ```
    pub fn get(&mut self, pos: usize) -> Result<T, &str> {
        if pos == 0 {
            return self.pop_front();
        } else if pos == self.len - 1 {
            return self.pop_back();
        }
        // normal get
        self.len -= 1;
        if let Some(node) = self.at(pos) {
            unsafe { Ok((*Box::from_raw(node.as_ptr())).get_data()) }
        } else {
            Err("Error : pos out of range.")
        }
    }

    /// # 지정된 위치의 요소에 대한 참조를 반환합니다.
    ///
    /// `pos`가 범위를 벗어나면 `Err`를 반환합니다.
    ///
    /// # 인수
    ///
    /// * `pos`: 참조할 요소의 위치입니다.
    ///
    /// # 반환값
    ///
    /// 요소에 대한 참조를 담은 `Result<&T, &str>`입니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let list = List::from_slice(&[1, 2, 3]);
    /// assert_eq!(list.get_ref(1), Ok(&2));
    /// ```
    pub fn get_ref(&self, pos: usize) -> Result<&T, &str> {
        if let Some(node) = self.at(pos) {
            unsafe { Ok(node.as_ref().get_data_ref()) }
        } else {
            Err("Error : pos out of range.")
        }
    }

    /// # 지정된 위치의 요소에 대한 가변 참조를 반환합니다.
    ///
    /// `pos`가 범위를 벗어나면 `Err`를 반환합니다.
    ///
    /// # 인수
    ///
    /// * `pos`: 참조할 요소의 위치입니다.
    ///
    /// # 반환값
    ///
    /// 요소에 대한 가변 참조를 담은 `Result<&mut T, &str>`입니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let mut list = List::from_slice(&[1, 2, 3]);
    /// if let Ok(val) = list.get_mut_ref(1) {
    ///     *val = 42;
    /// }
    /// assert_eq!(list.get_ref(1), Ok(&42));
    /// ```
    pub fn get_mut_ref(&mut self, pos: usize) -> Result<&mut T, &str> {
        if let Some(mut node) = self.at(pos) {
            unsafe { Ok(node.as_mut().get_data_mut_ref()) }
        } else {
            Err("Error : pos out of range.")
        }
    }

    /// # 지정된 위치의 요소를 제거합니다.
    ///
    /// # 인수
    ///
    /// * `pos`: 제거할 요소의 위치입니다.
    ///
    /// # 반환값
    ///
    /// 제거에 성공하면 `Ok(())`를, `pos`가 범위를 벗어나면 `Err`를 반환합니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let mut list = List::from_slice(&[1, 2, 3]);
    /// list.erase_at(1).unwrap();
    /// assert_eq!(list.len(), 2);
    /// ```
    pub fn erase_at(&mut self, pos: usize) -> Result<(), &str> {
        let result = self.get(pos);
        if result.is_ok() {
            Ok(())
        } else {
            Err(result.err().unwrap())
        }
    }

    /// # 리스트의 모든 요소를 제거합니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let mut list = List::from_slice(&[1, 2, 3]);
    /// list.clear();
    /// assert!(list.is_empty());
    /// ```
    pub fn clear(&mut self) {
        // consume all node
        while self.pop_back().is_ok() {
            // do nothing
        }
    }
}

impl<'a, T> List<T> {
    /// # 리스트에 대한 반복자를 반환합니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let list = List::from_slice(&[1, 2, 3]);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&1));
    /// ```
    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter::new(self)
    }

    /// # 리스트에 대한 가변 반복자를 반환합니다.
    ///
    /// # 예제
    ///
    /// ```
    /// use list::List;
    ///
    /// let mut list = List::from_slice(&[1, 2, 3]);
    /// for val in list.iter_mut() {
    ///     *val *= 2;
    /// }
    /// assert_eq!(list.get_ref(0), Ok(&2));
    /// ```
    pub fn iter_mut(&'a mut self) -> IterMut<'a, T> {
        IterMut::new(self)
    }
}

// trait implementations
impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T> PartialEq for List<T>
where
    Node<T>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        let mut cur_my = self.head;
        let mut cur_other = other.head;
        while cur_my.is_some() {
            // synthetic sugar?
            unsafe {
                if cur_my.unwrap().as_ref() != cur_other.unwrap().as_ref() {
                    return false;
                }
                cur_my = Node::get_next_node(cur_my.unwrap());
                cur_other = Node::get_next_node(cur_other.unwrap());
            }
        }
        true
    }
}

impl<T> Eq for List<T>
where
    Node<T>: Eq,
    List<T>: PartialEq,
{
}

impl<'a, T> std::iter::IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

impl<'a, T> std::iter::IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(self)
    }
}

impl<T> std::iter::IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<T> std::iter::FromIterator<T> for List<T> {
    fn from_iter<Itr: IntoIterator<Item = T>>(iter: Itr) -> Self {
        let mut l = List::<T>::new();
        for data in iter {
            l.push_back(data);
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_node() {
        let mut l = List::<i32>::new();
        l.push_back(33);
        assert_eq!(l, List::<i32>::from_slice(&[33]));
    }

    #[test]
    fn test_multi_node() {
        let mut l = List::<i32>::new();

        l.push_back(42);
        l.push_back(33);
        l.push_back(21);
        assert_eq!(l.len(), 3);
        assert_eq!(l, List::<i32>::from_slice(&[42, 33, 21]));
    }
    #[test]
    fn test_size() {
        let mut l = List::<i32>::new();

        l.push_back(42);
        l.push_back(42);
        assert_eq!(l.len(), 2);
        assert_eq!(l.erase_at(0), Ok(()));
        assert_eq!(l.len(), 1);
        assert_eq!(l.erase_at(0), Ok(()));
        assert_eq!(l.len(), 0);
        assert!(l.is_empty());
    }
    #[test]
    fn test_get() {
        let mut l = List::<i32>::new();

        l.push_back(42);
        assert_eq!(l.get(0), Ok(42i32));
    }

    #[test]
    fn test_ref() {
        let l = List::<i32>::from_slice(&[1, 2, 3, 4, 5, 6, 7]);

        assert_eq!(*(l.get_ref(0).unwrap()), 1);
        assert_eq!(*(l.get_ref(1).unwrap()), 2);
        assert_eq!(*(l.get_ref(2).unwrap()), 3);
        assert_eq!(*(l.get_ref(3).unwrap()), 4);
        assert_eq!(*(l.get_ref(4).unwrap()), 5);
        assert_eq!(*(l.get_ref(5).unwrap()), 6);
        assert_eq!(*(l.get_ref(6).unwrap()), 7);
        assert!((l.get_ref(7)).is_err());
    }
    #[test]
    fn test_mut_ref() {
        let mut l = List::<i32>::from_slice(&[1, 2, 3, 4, 5, 6, 7]);

        assert_eq!(*(l.get_mut_ref(0).unwrap()), 1);
        assert_eq!(*(l.get_mut_ref(1).unwrap()), 2);
        assert_eq!(*(l.get_mut_ref(2).unwrap()), 3);
        assert_eq!(*(l.get_mut_ref(3).unwrap()), 4);
        assert_eq!(*(l.get_mut_ref(4).unwrap()), 5);
        assert_eq!(*(l.get_mut_ref(5).unwrap()), 6);
        assert_eq!(*(l.get_mut_ref(6).unwrap()), 7);
        assert!((l.get_mut_ref(7)).is_err());

        let mut_ref = l.get_mut_ref(0).unwrap();
        *mut_ref = 42;
        assert_eq!(*(l.get_ref(0).unwrap()), 42);
        let mut_ref2 = l.get_mut_ref(3).unwrap();
        *mut_ref2 = 44;
        assert_eq!(*(l.get_ref(3).unwrap()), 44);
    }
    #[test]
    fn test_insert() {
        let mut l = List::<i32>::from_slice(&[1, 2, 3, 4, 5, 6]);

        l.insert_at(0, 0).expect("FIRST INSERTION FAILED");
        assert_eq!(l, List::<i32>::from_slice(&[0, 1, 2, 3, 4, 5, 6]));

        l.insert_at(7, 7).expect("LAST INSERTION FAILED");
        assert_eq!(l, List::<i32>::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7]));

        for n in 0..=7 {
            l.insert_at(n, n as usize).expect("INSERTION FAILED");
        }
        assert_eq!(
            l,
            List::<i32>::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7])
        );
    }
    #[test]
    fn test_erase() {
        let mut l = List::<i32>::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7]);

        // erase head
        l.erase_at(0).expect("HEAD ERASE FAILED");
        assert_eq!(l, List::<i32>::from_slice(&[1, 2, 3, 4, 5, 6, 7]));

        // erase tail
        let last = l.len() - 1;
        l.erase_at(last).expect("TAIL ERASE FAILED");
        assert_eq!(l, List::<i32>::from_slice(&[1, 2, 3, 4, 5, 6]));

        // erase mid
        l.erase_at(3).expect("MID ERASE FAILED");
        assert_eq!(l, List::<i32>::from_slice(&[1, 2, 3, 5, 6]));
        l.erase_at(2).expect("MID ERASE FAILED");
        assert_eq!(l, List::<i32>::from_slice(&[1, 2, 5, 6]));

        // erase all
        l.clear();
        assert_eq!(l, List::<i32>::new());

        // push after erase all
        l.push_front(3);
        l.push_front(2);
        l.push_front(1);
        assert_eq!(l, List::<i32>::from_slice(&[1, 2, 3]));
    }

    #[test]
    fn test_iter() {
        let l = List::<i32>::from_slice(&[1, 2, 3, 4, 5]);
        let mut itr = l.iter();
        assert_eq!(*itr.next().unwrap(), 1);
        assert_eq!(*itr.next().unwrap(), 2);
        assert_eq!(*itr.next().unwrap(), 3);
        // let data = l.get(2); // with this line, must not compile. borrow check.
        assert_eq!(*itr.next().unwrap(), 4);
        assert_eq!(*itr.next().unwrap(), 5);
        assert_eq!(itr.next(), None);

        let mut cnt = 1;
        for i in l.iter() {
            assert_eq!(*i, cnt);
            cnt += 1;
        }

        let mut cnt = 3;
        for i in l.iter().map(|x| x * 3) {
            assert_eq!(i, cnt);
            cnt += 3;
        }
    }
    #[test]
    fn test_iter_mut() {
        let mut l = List::<i32>::from_slice(&[1, 2, 3, 4, 5]);
        let mut itr = l.iter_mut();
        assert_eq!(*itr.next().unwrap(), 1);
        assert_eq!(*itr.next().unwrap(), 2);
        assert_eq!(*itr.next().unwrap(), 3);
        // let data = l.get(2); // with this line, must not compile. borrow check.
        assert_eq!(*itr.next().unwrap(), 4);
        assert_eq!(*itr.next().unwrap(), 5);
        assert_eq!(itr.next(), None);

        let mut cnt = 1;
        for i in l.iter_mut() {
            assert_eq!(*i, cnt);
            cnt += 1;
        }

        let mut cnt = 3;
        l.iter_mut().for_each(|x| *x *= 3); // change content through IterMut
        for i in l.iter() {
            assert_eq!(*i, cnt);
            cnt += 3;
        }
    }
    #[test]
    fn test_into_iter_trait() {
        let mut l = List::<i32>::from_slice(&[1, 2, 3, 4, 5]);

        // test for &List
        let mut val = 1;
        for i in &l {
            assert_eq!(val, *i);
            val += 1;
        }
        // test for &mut List
        let mut val = 2;
        for i in &mut l {
            *i *= 2;
            assert_eq!(val, *i);
            val += 2;
        }
        // test for List
        let mut val = 2;
        for i in l {
            assert_eq!(val, i);
            val += 2;
        }
    }
    #[test]
    fn test_from_iter() {
        let l: List<i32> = [1, 2, 3, 4, 5, 6, 7].into_iter().collect();
		assert_eq!(l, List::<i32>::from_slice(&[1, 2, 3, 4, 5, 6, 7]));
    }
}
