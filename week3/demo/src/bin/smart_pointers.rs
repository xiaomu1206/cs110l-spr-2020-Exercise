/*  
1、Box<T>
Description：Have a unique pointer to a chunk of heap memory

2、Rc<T>
Description：Have multiple immutable references to a chunk of heap
Caution：if create reference cycles it will get memory leaks

3、RefCell<T>
*/
use std::{cell::RefCell, rc::{Rc, Weak}, usize};
/* doubly linked list */

type Link = Option<Rc<RefCell<Node>>>;

struct DoublyLinkedList {
    head: Link,
    tail: Link,
    size: usize
}
struct Node {
    val: u32,
    next: Link,
    pre: Option<Weak<RefCell<Node>>>
}

impl Node {
    fn new(val: u32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val,
            next: None,
            pre: None,
        }))
    }
}

impl DoublyLinkedList {
    pub fn new() -> DoublyLinkedList {
        DoublyLinkedList{head: None, tail: None, size: 0}
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn push_front(&mut self, val: u32) {
        let node = Node::new(val);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().pre = Some(Rc::downgrade(&node));
                node.borrow_mut().next = Some(old_head);
                self.head = Some(node);
            }
            None => {
                self.tail = Some(node.clone());
                self.head = Some(node);
            }
        }
        self.size += 1;
    }

    pub fn push_tail(&mut self, val: u32) {
        let node = Node::new(val);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().pre = Some(Rc::downgrade(&old_tail));
                self.tail = Some(node);
            }
            None => {
                self.tail = Some(node.clone());
                self.head = Some(node);
            }
        }
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<u32> {
        self.head.take().map(|old_head| {
            let mut old_head_ref = old_head.borrow_mut();

            match old_head_ref.next.take() {
                Some(next) => {
                    next.borrow_mut().pre = None;
                    self.head = Some(next);
                }
                None => {
                    self.tail = None;
                }
            }

            self.size -= 1;
            old_head_ref.val
        })
    }

    pub fn pop_tail(&mut self) -> Option<u32> {
        self.tail.take().map(|old_tail| {
            let mut old_tail_ref = old_tail.borrow_mut();

            match old_tail_ref.pre.take().and_then(|w| w.upgrade()) {
                Some(pre) => {
                    pre.borrow_mut().next = None;
                    self.tail = Some(pre);
                }
                None => {
                    self.head = None;
                }
            }

            self.size -= 1;
            old_tail_ref.val
        })
    }

    pub fn display(&self) {
        let mut cur = self.head.clone();
        while let Some(node) = cur {
            print!("{} <-> ", node.borrow().val);
            cur = node.borrow().next.clone();
        }
    }
}


fn main () {
    let mut list = DoublyLinkedList::new();

    list.push_front(2);
    list.push_front(1);
    list.push_tail(3);
    list.push_tail(4);

    println!("size: {}", list.get_size());
    list.display();// 1 <-> 2 <-> 3 <-> 4

    println!("pop front: {:?}", list.pop_front()); // 1
    println!("pop back: {:?}", list.pop_tail());   // 4

    list.display(); // 2 <-> 3
}