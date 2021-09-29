
//mu When a variable does need to change its value during run-time, the mut keyword can be used to indicate that the variable is mutable: 

// building a single linked list. 

// a wrapper structure // acces the containers.// internal type is a vector.
// why 
// make it public
pub struct LinkedList<T>{
     
    head: Link<T>,

}

//these implementations wil me methods on linkedList 
impl LinkedList<u32>{}
//work only with linkedlist string
impl LinkedList<String>{

}
// linked list integration, 

// we have to first say we have a generic type t
impl <T>LinkedList<T>{

        // construction function . which by convention is called new 
        fn empty()-> LinkedList<T>{
            LinkedList { head:None}
        }

        fn push(&mut self, element: T){
            //todo!(" implement push !")
            //place holder std mem, none to our head. //

            let old_head: Option<Box<Node<T>>> = self.head.take();
            let new_head: Box<Node<T>> = Box::new(Node{
                element,
                next:old_head,

            });
            self.head = Some(new_head);
        }

        fn pop(&mut self) -> Option<T>{
            self.head.take().map(|n:Box<Node<T>>|{
                self.head = n.next;
                n.element
            });

        }
           
            // match self.head{
            //     None =>{ self.head = Some(Box::new(Node{
            //         element, 
            //         next:None,
            //     }))
            // }
            // Some(n: Box<Node>) => {
            //     let new_head: option<Box<Node>> = Some(Box::new (Node{
            //         element,
            //         next: Some(n);
            //     self.head = new_head;
            //     }));
            // }
//        }

//}
 
fn peek(&self)-> Option<&T>{
    self.head.as_ref().map(|n: &Box<Node<T>> | &n.element)
}

//to make t generic we need to say that node is generic over type t
struct Node<T>{

    //generic type t
    element: T,
    next: Link<T>,
}

// link is either empty, or it does excist.-> becomes an optinal type.
// none or some 
// we make link a type elias. option gives us the oppertunity to also use the standard libaries.
// and then we want to say link is also generic over some type t
type Link<T> = Option<Box<Node<T> >>;

// enum Link{
//     Empty,
//     nonRmpty(Box<Node>),

// }



#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn it_works(){
        // we can use Some and None from Option type
        let mut list: LinkedList = LinkedList::empty(); 
        list.push(element: 1024);
    }
    
}





// #[cfg(test)]
// mod tests{
//     use super::*;
//     #[test]
//     fn it_works(){
//         // we can use Some and None from Option type
//         let list: LinkedList = LinkedList { 
//             head: Some(Box::new(Node {
//             element:1024, 
//             next:None,
//         })),
//     };
//     }
// }












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


//rapper struct
// struct linkedList{
//     head: link,
// }

// enum Node{
//     // the node can be empty
//     element: i32,
//     // or is nonempty.   depending on your machine 
//     // &Node borrowing the value -> ownes the node, how long does this thing live for, prove to compiler that it lives 
//     // shorter amount of time 'a is a life time call it anything execpt for static. 
//     // box is an owned pointer  
//     next: link;
// }

// //std libary
// type link = option<Box<Node>>;

// enum list{
//     Empty,
//     Link(Box<Node>)

// }


// #[cfg(test)]
// mod tests {
//     use super::*;
//     fn it_works() {
//         let list: link = list::link(Box::new(Node){element:1024,
//              next:link:Empty,})
//     }
// }







// we don't know how big it is, can be. 
// we cant do it on the stack, indirectly we need a pointer somewhere else so the node itself 
// is alsways a certain size, 
//fn my_func(node: Node){}



