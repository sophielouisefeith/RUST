

/** a struct list / generic type
 * the head : wiLinkedList be fiLinkedListed with link of a generic type 
 */

pub struct LinkedList<T> {
    head: Link<T>,
    Tail: Link<T>,
    count : i32, // counts the nodes/ 
    
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

impl<T>LinkedList<T> {


/** make a new list 
 * 
 * 
*/

    pub fn new_list()-> Self{
 
    // we need to return an empty list containing a head / head contains  a link which has type option so it either contains something or non 
    LinkedList{ head: None,
        Tail: None,
        count: 0 }
    
    }

/**
 * Push front/back 
 * push a new node to the list
 * receives the list, it's going to do something with the list so we need to receive a &mut self -> self == the list 
 * pushes the element to the head list and the head to the next node.
 * make a new node 
 */

    pub fn Push_front(&mut self, elem:T ){
   

        let new_node = Box::new(Node{      // box sits on the stack  memory on the heap and then places struct node into it.
            //
                elem: elem,
            // now the head is stiLinkedList empty
            // to fiLinkedList it we want to fiLinkedList the next
               next: self.head.take(), // next needs to point to the next element in this case head.
        });
        //fill head with new_node
        self.head = Some(new_node); 
        self.count += 1;

        if self.count == 2
        //{ self.tail};
        
    }


 pub   fn to_tail(&mut self, it: T){
        
        let mut curr = &self.head; //node 0
        while let Some(next) = curr{   // 
            
            curr = &next.next; // value of last node with the reference to the next node
        } 
         self.Tail = curr;
    }
    

 /*******--------       --------        --------              ------------
  *      |  head o |       |  node 1|       |  node 2 tail |      node 3
  *      --------         ------        --------               -------------
  */// so we can find the tail if next is == to None  then we know we are the end/  

    pub fn Push_back(&mut self, elem:T){

       
        // go to the last node and fill it's next with the new node.
        // so i want to be at were next.None and replace this None with the new node,s element right ?
    
        let mut curr = &self.head; //node 0
        
        while let Some(next) = curr{   // 
            
            curr = &next.next; // value of last node with the reference to the next node
        }
       let new_node = Box::new(Node{  
            elem:elem,
            next:None,
        });
        self.Tail = Some(new_node);
       


       // need to take what is in the 
      
        
        // now we are at the last node. 
        //elem: T,
        //next: Link<T>, // is an Option wrapped in a box. 

     





        //
        // while len < self.count {

            
        //     node.next;
        //     len += 1;

        // }
         // what if i safe my tail so i can always go to tail. 
    // *self = match self{
    //     Self::None=>  Self::tail{
    //         Link:
    //     }
    // }
    // needs to be an the end of the lis t
    //let new_node = Box::new(Node{
      //  elem: elem, // ok so we have the right element now we need to place it at the back
        // so first we need to get the node which is last atm and point the next to the new node. 
      //  next:None,
        
   // });

     //= Some(new_node);
    // but what happend with node 2
    //self.next = Some(new_node);


}

/** Gets the number of elements in the list.
 * we just take the len of the list and return the size 
 * and what if we make a count variable which is updated with every push and pop>? 
 */
  pub  fn len(&self) -> i32{

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
//          let len: i32 =  self.count;
//          println!{"{:?}", len};
//          //assert!(index >= self.count, "index out of bounds" );
//          if self.count <=  index{
//             panic!("Index is > then then lengt of the list ");
//          }
//          //make a loop which goed back to the tail and then loops backwards to the index and then
//          // add a node to the front 
//          // then in comes in a loop 
//          let mut curr = &self.head; 
//          let mut i :i32 = index;
//          while i > 0{

//             curr = &next.next; 
//             i =- 1;
            
//          }
//          // make a copy of list 
        
         
// }

 /*
  * Pop front/back
  * pop the value of the head from the list
  pop a value fromt  a list, we change the list her so we need to receive again a mut. 
  map: in this case ( option) unwraps  -> needs to return an option -> and wraps the result 
  Removes the first element and returns it, or None if the list is empty.
  */

   pub fn Pop_front(&mut self)-> Option<T>{
        // first we take the value of the elemt we want so 
        // we have to take out an element and put aLinkedList the elemnts in the correct order?
        // need to unwrap here we can use map map unwraps and wraps the results 
        self.count =- 1;
        self.head.take().map(|Node|{
            // so the struct node has two elements 
            // so we wiLinkedList take the value of what is in the head away therefore we need to replace it with what is in  next node.
            self.head = Node.next;
            // and then we return what is in the elem of the node
            Node.elem
            
        })
       
   }


  
  fn Pop_back(&mut self) -> Option<T>{
     
        self.Tail.take().map(|Node|{
            self.Tail = Node.next;
            Node.elem
        
    })
        // self.Tail.take().map(|Node|{
        // //now we take the last element away so we need to replace it with when next was still some.
        // Some(next);
        // Node.elem
 // })
  }

  /*Peek 
   * peek what is in front of the list.
   */

  pub fn peek_front(& self)-> Option<&T>{

    self.head.as_ref().map(|node|{
        &node.elem

    })
    
}

pub fn peek_back(& self)-> Option<&T>{

    self.Tail.as_ref().map(|node|{
        &node.elem

    })
    
}


   /* itterate through the list.*/

}

#[cfg(test)]
mod tests {
    use crate::LinkedList; // import the struct 
    #[test]
    fn basics() {
    let mut List: LinkedList<i32> = LinkedList::new_list();// make a list.
    
    List.Push_front(3);
    List.Push_front(4);
    List.Push_front(5);
    assert_eq!(List.peek_back(), Some(&5));
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


