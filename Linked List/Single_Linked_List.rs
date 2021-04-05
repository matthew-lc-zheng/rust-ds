// use std::fmt::Display;

use i32 as T;
/// *main for test* ///

fn main() {
    let mut head = make_list(&[1, 2, 3, 4, 5]);
    traverse(head.as_ref());
    println!("{}",length(head.as_ref()));
    head=reverse(head);
    traverse(head.as_ref());
}



fn make_list(arr: &[T]) -> Option<Box<Node>> {
    if arr.len()==0{
        return None;
    }
    Some(Box::new(Node{
        val: arr[0],
        next:make_list(&arr[1..])
    }))
}

fn traverse(head: Option<&Box<Node>>) {
    let mut cnt = 0;
    let mut node = head;
    while node.is_some(){
        cnt+=1;
        if node.unwrap().next.is_none() {
            print!("{}", &node.unwrap().val);
        } else {
            print!("{} -> ", &node.unwrap().val);
        }
        node=node.unwrap().next.as_ref();
    }
    println!("\nlength is {}", cnt);
}

fn length(head: Option<&Box<Node>>) -> i32 {
    let mut cnt = 0;
    let mut node = head;
    while node.is_some(){
        cnt+=1;
        node=node.unwrap().next.as_ref();
    }
    cnt
}

fn reverse(mut head: Option<Box<Node>>)->Option<Box<Node>> {
    let mut pre_head: Option<Box<Node>> = None;
    while let Some(mut node)=head.take() {
        head=node.next.take();
        node.next=pre_head.take();
        pre_head=Some(node);
    }
    pre_head
}


// Define node
struct Node {
    val: T,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: T) -> Self {
        Node {
            next: None,
            val,
        }
    }
}
