/* source: https://doc.rust-lang.org/std/collections/struct.LinkedList.html */



/* 
linked list step by step with*/

// impl <T> LinkedList<T>{}

// /* create empty linkedlist */
// pub const fn new()-> LinkedList<T>{
   
// }

// /* mutable reference
// self as receiver 
// Moves all elements of other to the end of the list
// reuse all the nodes from others and moves them into self
// */

// pub fn append(&mut self, other:&mut LinkedList<T>){
//     // make a mut list  
//     let mut list1  = Linkedlist::new();
//     list1.push_back('a');

//     //let mut list2 = 
// }

/* other is empty */



#![allow(unused)]
fn main() {


    use std::collections::LinkedList;

    let mut list1 = LinkedList::new();
    list1.push_back('a');

    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');

    list1.append(&mut list2);

    let mut iter = list1.iter();
    assert_eq!(iter.next(), Some(&'a'));
    assert_eq!(iter.next(), Some(&'b'));
    assert_eq!(iter.next(), Some(&'c'));
    assert!(iter.next().is_none());

    assert!(list2.is_empty());

    println!("list 1 {:?}", list1);







    let mut list: LinkedList<u32> = LinkedList::new();

    list.push_back(5);
    list.push_back(1);
    list.push_front(2);

    let mut iter = list.iter();


    /*On panic, this macro will print the values of the expressions with their debug representations.
    Like assert!, this macro has a second form, where a custom panic message can be provided. */

    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);


    println!("list  {:?}", list);
}