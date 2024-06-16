use std::{cell::RefCell, rc::Rc};

type NodeType = Option<Rc<RefCell<Node>>>;

struct Node {
    data: i32,
    prev: NodeType,
    next: NodeType,
}
impl Node {
    fn new(data: i32) -> Self {
        Self {
            data,
            prev: None,
            next: None,
        }
    }
}

pub struct DoublyLinkedList {
    head: NodeType,
    tail: NodeType,
}

impl DoublyLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, data: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        if let Some(tail) = self.tail.as_ref() {
            tail.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(Rc::clone(tail));
            self.tail = Some(new_node);
        } else {
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        }
    }

    pub fn push_front(&mut self, data: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        if let Some(head) = self.head.as_ref() {
            head.borrow_mut().prev = Some(Rc::clone(&new_node));
            new_node.borrow_mut().next = Some(Rc::clone(head));
            self.head = Some(new_node);
        } else {
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        }
    }

    fn print_all(&self) {
        let mut cur_node = match self.head.as_ref() {
            Some(node) => Rc::clone(node),
            None => {
                return;
            }
        };

        loop {
            let cur = cur_node.clone();
            println!("data: {}", cur.borrow().data);
            cur_node = match cur.borrow().next.as_ref() {
                Some(node) => node.clone(),
                None => {
                    break;
                }
            };
        }
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.print_all();

    list.push_front(7);
    list.print_all();
}
