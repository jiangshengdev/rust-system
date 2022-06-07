use alloc::boxed::Box;
use core::mem;

#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

impl Default for Link {
    fn default() -> Self {
        Link::Empty
    }
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let head = mem::take(&mut self.head);
        let new_node = Box::new(Node { elem, next: head });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let head = mem::take(&mut self.head);

        match head {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::take(&mut self.head);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::take(&mut boxed_node.next);
        }
    }
}

pub fn basics() {
    let mut list = List::new();

    let list_size = mem::size_of::<List>();
    let link_size = mem::size_of::<Link>();
    let node_size = mem::size_of::<Node>();

    println!("{}", list_size);
    println!("{}", link_size);
    println!("{}", node_size);

    // Check empty list behaves right
    assert_eq!(list.pop(), None);

    println!("{:#?}", list);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    println!("{:#?}", list);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    println!("{:#?}", list);

    // Push some more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    println!("{:#?}", list);

    // Check normal removal
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    println!("{:#?}", list);

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);

    println!("{:#?}", list);
}
