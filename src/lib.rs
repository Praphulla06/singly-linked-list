use std::{rc::Rc, cell::RefCell};

#[derive(Debug, Clone)]
pub struct Node<T: Clone + Copy> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}
impl<T: Clone + Copy> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

#[derive(Debug, Clone)]
pub struct List<T: Clone + Copy> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}
impl<T: Clone + Copy> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None
        }
    }
    pub fn insert_at_head(&mut self, data: T) {
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
    pub fn insert_at_tail(&mut self, data: T) {
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
    pub fn delete_from_head(&mut self) -> Result<(), &'static str> {
        match self.head.take() {
            Some(old_head) => {
                self.head = old_head.borrow_mut().next.take();
                if self.head.is_none() {
                    self.tail = None;
                }
                Ok(())
            }
            None => Err("List is empty!")
        }
    }
    pub fn delete_from_tail(&mut self) -> Result<(), &'static str> {
        match &self.head {
            Some(head_node) => {
                if head_node.borrow().next.is_none() {
                    self.head = None;
                    self.tail = None;
                    return Ok(());
                }
                
                let mut temp = Rc::clone(&head_node);
                while temp.borrow().next.as_ref().unwrap().borrow().next.is_some() {
                    let next_node = Rc::clone(temp.borrow().next.as_ref().unwrap());
                    temp = next_node;
                }
                temp.borrow_mut().next = None;
                self.tail = Some(temp);
                Ok(())
            },
            None => Err("The list is empty!")
        }
    }
    pub fn traverse(&self) -> Result<Vec<T>, &'static str> {
        match &self.head {
            Some(current_node) => {
                let mut values: Vec<T> = Vec::new();
                let mut temp = Rc::clone(current_node);
                while temp.borrow().next.is_some() {
                    // print!("{} -> ", temp.borrow().data);
                    values.push(temp.borrow().data);
                    let next = Rc::clone(temp.borrow().next.as_ref().unwrap());
                    temp = next;
                }
                // println!("{} -> None.", temp.borrow().data);
                values.push(temp.borrow().data);
                Ok(values)
            },
            None => Err("The list is empty!")
        }
    }
}