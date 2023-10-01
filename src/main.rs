use std::collections::HashSet;
use rand::prelude::*;

fn main() {
}


fn get_balanced_tree(size: usize) -> Node{
    let mut memory = HashSet::<i64>::new();

    let rnd_root = rand::thread_rng().gen_range(i64::MIN..i64::MAX + 1);
    let mut root =  Node::new(rnd_root);

    let mut count: usize = 1; 
    while count < size{
        let rnd = rand::thread_rng().gen_range(i64::MIN..i64::MAX + 1);
        if memory.insert(rnd) {
            root.add(rnd);
            count += 1;
        }
    }

    root
}

fn get_skewed_tree(size: usize) -> Node{
    let mut root = Node::new(1);

    for i in 2..size{
        root.add(i as i64);
    }
    root
}

struct Node {
    value: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl Node {
    fn new(val: i64) -> Node{
        Node{
            value: val,
            left: None,
            right: None 
        }
    }
    fn add(&mut self, new_val: i64){
        if self.value == new_val {
            return;
        }

        let left_or_right = if new_val > self.value { &mut self.right } else { &mut self.left };

        match left_or_right {
            &mut Some(ref mut child) => child.add(new_val),
            &mut None => {
                let new_node = Box::new(Node::new(new_val));
                *left_or_right = Some(new_node);
            }
        }

    }
}