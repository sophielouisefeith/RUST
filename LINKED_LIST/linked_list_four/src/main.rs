
/* The last expression of a function is implicitly returned 
   We refer to variants of an enum using :: -> namespace operator 
*/

// enum of linked list with two elements  node and last each node contains an element 
//of i32 and  box pointing to the next element 

/** take() */

/* enum is a "sum type" -> a type that can have different values which may be different types */

// create a list
// add a new node
// push to the node
// pop 
// iterate
// count


#[derive(Debug)]

/** make a public struct list

 */
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct  Node<T> {
    
    elem: T,
    next: Link<T>,
}

// class/creats the method for the class 

// trait iterator  -> general behaviours -> you can use it for different structs. impl iter for list list.iter 
impl<T>List<T> {
    pub fn new() -> Self {
        List { head: None }     // head is none so has this work with the {   we put here head on none }
        //returns self self is the list 
    }


/**   Check if the list is empty.
make a new node
replace head with the new node 

replace the list's head with its next
 */ 

    pub fn push(&mut self, elem: T) {
        //make a new node to store the elemnt in 
        let new_node = Box::new(Node {
            elem: elem,      // we set elem to elem 
            // take how does this work 
            next: self.head.take(), // we place the value of the head in the next node.
        });
        // fill the new head. 
        self.head = Some(new_node);
        
    }

    // returns option


 /**   Check if the list is empty.
If it's empty, just return None
If it's not empty
remove the head of the list
remove its elem
replace the list's head with its next
return Some(elem)

closure:
 */ 
  // method take 
   //  map == match option { None => None, Some(x) => Some(y) } ?
   //  the self needs to be mutable.   
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node|         // list take the head and compare with the node 
            {         
            self.head = node.next;
            node.elem
            
        })
        
    }



    /* PEEK 
     * peek returns a reference  
    */


    /* LIFETIME
     * every reference in Rust has a lifetime, which is the scope for which that reference is valid. 'a
     * The main aim of lifetimes is to prevent dangling references, which cause a program
     * to reference data other than the data it’s intended to reference.
     */

     /** 
      * how to get to the next item for the iterator 
      * if there is no next item then return s in your implemention of iter 
      */

}

/**
 * MAKE AN ITTERATOR, oke we would like to implement an iterator
 * 
 */




#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

       
        // Check empty list behaves right
        // print the map
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);
        
       //assert_eq!(list.pop(), Some(2));
    }
}



fn main() {

    


    let mut list: List<i32> = List::new();


    list.push(0);
    list.push(1);
    list.push(2);

    
   
  
    println!("{:?}", list);
  
}
