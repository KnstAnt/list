use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    elem: Option<T>,
    next: Option<Rc<Box<Node<T>>>>,
}

#[derive(Debug)]
pub struct List<T> {
    head: Option<Rc<Box<Node<T>>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push(&mut self, value: T) {
        let mut next = Rc::new(Box::new(Node {
            elem: Some(value),
            next: None,
        }));

        if let Some(head) = self.head.take() {
            if let Some(node) = Rc::get_mut(&mut next) {
                node.next = Some(head);
            } else {
                panic!("error: can't push");
            }
        }

        self.head = Some(next);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(mut head) = self.head.take() {
            if let Some(head) = Rc::get_mut(&mut head) {
                self.head = head.next.take();
                return head.elem.take();
            } else {
                panic!("error: can't pop");
            }
        }

        None
    }

    pub fn iter(&self) -> NodeIter<T> {
        NodeIter {
            current: self.head.as_ref(),
        }
    }
}

pub struct NodeIter<'a, T> {
    current: Option<&'a Rc<Box<Node<T>>>>,
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            self.current = current.next.as_ref();
            return current.elem.as_ref();
        }

        None
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while !self.is_empty() {
            self.pop();
        }
    }
}
