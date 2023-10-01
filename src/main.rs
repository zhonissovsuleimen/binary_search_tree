fn main() {
    let mut root = Node::new(0);

    for i in 1..10{
        root.add(i);
    } 
    
    let mut current_node = &root;
    loop {
        println!("{}", current_node.value);
        match &current_node.right{
            Some(right) => current_node = &right,
            None => break
        } 
    }
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