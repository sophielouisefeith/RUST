
/*linked list could be empty therfore we take an Option
//We wrap it in a box */
#[derive(Debug)]


pub struct LL<T>(Option<(T,Box<LL<T>>)>);


impl<T>LL<T>{
    /* public function new returns as self */
    pub fn new()->Self{
        /*empty LL */
        LL(None)
    }

    /* push front methos, will take the current list in a mutable 
    and the data that we want to add to the front */
    pub fn push_front(&mut self, data:T){
        /*  0 = the first element of self 0 is the option
         take removes the first value of the list  and gives it to t
         t remains an option its still empty */
        let t = self.0.take();
        /* Now we want to give a new value 
        tot data pointing to the linkedlist so to box*/
        self.0 = Some((data, Box::new(LL(t))));
    }

    // let's add something to the back// there is not really a back yet this means we have to recurse it. 

    pub fn push_back(&mut self, data:T){
        match self.0{
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }

    }

}

fn main() {

    /* create a new LL*/
    let mut ll = LL::new();
    ll.push_front(3);
    ll.push_back(12);
    ll.push_front(1);
    /* each one contains the next */


    println!("ll = {:?}", ll );
    println!("Hello, world!");
}
