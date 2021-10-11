/** linked list
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
    head: Link<T>,
    Tail: Link<T>,
    count: i32, // counts the nodes/
}

/**  */
type Link<T> = Option<Box<Node<T>>>;

/** make a struct node which contains an element containing a generic T
 * and and a next which contains the link of a type t
 */
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>, //  an Option wrapped in a box.
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
            Tail :None,
            count: 1,
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


    /** Gets the number of elements in the list.
     * we just take the len of the list and return the size
     * and what if we make a count variable which is updated with every push and pop>?
     */
    pub fn len(&self) -> i32 {
        self.count
        //     let mut list = &self.head;
        //     let mut len = 0;
        //   // so i quess to find the len i  need to find the tail from the head and and safe this in a let len;
        //     // ref rest
        //     while let Some(ref rest) = *list{
        //         len += 1;
        //         list = &rest.next;
        //         }
        //         println!{"{:?}", len};

        //         len
    }

    /***
     *      |  head 0  |      |  node 1  |        INSERT 2.2           |   node 2.1 tail  |      |    node 3   |
     *
     */

    //**Inserts an element at the given index.
    // tail and head stay the same so looping through the place depending on the size
    //Panics if the index is greater than the length of the list. ? */
    // fn insert(&mut self, index:i32, elem:T){

    //         // inserting moving all the other nodes to the right
    //         //we need push back so first go back to that one.
   

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
        self.head.as_ref().map(|node| &node.elem)
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
            })
            
        }
   

    /* itterate through the list.*/



    //fn remove(&mut self, index: usize) -> Option<T>[−]
   // Removes the element at the given index. Returns None if the index is out of bounds.



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
        // List.Push_back(5);
        // List.Push_back(2);
        // List.Push_back(3);
       // List.Push_back(4);
       // List.Push_front();
        //List.Push_back(7);
        //println!("{}",  "POP BACK".blue().bold());
       // assert_eq!(List.Pop_back(), Some(7));
       // List.Push_back(8);
        assert_eq!(List.Pop_back(), None);
        //assert_eq!(List.peek_back(), Some(&7));
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


   
