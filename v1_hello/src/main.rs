//"our struct can be devote debug allowes the function to be displayed   {:?}"
#[derive(Debug)]

//Enum is an enumurated type 
//* colors are seperated objects.
pub enum Color{
    Red,
    Green,
    Blue,
}


// public struct ( a structure of data, all this data will be hold in the same place)
pub struct Person {
     name: String,                      // string is a pointer
     age: i32,                          // int with 32 bits.
     childeren: i32,                    //
}


// * We will give Person some abilities with impl
// * We take 'self' as a parameter and we will return a string. 
// * { represent the variables }
// * the variables become a property of self self.name etc / self refers to the variable that is being input on
// * We put return before format instead of a ; on the end of the function
impl Person{
    pub fn print(self)->String{         
           return format!("name  = {}, age = {}, childeren = {} ", 
            self.name, self.age , self.childeren
        )       
    }
}

// just a simple main functions.
fn main() {
    // create a new person
    let p = Person{
        name: "matt".to_string(), 
        age:35,
        childeren: 4,
    };
    println!("Hello, people from {:?}", p.print()); // p becomes self 

}
