
// like a node with something inside of it or not inside of it.
// see below a linked list named NODE 
// recursivly, everything is stack aloctade 
enum Node{
    // the node can be empty
    Empty, 
    // or is nonempty.   depending on your machine 
    NonEmpty(u32, Box<Node>),

}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // we create a node, how much space will it take on the stack
        //let node = node::nonEmpty(1, Node::nonEmpty);
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// we don't know how big it is, can be. 
// we cant do it on the stack, indirectly we need a pointer somewhere else so the node itself 
// is alsways a certain size, 
fn my_func(node: Node){}



