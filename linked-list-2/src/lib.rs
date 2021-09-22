
// like a node with something inside of it or not inside of it.
// see below a linked list named NODE 
// recursivly, everything is stack aloctade 
// node = generic on this liftime a >>> node is constrained as some life time a is hover long the reference lives for
// its also possible to use static 
// enum Node<'a>{
//     // the node can be empty
//     Empty, 
//     // or is nonempty.   depending on your machine 
//     // &Node borrowing the value -> ownes the node, how long does this thing live for, prove to compiler that it lives 
//     // shorter amount of time 'a is a life time call it anything execpt for static. 
//     // box is an owned pointer  
//     NonEmpty(u32, &'a Node<'a>),

// }


enum Node{
    // the node can be empty
    Empty, 
    // or is nonempty.   depending on your machine 
    // &Node borrowing the value -> ownes the node, how long does this thing live for, prove to compiler that it lives 
    // shorter amount of time 'a is a life time call it anything execpt for static. 
    // box is an owned pointer  
    NonEmpty(u32, box<Node>),

}




#[cfg(test)]
mod tests {
    use super::*;
    fn it_works() {
        // we create a node, how much space will it take on the stack
        //let node = node::nonEmpty(1, Node::nonEmpty);
        // we need to explain node is going to life for some time // we borrow it with &
        //let node: &Node = &Node::Empty;
        let list: Node = Node::NonEmpty(1091, Box::new(Node::Empty));
    }
}

// we don't know how big it is, can be. 
// we cant do it on the stack, indirectly we need a pointer somewhere else so the node itself 
// is alsways a certain size, 
//fn my_func(node: Node){}



