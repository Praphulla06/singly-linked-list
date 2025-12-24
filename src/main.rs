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
        match &self.head {
            None => {
                println!("The list is empty!");
                return;
            },
            Some(head_node) => {
                if head_node.borrow().next.is_none() {
                    self.head = None;
                    self.tail = None;
                    return;
                }
                
                let mut temp = Rc::clone(&head_node);
                while temp.borrow().next.as_ref().unwrap().borrow().next.is_some() {
                    let next_node = Rc::clone(temp.borrow().next.as_ref().unwrap());
                    temp = next_node;
                }
                temp.borrow_mut().next = None;
                self.tail = Some(temp);
            } 
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
    l.insert_at_tail(500);
    l.insert_at_tail(600);
    l.insert_at_tail(700);
    l.insert_at_head(800);

    // l.traverse(); 

    l.delete_from_head();
    // l.traverse(); 
    
    l.delete_from_tail();
    // l.traverse(); 
}
