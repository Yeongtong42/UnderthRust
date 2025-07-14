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

// data types
#[derive(Debug)]
pub struct List<T> {
    len: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

// private methods
impl<T> List<T> {
    fn alloc_node(data: T) -> NonNull<Node<T>> {
        let boxed_new_node = Box::new(Node::<T>::new(data));
        NonNull::new(Box::into_raw(boxed_new_node)).unwrap() // fuck
    }
    // fn free_node(node: NonNull<Node<T>>) {
    //     unsafe { drop(Box::from_raw(node.as_ptr())) }
    // }
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
    fn insert_when_empty(&mut self, node: NonNull<Node<T>>) {
        self.head = Some(node);
        self.tail = Some(node);
        self.len = 1;
    }
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
    pub fn new() -> Self {
        Self {
            len: 0usize,
            head: None,
            tail: None,
        }
    }
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
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0usize
    }

    // 삽입
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
    pub fn get_ref(&self, pos: usize) -> Result<&T, &str> {
        if let Some(node) = self.at(pos) {
            unsafe { Ok(node.as_ref().get_data_ref()) }
        } else {
            Err("Error : pos out of range.")
        }
    }
    pub fn get_mut_ref(&mut self, pos: usize) -> Result<&mut T, &str> {
        if let Some(mut node) = self.at(pos) {
            unsafe { Ok(node.as_mut().get_data_mut_ref()) }
        } else {
            Err("Error : pos out of range.")
        }
    }
    pub fn erase_at(&mut self, pos: usize) -> Result<(), &str> {
        let result = self.get(pos);
        if result.is_ok() {
            Ok(())
        } else {
            Err(result.err().unwrap())
        }
    }
    pub fn clear(&mut self) {
        // consume all node
        while self.pop_back().is_ok() {
            // do nothing
        }
    }

    // into_iter()
    // from_iter()
}

impl<'a, T> List<T> {
    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter::new(self)
    }

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
}
