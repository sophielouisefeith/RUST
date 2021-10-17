/** linked list
 * following https://docs.rs/linked-list/0.0.3/linked_list/struct.LinkedList.html
 * make a new list
 * list contains a head {elem , next} // maybe add a  sep tail prev 
 * pop a node to the front
 * pop a node to the back
 * pop a node from the front
 * pop a node from the back
 * 
 * --------
 * insert a node - index 
 * itterate
 */


 // tight up my pop and push with tail 

 // insert &mut self, index: usize, elem T  
 // 1 ->  4 ->  20 
 // 1 -> 4 -> 11 -> 20
 // get node index -1     function //pas index -> &mut ref       return the NODE. not the elemnt 
 // node 1 needs to be filled with next: new_node
 // store next 
 // make new_node -> next (20)
 // node -> 11 new_node


extern crate colored; 

use colored::*;
use colored::*;


/** a struct list / generic type
 * the head : wiLinkedList be fiLinkedListed with link of a generic type
 */

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    count: usize, // counts the nodes/
    // prev: Option<Box<Node<T>>>,
    //tail
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
            // prev: None,
        
        }
    }

    /**
     * push front/back
     * push a new node to the list
     * receives the list, it's going to do something with the list so we need to receive a &mut self -> self == the list
     * create a new node
     * pushes the element to the top of the head
     *
     */

    pub fn push_front(&mut self, elem: T) {
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
    pub fn push_back(&mut self, elem: T) {
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
        self.count -= 1;
        self.head.take().map(|Node| {
            // so the struct node has two elements
            // so we wiLinkedList take the value of what is in the head away therefore we need to replace it with what is in  next node.
            self.head = Node.next;
            // and then we return what is in the elem of the node
            Node.elem
        })
    }

    // you know the tail push front  // just -> 


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
        self.head.as_ref().map(|node| 
            &node.elem)
        
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
           // panic!(" Linked list is empty ");
           
            
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
          
                len
    }
    
    // len == usize 
/**If you allocate memory on the stack you have to return it as value. This includes the Box<_> scenario. 
 * You return the Box, which has a pointer to heap allocated memory, as value. If you do not allocate memory on the stack you can 
 * return references to the result which already lives in memory.
 * In Rust it is efficient to return by value, as the value is moved, not copied. */
    
    /*** return the node on the given index 
    * walk through the nodes when on Node index return.
    */
   

    fn get_node(&mut self, index:usize) ->  &mut Node<T>{ 
        

        let mut ind:usize = index;
        let mut len:usize = 0;
        let current = self.head;
        let &mut curr;

        while len < index{  // after each itteration the value get's dropped.
            self.head.map(|node|{
            //let mut curr_node;
            //match index{
            if len == index { 
                curr = &mut Node{ elem: node.elem, next: node.next};
                }
                //node.next;
                //curr;
            //&mut Node{ elem: curr.elem, next: curr.next};
            });
            //curr;
           // &mut Node{ elem: curr.elem, next: curr.next};
            len += 1;
        //}
        &mut Node{ elem: curr.elem, next: curr.next};
    }

    
        //&mut curr; // want to return this
    

        
        //&mut Node<T>}

        //let mut ind:usize = index;
        //let mut len:usize = 0;
        
    //walk trough the Nodes.
        // needs to point to next node 
        //let mut next_node = next.self.head;
        //let mut next_node = &mut self.head;
       // let mut curr_elem;
      // let &mut curr_node;
       // self.head.take().map(|node|{
            // let curr_elem = node.elem;
            // println!("node next{:?}", node.next);
            // self.head = node.next;
            // println!("curr elem {:?}", curr_elem);
            // let curr_node =   Node{ elem:curr_elem, next:self.head}
        //});
        //let elem = self.head.take();
    //     let &mut curr_node =  &mut Node{ elem, next:self.head.take()}; 


    // }
       
    //}
    //     let mut current = self;

    //   let mut curr_node = Node{ elem: self.head, next:self.head.next}; // want to start ad the head fill this with node of head.
      
    //   let mut current = self;
    //     while len < index{
    //         if len == index{
    //             curr_node = Node{
    //             elem: &node.elem,
    //             next: node //needs to point to the next node in the list 
    //            };
    //         }
    //         else {
    //             node.next;
    //         }
        
    //     }
    //    &mut curr_node

    // }

        //     self.head.map(|node|{
        //     let mut cur_node = Node{
        //         elem: &node.elem,
        //         next: None //needs to point to the next node in the list 
        //     };
        //     cur_node
        // });
    
        







    //     let mut ind:usize = index;
    //     let mut len:usize = 0;
    //     self.head.as_ref().map(|head|{
    //     let mut curr = head;

        
    //     self.head.take().map(|Node| {
    //     //walk trhough the list 
    //         while let Some(node) = &node.next{
    //             if len == ind{
    //             let cur_node = Node{
    //                 elem : &node.elem,    // value of current node 
    //                 next: None,   // value of node next.    
    //                 };
    //             }

    //         else if len < ind {
    //             println!{"smaller then{:?}", node.next};
    //         }
    //         node.next;
    //         len += 1;
    //     }
    // });
    //     None
    
    // }





        // self.head.as_ref().map(|head|{

        //     let mut curr = head;
        //     while let Some(node) = &curr.next{
        //         println!("res{:?}", ind);
        //         println!("len{:?}", len);
        //         match ind{
        //             len if len == ind => {
        //                 println!("len == index");
        //                 println!("current node {:?}", curr.elem);
        //                 curr = &node;

        //                 println!("curr{:?}", curr);
        //             }
        //             len if len < ind => { 
        //                 println!("len < index"); 
        //                 curr = &node;
        //             }
        //             _ => panic!("no index"),
                    
        //         };
        //         len += 1;
        //     }
        //     println!("current node {:?}", curr.elem);

        // })
          
        
       


        



        // get the index 
  

    //fn remove(&mut self, index: usize) -> Option<T>[−]

   // Removes the element at the given index. Returns None if the index is out of bounds.

            /*** index 0         1                  2                       3                  4
     *      |  head 0  |      |  node 1  |        INSERT 2.2           |   node 2.1  |      |    node 3   |
     *
     */

    //**Inserts an element at the given index.
    // tail and head stay the same so looping through the place depending on the size
    // //Panics if the index is greater than the length of the list. ? */
    fn insert(&mut self, index:usize, elem:T){

        let mut ind = index;
        let mut count = self.count;
       // let mut temp_list = &mut self.head;
       // let mut left_list = &mut self.head;
        // Check index not > then list. >
        if index + 1 > self.count{
            panic!(" out of bounds ");
        }
        let result = match ind{
        0 => {
            let new_node = Box::new(Node {
                elem,
                next: self.head.take(), 
            });
            self.count += 1;
            self.head = Some(new_node);
        }
        count => {  
        let mut curr = &mut self.head;  
        while let Some(node) = curr {
            curr = &mut node.next;
        }
        *curr = Some(Box::new(Node { 
            elem, 
            next: None }));
            self.count += 1;
            }
        };
    
    let mut node_1 = self.get_node(index-1);  // return the current node 
    let node_next = node_1.next; 
    let insert_node = Node{
            elem: elem,
            next: node_next, // this node.next was on the place of the previous // need Node 
    };
    node_1 =  &mut Node{ elem:elem, next:Some(Box::new(insert_node))}
    

      //  println!("current node {:?}", curr);

        // example we have a list of 4  and we want to insert a 5 element on index 2
        // 


         /*** index 0               1                  2                       3                  4
     *      |  head 0  |      |  node 1  |        node 2.2         |   node 2.1  |      |    node 3   |
     *
     */
       //actually just want that the previous is pointing to the value of new node
        //and that the new node points to value of the next node. 
        
        // if it cant map node is None return None 
       // self.head.as_ref().map(|node|{ 

           // add a new node which points to the node of the index.
            //and go one back to point that one to the new node
            // first 

        
        // step 1 loop to the current node with the index// different way ? 
        // step 2 place the node in a temp
        // add the new node pointing 
        // fill the index node with current 
        // self.head.map(|head|{  
        //  // make it mutable cause otherwise it always will points to head. 
        //     while let Some(node) = &curr.next{  // were walking the list.
        //         while ind > 1{
        //             curr = next; 
        //             println!("left list  {:?}", node);
        //             ind -= 1;
        //         }  
             

                        //curr = node;    // node is a ref to a boxed node.
                       // &curr.elem
//              }

//             });
            
        
        
         }
 }
    
    //})

        
            // acces the node before the index point it two new node 

        // walk through the list till indext point
        // safe this list in left_list
    //     let mut left_list: &std::boxed::Box<Node<T>>;
    //    // let mut value;
    //     //println!("begin left  {:?}", left_list);
    //     // fill left list 
    //         while ind > 1 {
    //             //value = node;
    //             left_list = node;
    //             println!("leftlist  {:?}", left_list);
    //             ind -= 1;
    //             //println!("leftlist  {:?}", left_list);
    //         }
            

    //         //println!("RESULT{:?}", left_list);
    //         // walk through the rest of the list till the end 
    //         // safe this in right_list
    //         println!("Total list  {:?}", self.head);
    //         println!("Size list  {:?}", self.count);
    //         self.count = self.count - index;
    //         println!("size right_list  {:?}", self.count);
    //         // same as split_off. 
            
    //    // });
   

// iterator is a trait - methods. 

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
            //println!{"curr_node{:?}", &node.elem};
            &node.elem
        })

    }


}






#[cfg(test)]
mod tests {
    use crate::LinkedList; // import the struct
    extern crate colored; 
    use crate::tests::colored::Colorize;
    use colored::*;
    use colored::*;
    #[test]
    fn push_pop_peek() {
        let mut List: LinkedList<i32> = LinkedList::new_list(); // make a list.
        println!("{}",  "TEST push pop  peek ".blue().bold());

        // check empty list back
        assert_eq!(List.Pop_back(), None);

        // check empty list front 
        assert_eq!(List.Pop_front(), None);

        // fill list back and front 
        
        List.push_front(1);
        List.push_front(2);
        List.push_front(3);
        List.push_front(4);
        List.push_front(5);
        List.push_back(6);
        List.push_back(7);
        List.push_back(8);

        // peek in the list 


        // assert_eq!(List.peek_front(), Some(&5));
        // assert_eq!(List.peek_back(), Some(&8));


    }
    #[test]
    fn iter(){
        println!("{}",  "TEST iter ".blue().bold());
        let mut List : LinkedList<i32> = LinkedList::new_list();

        // List.push_front(1);
        // List.push_front(2);
        // List.push_front(3);
        // List.push_front(4);
        // List.push_front(5);
       
    
        println!("print total list {:?}", List);
    
        let mut iter = List.iter();

        assert_eq!(iter.next(), None);
        // assert_eq!(iter.next(), Some(&5));
        // assert_eq!(iter.next(), Some(&4));
        
       //assert_eq!(iter.next(), Some(&5));
        
        println!("print total list {:?}", List);
  }


  #[test]
  fn insert(){

    let mut List : LinkedList<i32> = LinkedList::new_list();

    List.push_front(5);
    List.push_front(4);    // new node 
    List.push_front(3);
    List.push_front(2);
    List.push_front(1);
    List.len();
    List.insert(3,6);
  }



      

}




       // List.push_back(4);
        //List.push_front(4);
        //List.push_back(7);
        //println!("{}",  "POP BACK".blue().bold());
        // assert_eq!(List.Pop_back(), Some(7));
        // List.push_back(8);
        // assert_eq!(List.Pop_back(), None);

        //assert_eq!(List.peek_front(), Some(&7));
        //assert_eq!(List.Pop_back(), Some(&5));
        // assert_eq!(List.Pop_front(), Some(&5));
        //List.push_back(5);
        //assert_eq!(List.Pop_front(), Some(&4));
        // List.push_back(5);
        // assert_eq!(List.Pop_back(), Some(5));
        // assert_eq!(List.peek_back(), Some(&5));
        //List.insert(2,2 );
        // let len :i32 = List.insert(4,4 );
        // println!{"{:?}", len};
        //assert_eq!(List.Pop_front(), Some(5));
        // i want that it can pop from a certain place.
   
