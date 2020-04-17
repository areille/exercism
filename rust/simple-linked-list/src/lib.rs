use std::iter::FromIterator;
use std::mem;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut node = &self.head;
        let mut len = 0;
        while let Some(n) = node {
            len += 1;
            node = &n.next;
        }
        len
    }

    pub fn push<'a>(&'a mut self, _element: T) {
        let new_node = Box::new(Node {
            data: _element,
            next: mem::replace(&mut self.head, None),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let result: Option<T>;
        match mem::replace(&mut self.head, None) {
            None => {
                result = None;
            }
            Some(node) => {
                result = Some(node.data);
                self.head = node.next;
            }
        };
        result
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(&mut self) -> SimpleLinkedList<T> {
        // let mut prev = None::<T>;
        // let mut node = self.head;
        // let mut list = SimpleLinkedList::<T>::new();

        // while let Some(mut node_inner) = node {
        //     let next = node_inner.next;
        //     node_inner.next = prev;
        //     prev = Some(node_inner);
        //     node = next;
        // }
        // list
        unimplemented!();
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in _iter {
            list.push(i);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec: Vec<T> = vec![];
        while let Some(node) = &self.head {
            vec.push(node.data)
        }
        vec
    }
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node { data, next: None }
    }
}
