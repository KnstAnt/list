use std::rc::{Rc, Weak};


#[derive(Debug)]
struct Node <T> {
    elem: Option<T>,
    next: Option< Rc<Box<Node<T>>> >,
    last: Option< Weak<Box<Node<T>>> >,
}

#[derive(Debug)]
pub struct List <T> {
    head: Option< Rc<Box<Node<T>>> >,
    tail: Option< Weak<Box<Node<T>>> >,
//    size: usize,
}


impl <T> List<T> {
    pub fn new() -> Self {
        List { 
            head: None, 
            tail: None, 
  //          size: 0,
        }
    }

    pub fn push_front(&mut self, value: T ) {

        let node = Node { elem: Some(value), next: None, last: None };

        let mut next = Rc::new( Box::new( node ) );

        if let Some(mut head) = self.head.take() {

        //    let next_link = Some(Rc::downgrade(&next)); 
 
            if let Some(head) =  Rc::get_mut(&mut head) {
                head.last =  Some(Rc::downgrade(&next));
            } else {
                panic!("error: can't push_front");
            }

            if let Some(node) =  Rc::get_mut(&mut next) {
                if self.tail.is_none() {
                    self.tail = Some( Rc::downgrade(&head) );
                }

                node.next =  Some(head);
            } else {
                panic!("error: can't push_front");
            }
        }

/*         if self.size == 1 {
            self.tail = Some( Rc::downgrade(&node.next.unwrap()) );
        }

        if self.tail.is_none() {
            self.tail = Some( Rc::downgrade(&next) );
        } */

        self.head = Some(next);  
        
   //     self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {

        if let Some(mut head) = self.head.take() {
            if let Some(head) =  Rc::get_mut(&mut head) {
                self.head =  head.next.take();
                return head.elem.take();
            } else {
                panic!("error: can't get head of list");
            }
        }

        None
    } 

/*    pub fn push_back(&mut self, value: T) {
        if let Some(mut tail) = self.tail {
            self.tail = Some( Rc::new( Box::new(Node { elem: value, next: None, last: self.tail }) ) );
            tail.next = self.tail;
            self.size += 1;
        } else {
            self.push_front(value);
            return;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if let Some(tail) = self.tail {
            self.tail = tail.last.clone();

            if self.tail.is_none() {
                self.head = None;
            }     

            self.size -= 1;

            return Some(tail.elem);
        }

        None
    } */
}