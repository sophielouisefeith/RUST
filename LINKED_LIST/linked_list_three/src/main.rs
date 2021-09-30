


// 
enum LL {
    Node(i32, Box<LL>),    // ll is an enum two values data, box pointer to the ll  -> placing the box on the heap pointer comes on the stack but its pointing to the heap
    Last,                  // an LL is either a  Node or last 
}



fn main() {





    println!("Hello, world!");
}
