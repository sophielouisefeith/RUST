

/* please create a range object 0..10
**************** using if************* */

pub struct Stepper{
    // curr now
    curr: i32, // value now
    step: i32, // how big are the steps
    max:  i32, // were it stop's
}

/* impl used to give functionality to an object
• you can do this also to total crates
• an intertor requires a fn
• we also need to say what kind of type this iterator is.
• and the function next item and return need to have the same type */
impl Iterator for Stepper{

    type Item = i32;
    fn next(&mut self) -> Option<i32> {    // a pointer to a changable version of self // it will return some Option of i32 or none
        if self.curr >= self.max {
            return None;
        }
        // we want to add to current and the same time returning an actuall value.
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}


fn main() {

    // using loop
    let mut st = Stepper{curr:2, step:3, max:15};
    loop{
        // whatever comes back on match will be Some or None
        match st.next(){
            Some(v) => println!("loop{}", v),
            None => break,
        }
    }

    // using while 
    let mut st = Stepper{curr:2, step:3, max:15};
    while let Some(n) = st.next(){
        println!("while, {}!", n);
    }
      

    // using for 

    let it = Stepper{
            curr:2, 
            step:3, 
            max:15,
    };

    for i in it{
        println!("for loop {} ", i);
    }
    // for i in 0..10{
    //     println!("for loop {}", i);
    // }
}

//**************** using while *************/


// fn main() {

//     let mut n  = 0;
//     while n <= 10{
//         println!("Hello, world!, {}!", n);
//         n += 1; 
//     } 

//     for
//     println!("all done ");
// }


// create a loop, this loop will go on forever untills you say it shoudt 
// create a mut variable a variable which can change 
//**************** using loop *************/

// fn main() {

//     let mut n  = 0;
//     loop{
//         n += 1;                    // increament with 1. 
//         if n > 10{
//             break;
//         }

//         println!("Hello, world!, {}!", n);
//     }
//     println!("all done ");
// }


