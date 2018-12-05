extern crate slab;

use std::io;

use slab::Slab;
use std::fmt;
use std::ops::{Index, IndexMut};

pub fn read_all<T: std::str::FromStr>() -> Vec<T> {
    let mut xs = vec![];
    loop {
        let mut a_str = String::new();
        match io::stdin().read_line(&mut a_str) {
            Ok(x) => {
                if x == 0 {
                    break;
                }
            }
            Err(_) => break,
        }
        match a_str.trim().parse::<T>() {
            Ok(x) => xs.push(x),
            Err(_) => break,
        }
    }

    return xs;
}

// A doubly linked list.
pub struct List<T> {
    // All nodes get stored into this slab. A slab is basically just a
    // `Vec<Option<T>>` in disguse. We use it as a simple node allocator.
    pub slab: Slab<Node<T>>,
    // The head of the doubly linked list.
    pub head: Pointer,
    // The tail of the doubly linked list.
    pub tail: Pointer,
}

// A node in a doubly-linked list.
pub struct Node<T> {
    // The value stored in this node.
    pub value: T,
    // The next node in the list.
    pub next: Pointer,
    // The previous node in the list.
    pub prev: Pointer,
}

// A `Pointer` is just an index that refers to a node in the slab.
#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Pointer(pub usize);

impl Pointer {
    // The null pointer is `!0`, which is the largest possible value of type
    // `usize`. There's no way we'll ever have a legitimate index that large.
    #[inline]
    pub fn null() -> Pointer {
        Pointer(!0)
    }

    // Returns `true` if this pointer is null.
    #[inline]
    pub fn is_null(&self) -> bool {
        *self == Pointer::null()
    }
}

// Just for convenience, so that we can type `self[i]` instead of `self.slab[i]`.
impl<T> Index<Pointer> for List<T> {
    type Output = Node<T>;

    fn index(&self, index: Pointer) -> &Node<T> {
        &self.slab[index.0]
    }
}

// Just for convenience, so that we can type `self[i]` instead of `self.slab[i]`.
impl<T> IndexMut<Pointer> for List<T> {
    fn index_mut(&mut self, index: Pointer) -> &mut Node<T> {
        &mut self.slab[index.0]
    }
}

impl<T> List<T> {
    // Returns a new doubly linked list.
    pub fn new() -> List<T> {
        List {
            slab: Slab::new(),
            head: Pointer::null(),
            tail: Pointer::null(),
        }
    }

    // Inserts a new element at the back of the list.
    pub fn push_back(&mut self, t: T) -> Pointer {
        let tail = self.tail;
        if tail.is_null() {
            let n = Pointer(self.slab.insert(Node {
                value: t,
                prev: Pointer::null(),
                next: Pointer::null(),
            }));
            self.head = n;
            self.tail = n;
            n
        } else {
            self.insert_after(tail, t)
        }
    }

    // Inserts a new element at the front of the list.
    pub fn push_front(&mut self, t: T) -> Pointer {
        let head = self.head;
        if head.is_null() {
            self.push_back(t)
        } else {
            self.insert_before(head, t)
        }
    }

    // Inserts a new element after `node`.
    pub fn insert_after(&mut self, node: Pointer, t: T) -> Pointer {
        let next = self[node].next;
        let n = Pointer(self.slab.insert(Node {
            value: t,
            prev: node,
            next: next,
        }));

        if next.is_null() {
            self.tail = n;
        } else {
            self[next].prev = n;
        }
        self[node].next = n;
        n
    }

    // Inserts a new element before `node`.
    pub fn insert_before(&mut self, node: Pointer, t: T) -> Pointer {
        let prev = self[node].prev;
        let n = Pointer(self.slab.insert(Node {
            value: t,
            prev: prev,
            next: node,
        }));

        if prev.is_null() {
            self.head = n;
        } else {
            self[prev].next = n;
        }
        self[node].prev = n;
        n
    }

    // Removes `node` from the list and returns its value.
    pub fn remove(&mut self, node: Pointer) -> T {
        let prev = self[node].prev;
        let next = self[node].next;

        if prev.is_null() {
            self.head = next;
        } else {
            self[prev].next = next;
        }

        if next.is_null() {
            self.tail = prev;
        } else {
            self[next].prev = prev;
        }

        self.slab.remove(node.0).value
    }


}

impl<T: fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        let mut n = self.head;

        write!(f, "List(")?;
        while !n.is_null() {
            if !first {
                write!(f, ", ")?;
            }
            first = false;

            write!(f, "{:?}", self[n].value)?;
            n = self[n].next;
        }
        write!(f, ")")?;

        Ok(())
    }
}

//pub struct ListIterator<T> {
//    pub list: List<T>,
//    pub curr: Pointer,
//}
//
//impl<T> IntoIterator for List<T> {
//    type Item = T;
//    type IntoIter = ListIterator<T>;
//
//    fn into_iter(self) -> Self::IntoIter {
//        ListIterator{ list: self, curr: self.head }
//    }
//}
//
//impl<T> Iterator for ListIterator<T> {
//    type Item = T;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        None
//    }
//}

pub fn deep_copy<T>(xs: &List<T>) -> List<T> where T: Copy {
    let mut other = List::<T>::new();

    let mut n = xs.head;
    while !n.is_null() {
        other.push_back(xs[n].value);
        n = xs[n].next;
    }

    other
}
