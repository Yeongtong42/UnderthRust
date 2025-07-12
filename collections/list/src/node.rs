use std::ptr::NonNull;

pub struct Node<T> {
    data: T, // maybe uninit
    next_node: Option<NonNull<Node<T>>>,
    prev_node: Option<NonNull<Node<T>>>,
}

// public
impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            next_node: None,
            prev_node: None,
        }
    }
    pub fn get_data(self) -> T {
        let mut temp = self;
        Node::splice_left_right(&mut temp);
        temp.data
    }
    pub fn get_data_ref(&self) -> &T {
        &self.data
    }
    pub fn get_data_mut_ref(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn link_nodes(mut left_node: NonNull<Node<T>>, mut right_node: NonNull<Node<T>>) {
        unsafe {
            left_node.as_mut().next_node = Some(right_node);
            right_node.as_mut().prev_node = Some(left_node);
        }
    }
    pub fn insert_before(single_node: NonNull<Node<T>>, mut chained_node: NonNull<Node<T>>) {
        // link prev of chain
        unsafe {
            if chained_node.as_mut().prev_node.is_some() {
                Node::link_nodes(chained_node.as_mut().prev_node.unwrap(), single_node);
            }
        }
        Node::link_nodes(single_node, chained_node);
    }
    pub fn get_next_node(cur_node: NonNull<Node<T>>) -> Option<NonNull<Node<T>>> {
        unsafe { cur_node.as_ref().next_node }
    }
    pub fn get_prev_node(cur_node: NonNull<Node<T>>) -> Option<NonNull<Node<T>>> {
        unsafe { cur_node.as_ref().prev_node }
    }
    pub fn unlink_next(mut node: NonNull<Node<T>>) {
        unsafe {
            node.as_mut().next_node = None;
        }
    }
    pub fn unlink_prev(mut node: NonNull<Node<T>>) {
        unsafe {
            node.as_mut().prev_node = None;
        }
    }
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
