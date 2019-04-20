use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    pub length: u64,
}

impl<T> LinkedList<T> {
    pub fn new_empty() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: T) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }

            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_string_to_empty_list() {
        let mut list = LinkedList::new_empty();
        list.append("Hello".to_string());
        list.append(",".to_string());
        list.append("LinkedList!".to_string());
        assert_eq!(list.length, 3);
        assert_eq!(list.pop(), Some("Hello".to_string()));
    }

    #[test]
    fn append_int32_to_empty_list() {
        let mut list = LinkedList::new_empty();
        list.append(1);
        list.append(-2);
        list.append(333);
        assert_eq!(list.length, 3);
        assert_eq!(list.pop(), Some(1));
    }
}
