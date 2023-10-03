use std::{collections::HashSet, time::Instant, thread};
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

    // Increase stack size for the thread that generates trees, as default stack size doesn't allow amount of nodes higher than ~10k
    let stack_size = 32 * 1024 * 1024; // 32 MB
    let handle = thread::Builder::new().stack_size(stack_size).spawn(move || {
        let skewed = get_skewed_tree(size);
        let balanced = get_balanced_tree(size);

        // Getting results
        let mut timer;
        let mut result;
        let to_find = size;

        //skewed
        timer = Instant::now();
        result = skewed.find(to_find);
        println!("Skewed BST, RESULT - {}:\n {} {} found,\t took {} micros\n", result, to_find, if result {"is"} else {"is not"}, timer.elapsed().as_micros());

        //balanced
        timer = Instant::now();
        result = balanced.find(size);
        println!("Balanced BST, RESULT - {}:\n {} {} found,\t took {} nanos\n", result, to_find, if result {"is"} else {"is not"}, timer.elapsed().as_nanos());
    });

    // Check if the thread creation was successful
    match handle {
        Ok(handle) => {
            // Wait for the thread to finish
            handle.join().unwrap();
        }
        Err(e) => println!("Error creating thread: {:?}", e),
    }
}


fn get_balanced_tree(size: i64) -> Node{
    let mut memory = HashSet::<i64>::new();

    let mut rnd = rand::thread_rng().gen_range(i64::MIN..=i64::MAX);
    let mut root =  Node::new(rnd);

    let mut count: i64 = 1; 
    while count < size{
        rnd = rand::thread_rng().gen_range(i64::MIN..=i64::MAX);
        if memory.insert(rnd) {
            root.insert_node_as_i64(rnd);
            count += 1;
        }
    }

    root
}

fn get_skewed_tree(size: i64) -> Node{
    let mut root = Box::new(Node::new(1));

    let mut latest_node = &mut root;
    for i in 2..=size{
        let new_node = Box::new(Node::new(i));
        latest_node.insert_node(*new_node);
        latest_node = latest_node.right.as_mut().unwrap();
    }
    *root
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
    
    fn insert_node_as_i64(&mut self, new_val: i64){
        if self.value == new_val {
            return;
        }

        let left_or_right = if new_val > self.value { &mut self.right } else { &mut self.left };

        match left_or_right {
            Some(ref mut child) => child.insert_node_as_i64(new_val),
            None => {
                let new_node = Box::new(Node::new(new_val));
                *left_or_right = Some(new_node);
            }
        }
    }

    fn insert_node(&mut self, node: Node){
        if self.value == node.value{
            return;
        }

        let left_or_right = if node.value > self.value { &mut self.right } else { &mut self.left };

        match left_or_right{
            &mut Some(ref mut child) => child.insert_node(node),
            None => {
                let new_ndoe = Box::new(node);
                *left_or_right = Some(new_ndoe);
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
