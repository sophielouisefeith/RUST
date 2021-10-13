/** linked list
 * following https://docs.rs/linked-list/0.0.3/linked_list/struct.LinkedList.html
 * make a new list
 * list contains a head {elem , next} // maybe add a  sep tail
 * pop a node to the front
 * pop a node to the back
 * pop a node from the front
 * pop a node from the back
 * insert a node - index
 * itterate
 */

extern crate colored; 

use colored::*;
use colored::*;

/** a struct list / generic type
 * the head : wiLinkedList be fiLinkedListed with link of a generic type
 */

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    count: i32, // counts the nodes/
}

/** for itteration */
pub struct Iter<'a,T>{
    next: Option <&'a Node<T>>
}

/** make a struct node which contains an element containing a generic T
 * and and a next which contains the link of a type t
 */
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>, //  an Option wrapped in a box.
    
}

/** implement Linkedlist */
impl<T: std::fmt::Debug> LinkedList<T> {
    // impl for ll with type t contrained to implements trait.

    /** make a new list
     * returns an empty list
     */
    pub fn new_list() -> Self {
        LinkedList {
            head: None,
            count: 0,
        }
    }

    /**
     * Push front/back
     * push a new node to the list
     * receives the list, it's going to do something with the list so we need to receive a &mut self -> self == the list
     * create a new node
     * pushes the element to the top of the head
     *
     */

    pub fn Push_front(&mut self, elem: T) {
        let new_node = Box::new(Node {
            // box sits on the stack  memory on the heap and then places struct node into it.
            elem: elem,
            next: self.head.take(), // next needs to point to the next element in this case head.
        });
        //fill head with new_node
        self.head = Some(new_node);
        self.count += 1;
    }

    /*******--------       --------        --------                ------------
     *      |  head o |       |  node 1 = 4|       |  node 2 tail |      node 3
     *      --------         ------        --------               -------------
     */// so we can find the tail if next is == to None  then we know we are the end/  or do we make a seperate tail
    pub fn Push_back(&mut self, elem: T) {
        let mut curr = &mut self.head;   // mut ref to an option

        while let Some(node) = curr {
            curr = &mut node.next;
        }
        *curr = Some(Box::new(Node { 
            elem, 
            next: None }));
        self.count += 1;
        // println!(" ha ");
        // // go to the last node and fill it's next with the new node.
        // // so i want to be at were next.None and replace this None with the new node,s element right ?
        // match &mut self.head {
        //     // taking a ref
        //     None => {
        //         let new_node = Box::new(Node { elem, next: None });
        //         self.head = Some(new_node);
        //     }
        //     Some(head) => {
        //         // new variable head -> compiler know this is box node. //pattern match
        //         let mut curr = &mut head.next; // mut assign it again.
        //         while let Some(node) = curr {
        //             // you know that its not because of the if// curr we want to unwrap?
        //             curr = &mut node.next; // value of last node with the reference to the next node //
        //         }
        //         let new_node = Box::new(Node { elem, next: None });
        //         *curr = Some(new_node);
        //     }
        // }
    }


  
    /*
    * Pop front/back
    * pop the value of the head from the list
    pop a value fromt  a list, we change the list her so we need to receive again a mut.
    map: in this case ( option) unwraps  -> needs to return an option -> and wraps the result
    Removes the first element and returns it, or None if the list is empty.
    */

    pub fn Pop_front(&mut self) -> Option<T> {
        // first we take the value of the elemt we want so
        // we have to take out an element and put aLinkedList the elemnts in the correct order?
        // need to unwrap here we can use map map unwraps and wraps the results
        self.count = -1;
        self.head.take().map(|Node| {
            // so the struct node has two elements
            // so we wiLinkedList take the value of what is in the head away therefore we need to replace it with what is in  next node.
            self.head = Node.next;
            // and then we return what is in the elem of the node
            Node.elem
        })
    }


    fn Pop_back(&mut self) -> Option<T>{  
        
        self.head.take().map(|head|{   

            let mut curr = head; // make it mutable cause otherwise it always will points to head. 
            while let Some(node) = curr.next{  // were walking the list.  till next points to none
                    curr = node;    
            }
            self.count -= 1;
            curr.elem
        })
    }
                                                                                                         
    /*Peek_front 
     * peek what is in front of the list.
     */

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem);
        panic!("link list is empty");
    }

    /** peek back 
     * 
    */
    pub fn peek_back(&self) -> Option<&T> { // receive self borrowed
            
            // map is a method on option (no if) map in option only maps when you have something you can use it as an if. 
            self.head.as_ref().map(|head|{   // value of the options as ref

                let mut curr = head; // make it mutable cause otherwise it always will points to head. 
                while let Some(node) = &curr.next{  // were walking the list.  
                        curr = node;    // node is a ref to a boxed node.
                }
                &curr.elem
            });
            panic!(" Linked list is empty ");
           
            
        }
   

          /** Gets the number of elements in the list.
     * we just take the len of the list and return the size
     * and what if we make a count variable which is updated with every push and pop>?
     */
    pub fn len(&self) -> i32 {
        //self.count
            let mut ll = &self.head;
            let mut len = 0;
          // so i quess to find the len i  need to find the tail from the head and and safe this in a let len;
            // ref rest
            while let Some(ref rest) = *ll{ // using ref the value is only borrowed not moved 
                len += 1;
                ll = &rest.next;
                }
                println!{"{:?}", len};
                len
    }






    //fn remove(&mut self, index: usize) -> Option<T>[−]
   // Removes the element at the given index. Returns None if the index is out of bounds.
            /***
     *      |  head 0  |      |  node 1  |        INSERT 2.2           |   node 2.1 tail  |      |    node 3   |
     *
     */

    //**Inserts an element at the given index.
    // tail and head stay the same so looping through the place depending on the size
    // //Panics if the index is greater than the length of the list. ? */
//     fn insert(&mut self, index:i32, elem:T){
      
    
//         let mut ind = index;
//        // let mut temp_list = &mut self.head;
//        // let mut left_list = &mut self.head;
//         // Check index not > then list. >
//         if index > self.count{
//             panic!(" index is bigger then the lengt of the list");
//         }

//         self.head.as_ref().map(|node|{  
//              // walk through the list till indext point
//              // safe this list in left_list
//             let mut left_list: &std::boxed::Box<Node<T>>;
//             while ind > 0 {
                
//                 let mut value = node;
//                 println!("leftlist  {:?}", value);
//                 ind -= 1;
//                 println!("leftlist  {:?}", left_list);
//             }
            
//             // swapping the values, but keeping the same references ? so we use map to have a look and return the right element. 

//             //println!("RESULT{:?}", left_list);
//             // loop through the rest of the list till the end 
//             // safe this in right_list
//             println!("Total list  {:?}", self.head);
//             println!("Size list  {:?}", self.count);
//             self.count = self.count - index;
//             println!("size right_list  {:?}", self.count);
//             // same as split_off. 
            
//         });
        
    
    
//    }


}

//fn iter<'a>(&'a self) -> Iter<'a, T>
// Provides a forward iterator.
 /* itterate through the list using trait iter 
.*/
impl<T>LinkedList<T>
{
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}
// iterator is a trait - methods. 
impl<'a,T> Iterator for Iter<'a,T>{
        
    type Item = &'a T;
    
    // next is part of the iterator trait
    fn next(&mut self)-> Option<Self::Item>{
        // walk through the list // if there is a node we loop returns the next if end none 
        self.next.map(|node|{
            self.next = node.next.as_deref(); // converts to target
            &node.elem
        })

    }

       // fn itterate(&self)-> Option<&T>{

        // receive the list and returns an option with value t a reference 
        // so we again dont do anything with the list we're just looking into it. 

    

}






#[cfg(test)]
mod tests {
    use crate::LinkedList; // import the struct
    extern crate colored; 
    use crate::tests::colored::Colorize;
    use colored::*;
    use colored::*;
    #[test]
    fn basics() {
        let mut List: LinkedList<i32> = LinkedList::new_list(); // make a list.

        // println!("{}",  "TEST with empty list".blue().bold());

        // println!("{}",  "PUSH BACK".blue().bold());
        // List.Push_back();
        //List.Push_back(2);
        // List.Push_back(3);
        // List.Push_front(1);
        // List.Push_front(2);
        // List.Push_front(3);
        // List.Push_front(4);
        // List.Push_front(5);
       // List.insert(3,9);

       List.Push_front(3);
       List.Push_front(4);
       let mut iter = List.iter();
       assert_eq!(iter.next(), Some(&4));


       // List.Push_back(4);
        //List.Push_front(4);
        //List.Push_back(7);
        //println!("{}",  "POP BACK".blue().bold());
       // assert_eq!(List.Pop_back(), Some(7));
       // List.Push_back(8);
        //assert_eq!(List.Pop_back(), None);

        //assert_eq!(List.peek_front(), Some(&7));
        //assert_eq!(List.Pop_back(), Some(&5));
        // assert_eq!(List.Pop_front(), Some(&5));
        //List.Push_back(5);
        //assert_eq!(List.Pop_front(), Some(&4));
        // List.Push_back(5);
        // assert_eq!(List.Pop_back(), Some(5));
        // assert_eq!(List.peek_back(), Some(&5));
        //List.insert(2,2 );
        // let len :i32 = List.insert(4,4 );
        // println!{"{:?}", len};
        //assert_eq!(List.Pop_front(), Some(5));
        // i want that it can pop from a certain place.
    }
}


   
