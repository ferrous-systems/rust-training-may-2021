use std::sync::Arc;

struct Node {
    other_nodes: Vec<Arc<Node>>
}


fn main() {



    println!("Hello, world!");

    let mut one = Arc::new(Node {
        other_nodes: vec![],
    });

    let mut two = Arc::new(Node {
        other_nodes: vec![],
    });

    let mut three = Arc::new(Node {
        other_nodes: vec![],
    });

    // Each Arc has a count of one

    // NOTE: Psuedocode!
    one.other_nodes.push(two.clone());
    two.other_nodes.push(three.clone());
    three.other_nodes.push(one.clone());

    // Each Arc has a count of two

    drop(one);
    drop(two);
    drop(three);

    // BUT each Arc still has a count of 1
}

