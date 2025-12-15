use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

struct Node {
    content: String,
    next: Option<Rc<RefCell<Node>>>,
    previous: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(content: String, next: Option<Rc<RefCell<Node>>>, previous: Option<Rc<RefCell<Node>>>) -> Self {
        Node { content: content, next: next, previous: previous }
    }
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
        if self.size == 0 {
            let new_node = Some(Rc::new(RefCell::new(Node::new(new_str, None, None))));
            self.first = new_node;
        } else {
            let mut curr_node: Option<Rc<RefCell<Node>>> = match self.first {
                Some(ref rc) => Some(Rc::clone(rc)),
                None         => None,
            };
            while let Some(ref rc) = curr_node.clone().unwrap().borrow().next {
                curr_node = Some(Rc::clone(rc));
            };
            let new_node = Some(Rc::new(RefCell::new(Node::new(new_str, None, curr_node.clone()))));
            curr_node.unwrap().borrow_mut().next = new_node;
        }
        self.size += 1;
    }

    fn insert_beginning(&mut self, new_str: String) -> () {
        let new_node_content: Node = 
            if self.size == 0 {
                Node::new(new_str, None, None)
            } else {
                Node::new(
                    new_str,
                    match &self.first {
                        Some(rc) => Some(Rc::clone(&rc)),
                        None     => None,
                    },
                    None
                )
            };

        let new_node: Rc<RefCell<Node>> = Rc::new(RefCell::new(new_node_content));
        self.first = Some(Rc::clone(&new_node));
        self.size += 1;
    }

    fn find(&self, key: String) -> () {
        fn find_aux(key: &String, node: &Option<Rc<RefCell<Node>>>) -> bool {
            match node {
                Some(rc) => {
                    if rc.borrow().content == *key {
                        true
                    } else {
                        find_aux(key, &(rc.borrow().next))
                    }
                }
                None      => false
            }
        }

        let found = find_aux(&key, &(self.first));
        if found { println!("The list contains the string '{}'.", key); }
        else { println!("The list doesn't contain the string '{}'.", key); };
    }

    fn delete(&self, key: String) -> () {
        fn delete_aux(key: &String, node: &Option<Rc<RefCell<Node>>>) -> () {
            match node {
                Some(rc) => {
                    if rc.borrow().content == *key {
                        let new_rc = Rc::clone(&rc);
                        new_rc.borrow_mut().previous = new_rc.borrow().next.clone();
                        return;
                    } else {
                        delete_aux(key, &(rc.borrow().next));
                    }
                }
                None     => return,
            }
        }

        delete_aux(&key, &(self.first));
    }
}

impl fmt::Display for DoublyLinkedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn fmt_aux(node: &Option<Rc<RefCell<Node>>>, elems: &mut Vec<String>) -> () {
            match node {
                Some(ref rc) => {
                    elems.push(rc.borrow().content.clone());
                    fmt_aux(&rc.borrow().next, elems)
                }
                None     => return,
            }
        }
        let mut elems: Vec<String> = Vec::new();
        fmt_aux(&self.first, &mut elems);
        for v in &elems {
            write!(f, "\t{}", v);
        }
        Ok(())
    }
}

fn main() {
    println!("Hello World!");

    let mut linked_list: DoublyLinkedList = DoublyLinkedList::new();

    linked_list.insert_beginning(String::from("FIRST STRING"));
    linked_list.insert_end(String::from("SECOND STRING"));

    println!("The full linked list is as: {}", linked_list);

    linked_list.find(String::from("Not in list"));
    linked_list.find(String::from("FIRST STRING"));
}