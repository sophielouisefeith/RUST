/** linked list
 * following https://docs.rs/linked-list/0.0.3/linked_list/struct.LinkedList.html
 * --------
 * 
 * missing the use of NonNull and unsafe look intoo this 
 */ 


/** a struct list / generic type
 *
 */

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    count: usize,
    //tail: Option<Box<Node<T>>>,  add this to  optimise push front 
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
    pub fn new() -> Self {
        LinkedList {
        head: None,
        count: 0,     
        }
    }

 /**
  * * push front/back
     * push a new node to the list
     * receives the list, it's going to do something with the list so we need to receive a &mut self -> self == the list
     * create a new node
     * pushes the element to the top of the head
     *
     */

    pub fn push_front(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(), // take the current self.head and leave a non in its place - it will not excits anymore 
        });
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

    pub fn pop_front(&mut self) -> Option<T> {
       
       
        self.head.take().map(|node| {
            self.count -= 1;
            self.head = node.next;
            node.elem
        })
        
    }

    fn pop_back(&mut self) -> Option<T>{  
        
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
    }
   
    //* returns a mut ref of type T*/
    pub fn peek_mut(&mut self)-> Option<&mut T>{

        self.head.as_mut().map(|node| &mut node.elem) // unwrap with map and return if some. 
    }

/** Gets the number of elements in the list.
* we just take the len of the list and return the size
* and what if we make a count variable which is updated with every push and pop>?
*/
pub fn len(&self) -> i32 {
    let mut ll = &self.head;
    let mut len = 0;
    while let Some(ref rest) = *ll{ // using ref the value is only borrowed not moved 
        len += 1;
        ll = &rest.next;
    }
    len
}

/**If you allocate memory on the stack you have to return it as value. This includes the Box<_> scenario. 
 * You return the Box, which has a pointer to heap allocated memory, as value. If you do not allocate memory on the stack you can 
 * return references to the result which already lives in memory.
 * In Rust it is efficient to return by value, as the value is moved, not copied. */
 
fn get_node_mut(&mut self, index:usize) -> Option<&mut Box<Node<T>>>{ 
        
    if index  >= self.count{
        return None;}
    let mut current =  self.head.as_mut();  //box i want to take the head  as mut
    for _ in 0..index{
        current = current?.next.as_mut()     // it current is none? then return if it's some then next. // if you have a method return an option you can use ?
    }
        current   
    }

//**Inserts an element at the given index.
  
pub  fn insert(&mut self, index:usize, elem:T){

    if index == 0 { 
    self.push_front(elem)} 
    else {
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
fn empty_list(){

    let mut list: LinkedList<i32> = LinkedList::new();
    assert_eq!(list.pop_back(), None);
    assert_eq!(list.pop_front(), None);
}

#[test]
fn push_pop_peek() {

    let mut list: LinkedList<i32> = LinkedList::new();
   
    // fill list back and front     
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);
    list.push_front(5);
    list.push_back(6);
    list.push_back(7);
    list.push_back(8);


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
  fn insert_iter(){

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
