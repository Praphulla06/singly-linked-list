use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct Node {
    data: i32,
    next: Option<Rc<RefCell<Node>>>,
}
impl Node {
    fn new(data: i32) -> Self {
        Self { data, next: None }
    }
}

#[derive(Debug, Clone)]
struct List {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}
impl List {
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    fn insert_at_head(&mut self, data: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        match self.head.take() {
            Some(old_head) => {
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }
    }
    fn insert_at_tail(&mut self, data: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }
    }
    fn delete_from_head(&mut self) {
        match self.head.take() {
            Some(old_head) => {
                self.head = old_head.borrow_mut().next.take();
                if self.head.is_none() {
                    self.tail = None;
                }
            }
            None => {
                println!("List is empty!");
                return;
            }
        }
    }
    fn delete_from_tail(&mut self) {
        let temp_head = Rc::clone(
            match &self.head {
                Some(node) => node,
                None => {
                    println!("List is empty!");
                    return;
                }
            }
        );
        match self.tail.take() {
            Some(old_tail) => {
                while temp_head.borrow_mut().next.is_some() {
                    let temp = temp_head.borrow_mut().next.clone();
                }
            },
            None => {}
        }
    }
}

fn main() {
    let mut l = List {
        head: None,
        tail: None,
    };
    l.insert_at_head(100);
    l.insert_at_head(200);
    l.insert_at_head(300);

    l.insert_at_tail(400);

    println!("{:#?}", l);
    l.delete_from_head();
    println!("{:#?}", l);
}
