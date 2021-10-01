

// 
#[derive(Debug)]
enum LL {
    Node(i32, Box<LL>),    // ll is an enum two values data, box pointer to the ll  -> box lives on the stack but they store  an address to an object on the heap
    Tail,                  // an LL is either a  Node or last 
}

use LL::{Node, Tail};
// implement in LL the function create 
// which takes a type of 32 
// points to the linked list which we use self for 
impl LL{ 
    fn create(elem: i32)->Self{  
        // create a linked list and return it back. 
        Node(elem, Box::new(Tail) ) //  first elem and second is the box which we point to the last which is the second...  Last-> heap 
        // we put NO ; so it returns back to  in this case (main)
    }

    //self- linkedlist
    fn addNode(&mut self, elem: i32 ){
        // run the match operator on the self.
        match self{
            Node(a, b) => { b.addNode(elem);}    //need to check if the self is the first node not the last , recursive call
            Tail => {*self = LL::create(elem);} //replace the last witit // take the self which is a mutable reference so  we need to dereference it 
        }

    }
}

fn main() {

    let mut ll = LL::create(4);
    ll.addNode(3);
    ll.addNode(4);
    ll.addNode(5);
    ll.addNode(6);

    println!("{:?}", ll);

}

