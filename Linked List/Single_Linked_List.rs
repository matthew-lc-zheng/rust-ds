fn _list_from<T>(mut arr: Vec<T>) -> Option<Box<Node<T>>> {
    arr.reverse();
    let mut head: Option<Box<Node<T>>> = None;
    for i in arr {
        let mut node: Box<Node<T>> = Box::from(Node::new(i));
        node.next = head;
        head = Some(node);
    }
    head
}

fn _list_traverse<T:Display+Debug>(mut head:Option<Box<Node<T>>>, show: bool) -> i32 {
    let mut cnt = 0;
    while let Some(mut node) = head {
        if show { print!("{:?} ", &node.val) }
        cnt += 1;
        head = node.next;
    }
    cnt
}

fn _list_reverse<T>(mut head:Option<Box<Node<T>>>)->Option<Box<Node<T>>> {
    let mut pre_head: Option<Box<Node<T>>> = None;
    while let Some(mut node) = head {
        head = node.next;
        node.next = pre_head;
        pre_head = Some(node);
    }
    pre_head
}




// Define node
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node {
            next: None,
            val,
        }
    }
}

