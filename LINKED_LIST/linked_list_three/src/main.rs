//reminder repeat / need to know by heart. 
//https://stackoverflow.com/questions/79923/what-and-where-are-the-stack-and-heap
//stack does needs to know size
//heap doesnt 

// 
#[derive(Debug)]
enum LL {
    Node(i32, Box<LL>),    // ll is an enum two values data, box pointer to the ll  -> placing the box on the heap pointer comes on the stack but its pointing to the heap
    Last,                  // an LL is either a  Node or last 
}

use LL::{Node, Last};
// implement in LL the function create 
// which takes a type of 32 
// points to the linked list which we use self for 
impl LL{ 
    fn create(elem: i32)->Self{  
        // create a linked list and return it back. 
        Node(elem, Box::new(Last) ) //  first elem and second is the box which we point to the last which is the second...  Last-> heap 
        // we put NO ; so it returns back to  in this case (main)
    }
}

fn main() {

    // we need to make it mutable ( changable)
    let mut ll =  LL::create(4);    // take a follow of 4 // single node contains 4 

    println!("{:?}", ll);
    println!("{:?}", LL::Last);

    // create a second node and add it to the list 
    // using match
    match &mut ll{
        //mut b cause it dereference 
        Node(a, b) => { **b = LL::create(5); }   // a = elem, b = box to ll // create a linked list // from create who returns a node  we want to replace the b second element with the current node 
        // b is LAST we only have 1 node 

        Last => {} // Last is a node wont be reached
    }
    println!("{:?}", ll);
}
