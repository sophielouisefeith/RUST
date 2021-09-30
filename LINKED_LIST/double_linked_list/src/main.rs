


/* A Weak pointer is useful for keeping a temporary reference to the allocation 
managed by Rc without preventing its inner value from being dropped. */
/* 

/* Rc 
Single-threaded reference-counting pointers. ‘Rc’ stands for ‘Reference Counted’.
The type Rc<T> provides shared ownership of a value of type T, allocated in the heap. 
Invoking clone on Rc produces a new pointer to the same allocation in the heap. 
When the last Rc pointer to a given allocation is destroyed, the value stored in that 
allocation (often referred to as “inner value”) is also dropped. */


• RefCell hides the multibility 
• Immutable outside, but can mutate interior
• A mutable memory location with dynamically checked borrow rules*/


// why are the tools are not ideal ->  keep t
//keepin


use std::cell::RefCell;

/* weak reference ????*/
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct DbNode<T>{
    data:T, 
    next:Option<Rc<RefCell<DbNode<T>>>>,
    /* needs to be a weak reference, avoid memory leak dive intoo this more...... */
    prev:Option<Weak<RefCell<DbNode<T>>>>,
}

/* rc = reference counted object
when the last element is dropped the data element will be dropped
you can have multipul copies at one time, you can do the tracking of how many copies there are. */
#[derive(Debug)] // why is this so important 
pub struct DbList<T>{
    first:Option<Rc<RefCell<DbNode<T>>>>,
    last:Option<Weak<RefCell<DbNode<T>>>> 
}



impl<T> DbList<T>{
    pub fn new() -> Self {
        DbList {
            last: None,
            first: None, 
        }
    }
    pub fn push_front(&mut self, data:T){
        /* Take the first value to check if it has something in it*/
        match self.first.take(){
            /* if it has the value we need to put the value in front of it*/
            Some(r)=> {
                /* create a new front object */
                let new_front = Rc::new(RefCell::new(DbNode {
                    data,
                    next:Some(r.clone()), /* we need clone because of the refers. */
                    prev:None,
                }));
                /* tell the first object this is now in front of it */
                let mut m = r.borrow_mut();
                /* downgrade fromt strong to weak*/
                m.prev = Some(Rc::downgrade(&new_front));
                /* push it to the front */
                self.first = Some(new_front);
            }
            None => {
                // rc assums are immutabele
                let new_data = Rc::new(RefCell::new(DbNode{
                    data, 
                    next:None,
                    prev:None,
                }));
                /* why does last has to be a weak reference ?*/
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }

    pub fn push_back(&mut self, data:T){
        /* Take the first value to check if it has something in it*/
        match self.last.take(){
            /* if it has the value we need to put the value in front of it*/
            Some(value)=> {
                /* create a new back object */
                let new_back = Rc::new(RefCell::new(DbNode {
                    data,
                    prev:Some(value.clone()), /* we need clone because of the refers ? this has to do with rc
                    try to understand this better for yourself. */
                    next:None,
                }));
                /* tell the last object this is now behind it
                upgrade the weak reference r */
                //let st = Weak::upgrade(&value).unwrap(); //Constructs a new Weak<T>, without allocating any memory. Calling upgrade on the return value always gives None.
                let st = match Weak::upgrade(&value){
                    None => return,
                    Some(value) => value
                        //match on option// 
                        
                    
                };
                    //unwrap -> pannic 

                /* could we also write this like ?*/
               // let st = Weak::upgrade(&r)?;
                // no not ness because its an option so i returns NONE 
                //if it doesnt work  it's not ness an error?
                
                let mut m = st.borrow_mut();
                /* downgrade fromt strong to weak*/
                self.last = Some(Rc::downgrade(&new_back));
                /* push it to the front */
                m.next = Some(new_back);
                //put this on the front
            }
            None => {
                // rc assums are immutabele
                let new_data = Rc::new(RefCell::new(DbNode{
                    data, 
                    prev:None,
                    next:None,
                }));
                /* why does last has to be a weak reference ?*/
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }
}




fn main() {
    /* We want to push stuff on the front end the end of the array */
    /* Let's make a new list d1*/
    let mut dl = DbList::new();
    dl.push_front(6);
    dl.push_back(11);
    dl.push_front(5);
    dl.push_back(15);
    println!("dl = {:?}", dl);
}


// write pop front and pop back 