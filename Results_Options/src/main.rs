
//*********************  EXAMPLE with enum or use std RESULT  & OPTION  *****************************//
//********************* OK is also port of std remove RESULT *****************************//
//*********************  USE OPTION *****************************//

#![allow(non_snake_case)] // included this after an error. 


pub enum Option<T>{
    Some(T),
    None,
}

fn main() {

    // let's call the function divide 
    let a = divide(4, 5); // this will be a thing 
    let b = divide (10,0 ); // thist will be an error 
    let c = divide (10, 5); // thist will be an error 

    // can you use result with match
    // match c {
    //     Result::Thing(v) => println! ("val ={}", v),
    //     _ => {}
    // }
    println!("a = {:?}, b = {:?}", a, b);
    println!("c = {:?}", c);

    //or instead of match we use if let statement , BUT if you want to handle more cases then use match
    // so we say if thing(v)-> v == t a generic type...   we put c in v  then we print the value of v if not an error
    if let Ok(v) = c {
        println!("val = {}",v);
    }   
}

fn divide(a:i32, b:i32)->Result<i32, String> { 
    if b == 0{
        return Err("Cannot Divide by zero".to_string());
    }
    Ok(a /b)
} 


//*********************  un command to use std *****************************//
//*********************  WHEN WE USE STD RESULT::ERR *****************************//
// #![allow(non_snake_case)] // included this after an error. 

// fn main() {

//     // let's call the function divide 
//     let a = divide(4, 5); // this will be a thing 
//     let b = divide (10,0 ); // thist will be an error 
//     let c = divide (10, 5); // thist will be an error 

//     // can you use result with match
//     // match c {
//     //     Result::Thing(v) => println! ("val ={}", v),
//     //     _ => {}
//     // }
//     println!("a = {:?}, b = {:?}", a, b);
//     println!("c = {:?}", c);

//     //or instead of match we use if let statement , BUT if you want to handle more cases then use match
//     // so we say if thing(v)-> v == t a generic type...   we put c in v  then we print the value of v if not an error
//     if let Result::Ok(v) = c {
//         println!("val = {}",v);
//     }   
// }

// fn divide(a:i32, b:i32)->Result<i32, String> { 
//     if b == 0{
//         return Result::Err("Cannot Divide by zero".to_string());
//     }
//     Result::Ok(a /b)
// } 

//*********************  un command to use enum *****************************//
//*********************  WHEN WE USE ENUM  *****************************//
// fn main() {

//     // let's call the function divide 
//     let a = divide(4, 5); // this will be a thing 
//     let b = divide (10,0 ); // thist will be an error 
//     let c = divide (10, 5); // thist will be an error 

//     match c {
//         Res::Thing(v) => println! ("val ={}", v),
//         _ => {}
//     }
//     println!("a = {:?}, b = {:?}", a, b);
//     println!("c = {:?}", c);

//     //or instead of match we use if let statement , BUT if you want to handle more cases then use match

//     // so we say if thing(v)-> v == t a generic type...   we put c in v  then we print the value of v if not an error

//     if let Res::Thing(v) = c {
//         println!("val = {}",v);
//     }
//     // should here to be an else? with then the error ?
   
// }


// create an enum res
// Generics <T> could be any type, 
// or it could be type E
// option you keep the flow of your program
// NOTE there is a standard libary funciton we could use. 

// #[derive(Debug)]
// pub enum Res<T,E>{
//     Thing(T),
//     Error(E),
// }

// i32 = t , string = E
// when we use enum
// fn divide(a:i32, b:i32)->Res<i32, String> { 
//     if b == 0{
//         return Res::Error("Cannot Divide by zero".to_string());
//     }
//     Res::Thing(a /b)
// } 

