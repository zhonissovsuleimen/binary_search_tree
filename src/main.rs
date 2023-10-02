use std::collections::HashSet;
use rand::prelude::*;
use std::io::stdin;

fn main() {
    // Getting user input
    println!("Provide number of elements in the tree.");    
    let size: i64;
    loop{
        let mut input_str = String::new();
        stdin().read_line(&mut input_str).expect("Error reading console input");
        match input_str.trim().parse::<i64>(){
            Ok(input) => {
                size = input; 
                break;
            }
            Err(_) =>{
                println!("Input is incorrect. Please provide unassigned 32 bit integer.");
            }
        }
    }

    let skewed = get_skewed_tree(size);
    let balanced = get_balanced_tree(size);
}


fn get_balanced_tree(size: i64) -> Node{
    let mut memory = HashSet::<i64>::new();

    let rnd_root = rand::thread_rng().gen_range(i64::MIN..=i64::MAX);
    let mut root =  Node::new(rnd_root);

    let mut count: i64 = 1; 
    while count < size{
        let rnd = rand::thread_rng().gen_range(i64::MIN..=i64::MAX);
        if memory.insert(rnd) {
            root.add(rnd);
            count += 1;
        }
    }

    root
}

fn get_skewed_tree(size: i64) -> Node{
    let mut root = Node::new(1);

    for i in 2..=size{
        root.add(i);
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

    fn find(&self, target: i64) -> bool{
        let mut current_node = self;
        loop{
            let value = current_node.value;
            if target == value{
                return true;
            }

            let child_to_check = if target > value {&current_node.right} else {&current_node.left};
            match child_to_check {
                Some(child) => {current_node = child},
                None => return false
            } 
        } 
    }
}