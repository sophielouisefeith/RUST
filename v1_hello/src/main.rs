//"our struct can be devote debug allowes the function to be displayed   {:?}"





// public struct ( a structure of data, all this data will be hold in the same place)
#[derive(Debug)]
pub struct Person {
     name: String,                      // string is a pointer
     age: i32,                          // int with 32 bits.
     childeren: i32,                    //
     fave_color: Color,
}


//Enum is an enumurated type 
//* colors are seperated objects.
//* it repesents a choice , its either going to be red green or blue
//* Let's derive debug to receive some data.
//* enums can have a variable inside them
// * even when they have different memmory object, rust takes the biggest one 
//* it can contain different types of objects in the same object

#[derive(Debug)]

pub enum Color{
    Red(String),
    Green,
    Blue,
}

// * We will give Person some abilities with impl
// * We take 'self' as a parameter and we will return a string. 
// * { represent the variables }
// * the variables become a property of self self.name etc / self refers to the variable that is being input on
// * We put return before format instead of a ; on the end of the function
impl Person{
    pub fn print(self)->String{         
           return format!("name  = {}, age = {}, childeren = {}", 
            self.name, self.age , self.childeren,
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
        fave_color: Color::Green
    };
   
    // get a color

    let c = Color::Red("hello".to_string());

    // we  match against c 
    // if c is red then i want you to print the line red.
    // red has content string 
    match c {
        Color::Red(s) => println!("it's red {}", s), // output s
        Color::Green => println!("it's green"),
        Color::Blue => println!("it's blue"),
    }
    println!("Hello, people from {:?}", p.print()); // p becomes self 
}
