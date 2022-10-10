//! A LinkedList implementation using enums and Boxes.

#![allow(dead_code, unused_variables)]

use std::collections::VecDeque;

fn main() {
    let v = vec![1, 2, 3, 4];
    let a: LinkedList<_> = v.iter().collect();

    for i in &a {
        println!("{i}");
    }
}

#[derive(Debug)]
enum LinkedList<T> {
    Empty,
    WithData(T, Box<LinkedList<T>>),
}

impl<T> LinkedList<T> {
    pub fn empty() -> LinkedList<T> {
        LinkedList::Empty
    }

    pub fn append(head: T, tail: LinkedList<T>) -> LinkedList<T> {
        LinkedList::WithData(head, Box::new(tail))
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, LinkedList::Empty)
    }

    pub fn head(&self) -> &T {
        if let LinkedList::WithData(head, _) = self {
            head
        } else {
            panic!("This linked list is empty.")
        }
    }

    pub fn tail(&self) -> &LinkedList<T> {
        if let LinkedList::WithData(_, tail) = self {
            &**tail
        } else {
            panic!("This linked list is empty.")
        }
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut vec = VecDeque::from([]);

        for item in iter {
            vec.push_front(item);
        }

        let mut list = LinkedList::<T>::Empty;

        for item in vec {
            list = LinkedList::WithData(item, Box::new(list));
        }

        list
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = LLRefIter<'a, T>;

    fn into_iter(self) -> LLRefIter<'a, T> {
        LLRefIter { list: self }
    }
}

struct LLRefIter<'a, T> {
    list: &'a LinkedList<T>,
}

impl<'a, T> Iterator for LLRefIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let LinkedList::WithData(head, tail) = self.list {
            self.list = &*tail;
            Some(head)
        } else {
            None
        }
    }
}
