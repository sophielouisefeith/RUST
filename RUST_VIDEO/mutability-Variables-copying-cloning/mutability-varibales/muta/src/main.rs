#[derive(Debug, Clone, Copy)]

pub struct Person {
    name:String,
    age: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}


fn main() {
    let mut x = 34;
    let y = x;
    x += 5;
    println!(" y = {}, x = {}", y, x);


    // let's make a person 
    // we make it mut so that means we can change it.
    let mut p = Person{
        name: "Matt".to_string(),
        age:35,
    };

     // this is not possible -> if you want to copy somehting you better know what kind of copying you are doing.
    //let p2 = p;
   
    // use clone instead
    // were copying the string into another section of memory
    // serperate object
    let p2 = p.clone();
    p.name.push_str("Stoodly");
    println!("p = {:?}, p2 = {:?}", p, p2);
    let pnt = Point::new(3,4);


    let pnt = Point::new(3,4);
    let pn2 = pnt;
    
    println!("p = {:?}, p2 = {:?}", p, p2);
}