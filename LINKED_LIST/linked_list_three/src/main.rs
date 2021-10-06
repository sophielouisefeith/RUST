/** references 
 * 
 * https://rust-unofficial.github.io/too-many-lists/second-final.html
 * https://doc.rust-lang.org/std/collections/struct.LinkedList.html
 * https://docs.rs/static-linkedlist/0.1.2/static_linkedlist/struct.StaticLinkedList.html#method.append
 */


 /** BOX:
  * when you dereference a BOX they unbox their addresses contents by following the pointer 
  */

  /* enum is a "sum type" -> a type that can have different values which may be different types */


// warning for value 


#[derive(Debug)]

enum LL<T>{
    None,
    Node {Value:T, next: Box<LL<T>>},    // ll is an enum two values data, box pointer to the ll  -> box lives on the stack but they store  an address to an object on the heap
    Tail {Value:T},                      // an LL is either a  Node or tail
   
}

//use LL::{Node, Tail};
// // implement in LL the function create 
// // which takes a generic T
// // points to the linked list which we use self for 

impl<T> LL<T> where T: Copy{ 
    // fn create(elem: i32)->Self{  
    //     // create a linked list and return it back. 
    //     Node(Node, Box::new(Tail) ) //  first elem and second is the box which we point to the last which is the second...  Last-> heap 
    //     // we put NO ; so it returns back to  in this case (main)
    // }
    
//     //self- linkedlist
//     fn addNode(&mut self, elem: i32 ){
//         // run the match operator on the self.
//         match self{
//             Node(a, b) => { b.addNode(elem);}    //need to check if the self is the first node not the last , recursive call
//             Tail => {*self = LL::create(elem);} //replace the last witit // take the self which is a mutable reference so  we need to dereference it 
//         }

//     }

    pub fn new()->Self{
        Self::None
    }

    pub fn push(&mut self, x: T){
        // compares self 
        match self{
            Self::None => { self.tail(x)},
            Self::Tail{..} => self.link(x),
            Self::Node{next, ..} => next.push(x)
        };
    }

    fn tail(&mut self, iterate : T){
        *self = match self{
            Self::None =>
                Self::Tail{Value: iterate},
            Self::Node{Value:_, next:_} =>
                Self::Tail { Value: iterate},
                _=> panic!("no tail")
                
        }
    }

    fn link(&mut self, x :T){
         *self = match self {
            Self::Tail{ Value} => {
                Self::Node{
                    Value: *Value,
                    next: Box::new(Self::Tail{ Value: x})
                }
            },

            
            _=> panic!("no link")
        }; //this returns 

    }
 }


#[cfg(test)]
mod test{

    // make a new list with new



}




fn main() {

    //let mut ll = LL::push(Self,4);
    let mut ll: LL<i32> = LL::new();
    // ll.addNode(3);
    // ll.addNode(4);
    // ll.addNode(5);
    // ll.addNode(6);

    ll.push(1);
    ll.push(2);
  
    println!("{:?}", ll);

}

