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

// use core::ptr::NonNull;
   

// extern crate colored; 

// use colored::*;
// use colored::*;


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

// pub struct Cursor<'list, T: 'list> {
//     current: Option<NonNull<Node<T>>>,
//     list: &'list LinkedList<T>,
// }

// pub struct CursorMut<'list, T: 'list> {
//     current: Option<NonNull<Node<T>>>,
//     list: &'list mut LinkedList<T>,
//     current_len: usize,
// }
// impl<'list, T> CursorMut<'list, T> {

//     fn next(&self) -> Option<Box<Node<T>>>{
//         self.current
//             .map_or(self.list.head, |node| unsafe { node.as_ref().next })
//     }

//     pub fn current(&mut self) -> Option<&mut T> {
//         self.current.map(|node| unsafe {
//             // Need an unbound lifetime to get same lifetime as self
//             let node = &mut *node.as_ptr();
//             &mut node.elem
//         })
//     }


// }


/** implement Linkedlist */
impl<T: std::fmt::Debug> LinkedList<T> {
    // impl for ll with type t contrained to implements trait.

    /** make a new list
     * returns an empty list
     */
    pub fn new() -> Self {
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
    }


  
    /*
    * Pop front/back
    * pop the value of the head from the list
    pop a value fromt  a list, we change the list her so we need to receive again a mut.
    map: in this case ( option) unwraps  -> needs to return an option -> and wraps the result
    Removes the first element and returns it, or None if the list is empty.
    */

    pub fn Pop_front(&mut self) -> Option<T> {
        self.count -= 1;
        self.head.take().map(|Node| {
            self.head = Node.next;
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
   


// pub fn peek_mut(&self)-> Option<&mut T>{

//     self.head.as_mut().map
// }


    

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
   

    // fn next(&self) -> Option<Box<Node<T>>>{

    //     self.head.map(|node| 
    //         { 
    //             node.next
            
    //         })
    // }

 
    fn get_node_mut(&mut self, index:usize) -> Option<&mut Box<Node<T>>>{ 
        
        if index  >= self.count{
            return None;
        }
        let mut current =  self.head.as_mut();  //box i want to take the head  as mut
        for _ in 0..index{
            current = current?.next.as_mut()     // it current is none? then return if it's some then next. // if you have a method return an option you can use ?
        }
        current   
    }



   // Removes the element at the given index. Returns None if the index is out of bounds.

            /*** index 0         1                  2                       3                  4
     *      |  head 0  |      |  node 1  |        INSERT 2.2           |   node 2.1  |      |    node 3   |
     *
     */

    //**Inserts an element at the given index.
    // tail and head stay the same so looping through the place depending on the size
    // //Panics if the index is greater than the length of the list. ? */
    pub  fn insert(&mut self, index:usize, elem:T){


        if index == 0 { 
            self.push_front(elem);
        } else {

           let  prev =  self.get_node_mut(index -1) ;
            if let Some(node) = prev{   //if let unwraps 

                let next = node.next.take();  
                
                let insert_node = Box::new(Node{
                     elem,
                    next,
                });
                
                node.next  = Some(insert_node);  

            }

        }
      
    }

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
        let mut List: LinkedList<i32> = LinkedList::new(); // make a list.
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
        let mut List : LinkedList<i32> = LinkedList::new();

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
  fn get_node_mut(){

    let mut list : LinkedList<i32> = LinkedList::new();

    let node = list.get_node_mut(0);
    assert!(node.is_none());

    list.push_front(5);
    let node = list.get_node_mut(0);
    assert!(node.is_some());

    let node = list.get_node_mut(2);
    assert!(node.is_none());

    list.push_front(10);
    list.push_front(12);
    let node = list.get_node_mut(2);
    assert_eq!(node.unwrap().as_ref().elem, 5);
    

    
    
  }

  #[test]
  fn insert(){

    let mut list : LinkedList<i32> = LinkedList::new();

    list.push_front(1);
    list.push_front(2);
    list.push_front(20);
    list.push_front(21);

    list.insert(2, 11);
    list.insert(0, 12);
    let  mut iter = list.iter();

     assert_eq!(iter.next(), Some(&12));
     assert_eq!(iter.next(), Some(&21));
     assert_eq!(iter.next(), Some(&20));
     assert_eq!(iter.next(), Some(&11));
     assert_eq!(iter.next(), Some(&2));
     assert_eq!(iter.next(), Some(&1));

     

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
   
