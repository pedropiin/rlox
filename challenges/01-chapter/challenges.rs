use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

struct Node {
    content: String,
    next: Option<Rc<RefCell<Node>>>,
    previous: Option<Rc<RefCell<Node>>>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

struct DoublyLinkedList {
    first: Option<Rc<RefCell<Node>>>,
    size: usize,
}

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList {
            first: None,
            size: 0
        }
    }

    fn insert_end(&mut self, new_str: String) -> () {
        // let new_node: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
        //     content: new_str,
        //     next: None,
        //     previous: match &self.last {
        //         Some(rc) => Some(Rc::clone(&rc)),
        //         None     => None,
        //     }
        // }));
        // self.last = Some(Rc::clone(&new_node));
        // if self.size == 0 {
        //     self.first = Some(Rc::clone(&new_node));
        // }
        // self.size += 1;
        todo!();
    }

    fn insert_beginning(&mut self, new_str: String) -> () {
        let new_node_content: Node = 
            if self.size == 0 {
                Node {
                    content: new_str, 
                    next: None,
                    previous: None
                }
            } else {
                Node {
                    content: new_str,
                    next: match &self.first {
                        Some(rc) => Some(Rc::clone(&rc)),
                        None     => None,
                    },
                    previous: None
                }
            };

        let new_node: Rc<RefCell<Node>> = Rc::new(RefCell::new(new_node_content));
        self.first = Some(Rc::clone(&new_node));
        self.size += 1;
    }

    fn find(&self, key: String) -> () {
        todo!();
    }

    fn delete(&self, key: String) -> () {
        todo!();
    }
}

fn main() {
    println!("Hello World!");

    let mut linked_list: DoublyLinkedList = DoublyLinkedList::new();
    // linked_list.insert_end(String::from("Testing!"));
    // match &linked_list.last {
    //     Some(rc) => println!("last value from linked list == {}", *rc.borrow()),
    //     None     => println!("nothing on the end of the list."),
    // };
    linked_list.insert_beginning(String::from("first string in the linked list!"));
    match &linked_list.first {
        Some(rc) => println!("1st == {}", *rc.borrow()),
        None     => println!("Empty list."),
    }
}