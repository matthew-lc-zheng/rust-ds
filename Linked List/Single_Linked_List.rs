use std::fmt::Display;

// fn main() {
//     let mut head = _list_from(vec![1, 2, 3, 4, 5]);
//     _list_traverse(&head, true);
//     head = _list_reverse(head);
// 
//     _list_traverse(&head, true);
// }

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

fn _list_traverse<T: Display>(head: &Option<Box<Node<T>>>, show: bool) {
    let mut cnt = 0;
    let mut head_ = head.clone();
    while let Some(node) = head_ {
        if show {
            if node.next.is_none() { print!("{}", &node.val); } else { print!("{} -> ", &node.val); }
        }
        cnt += 1;
        head_ = &node.next;
    }
    println!("\nlength is {}", cnt);
}

fn _list_length<T>(head: &Option<Box<Node<T>>>) -> i32 {
    let mut cnt = 0;
    let mut head_ = head.clone();
    while let Some(node) = head_ {
        cnt += 1;
        head_ = &node.next;
    }
    cnt
}

fn _list_delete(head: &mut Option<Box<Node<T>>>,val:i32){
    
}

fn _list_delete_all(head: &mut Option<Box<Node<T>>>,val:i32){
    
}

fn _list_reverse<T>(mut head: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    // let mut head_ = Some(Box::new(head.as_ref().unwrap().val));
    // let mut  head_=Some(head.unwrap().clone());
    let mut pre_head: Option<Box<Node<T>>> = None;
    while let Some(mut node) = head.take() {
        head = node.next.take();
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




