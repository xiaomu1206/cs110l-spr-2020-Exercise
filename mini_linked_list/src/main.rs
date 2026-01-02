struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

struct Node {
    val: u32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: u32, next: Option<Box<Node>>) -> Node {
        Node { val: value, next }
    }
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList { head: None, size: 0 }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, value: u32) {
        let node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<u32> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.val)
    }

    pub fn display(&self) {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            print!("{} -> ", node.val);
            current = node.next.as_ref();
        }

        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();

    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);

    for i in 1..10 {
        list.push(i);
    }

    list.display();
    println!("list size: {}", list.get_size());

    if let Some(val) = list.pop() {
        println!("popped element: {}", val);
    }

    list.display();
}
