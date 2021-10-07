/** own linked list/ through the linked list i hoped to learn the basics of rust parallel on making this linked list
 * i've been practising with small excersises and reading the rust book.
 * made with having the book 
 * 
 * this linked list will do the following 
 * struct -> option box
 * Push
 * 
 * Pop
 * 
 * peek
 * 
 * itterate 
 * 
 * We use : map, take, lifetimes, 
 */



/** a struct list / generic type
 * the head : will be filled with link of a generic type 
 */

pub struct LL<T> {
    head: Link<T>,
    count : i32,
    //tail: Link<T>, i think we dont need it since the node with a next pointing to none is the tail 
}

/**  */
type Link<T> = Option<Box<Node<T>>>;  

/** make a struct node which contains an element containing a generic T
 * and and a next which contains the link of a type t
 */
#[derive(Debug)]
struct  Node<T> {
    
    elem: T,
    next: Link<T>, // is an Option wrapped in a box. 
}

impl<T>LL<T>{


/** make a new list 
 * you will need the list list 
 * 
*/

pub fn new_list()-> Self{
 
    // we need to return an empty list containing a head / head contains  a link which has type option so it either contains something or non 
    LL{ head: None,
    count: 0 }
    
}

/**
 * Push
 * push a new node to the list
 * receives the list, it's going to do something with the list so we need to receive a &mut self -> self == the list 
 * pushes the element to the head list and the head to the next node 
 * make a new node 
 */

 pub fn Push_front(&mut self, elem:T ){
   


        let new_node = Box::new(Node{      // Allocates memory on the heap and then places struct node into it.
            // so we receive a 
                elem: elem,
            // now the head is still empty
            // to fill it we want to fill the next
                next: self.head.take(), // take takes the value of head it and put heads on none. this points always to the next value.
        });
        // now we need to fill the first elemnt with the elem we received 
        self.head = Some(new_node);    

        // we need to fill the head with the new elem.
        //self.head = Some(elem);
        // we need to link the head to the next node.
        //self.head 
        // we need to fill the head with the new element

        // we return the list? 


 }

 /*******--------       --------        --------              ------------
  *      |  head  |       |  node 1|       |  node 2 tail |      node 3
  *      --------         ------        --------               -------------
  */// so we can find the tail if next is == to none then we know we are the end/  

pub fn Push_back(&mut self, elem:T){
    // needs to be an the end of the lis t
    let new_node = Box::new(Node{
        elem: elem, // ok so we have the right element now we need to place it at the back
        // so first we need to get the node which is last atm and point the next to the new node. 
        next:None,
        // oke so now it points to the end of the list 
    });

     //= Some(new_node);
    // but what happend with node 2
    //self.next = Some(new_node);
    

}

/** Gets the number of elements in the list.
 * we just take the len of the list and return the size 
 * and what if we make a count variable which is updated with every push and pop>? 
 */
fn len(&self) -> usize{

    let mut list = &self.head;
    let mut len = 0;
  // so i quess to find the len i  need to find the tail from the head and and safe this in a let len;
    // ref rest 
    while let Some(ref rest) = *list{
        len += 1;
        list = &rest.next;
    }
        len
}


//**Inserts an element at the given index. */
fn insert(&mut self, index:usize, elem:T){
         // think first need to know the lengt of the linked list 
         // so i call len 
}

 /*
  * Pop
  * pop the value of the head from the list
  pop a value fromt  a list, we change the list her so we need to receive again a mut. 
  map: in this case ( option) unwraps  -> needs to return an option -> and wraps the result 
  Removes the first element and returns it, or None if the list is empty.
  */

   pub fn Pop_front(&mut self)-> Option<T>{
        // first we take the value of the elemt we want so 
        // we have to take out an element and put all the elemnts in the correct order?
        // need to unwrap here we can use map map unwraps and wraps the results 
        self.head.take().map(|Node|{
            // so the struct node has two elements 
            // so we will take the value of what is in the head away therefore we need to replace it with what is in  next
            self.head = Node.next;
            // and then we return what is in the elem of the node
            Node.elem

        })

   }


  /*Peek 
   * peek what is in front of the list.
   */

  pub fn peek(& self)-> Option<&T>{

    self.head.as_ref().map(|node|{
        &node.elem

    })
    
}


   /* itterate through the list.*/

}

#[cfg(test)]
mod tests {
    use crate::LL; // import the struct 
    #[test]
    fn basics() {
    let mut List: LL<i32> = LL::new_list();// make a list.
    
    List.Push_front(3);
    List.Push_front(4);
    assert_eq!(List.Pop_front(), Some(4));
    List.Push_back(5);
    //assert_eq!(List.Pop_front(), Some(5));
    // i want that it can pop from a certain place. 


    }
}
