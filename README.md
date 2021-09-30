# How to learn rust?

# No tutorials

Follow:

https://doc.rust-lang.org/rust-by-example/index.html


We just simply start with excercises. 



list of elements each pointing to the next 
changing the pointer on the current element setting it on the next element.
usefull for stack object. 
or linked list.


- to add something to the back
using [match](https://doc.rust-lang.org/std/keyword.match.html)


OWN MADE QUIZE AND EXCERCISES
----------------------------------------
- Match is MUCH more powerful than switch ? why
write an example.




Doubly Linked list
-------------------
- using [weak](https://doc.rust-lang.org/std/rc/struct.Weak.html#method.upgrade)



Binary tree 
-----------------

https://levelup.gitconnected.com/rust-binary-tree-30efdd355b60




LIST:
------------
Slices 


# Learning rust 

“Rust is a systems programming language focused on three goals: safety, speed, and concurrency.”
__ Rust Documentation

## rust is a 
**A compiled language** 

A compiled language is a programming language whose implementations are typically compilers (translators that generate machine code from source code), and not interpreters (step-by-step executors of source code, where no pre-runtime translation takes place).

The term is somewhat vague. In principle, any language can be implemented with a compiler or with an interpreter.[1] A combination of both solutions is also common: a compiler can translate the source code into some intermediate form (often called p-code or bytecode), which is then passed to an interpreter which executes it.

**Comparison of multi-paradigm programming languages**


• rust needs to know for stack alocated things ow much room it will take up.
• rust always needs to know a value of a given type. value should always be the same size in memory.
• box size is a heap allocted own pointer. 


## **Resources**

1. https://people.eecs.berkeley.edu/~apanda/assets/papers/hotos2017-rust.pdf
2. https://learning-rust.github.io/docs/a3.hello_world.html
3. https://towardsdatascience.com/you-want-to-learn-rust-but-you-dont-know-where-to-start-fc826402d5ba#6919
4. https://rust-unofficial.github.io/too-many-lists/first.html
https://www.youtube.com/watch?v=IiDHTIsmUi4
5. Udemy:https://www.udemy.com/course/hands-on-data-structures-and-algorithms-in-rust/learn/lecture/20110448#overview
6. https://www.youtube.com/watch?v=IiDHTIsmUi4
7. https://docs.rs/petgraph/0.6.0/petgraph/
8.https://github.com/ProgrammingRust/examples


### NOTES from [udemy course](https://www.udemy.com/course/hands-on-data-structures-and-algorithms-in-rust/learn/lecture/20110450#overview) Hands-On data Structures and Algorithms in Rust 



##### 
Install rust and running a simple program
-----------------
step 1 create a new file
- `cargo new ` 
step 2 
- `cargo run` 

Structures and Enums in Rust
------------------------------------
new handled information:

impl:

Format:

derive debug:
- `#[derive(Debug)] `
- `{:?}`

Enum:
Enum is an enumurated type 

match


println! 
is not a function, it is a macro. Macros use ! to distinguish them from normal method calls. The documentation contains more information.

[Macro](https://doc.rust-lang.org/book/ch19-06-macros.html)
!

Results and Options 
-----------------------
Generics <T> could be any type

replace an enum with [Result](https://doc.rust-lang.org/std/result/)

but Result is also part of std. 

Looping meganisms in iterators
--------------------------------

doing the same thing multiple times but slightly different. 

Loop: 
While:
for:

mut : a variable that can change 

##### iterator
for an iterator you need the type
and your need 

Stack Data Structure in Rust
---------------------------------

Stack
• it is a list where items are added at one end and removed from the same end 
• you take from the top ( rapidly changing opbjects, parsers![](https://hackmd.io/_uploads/Bkg9YIgNF.png)
)

![](https://hackmd.io/_uploads/BJKUzDlNY.png)


recursive 
------------
tail recurtion
calling itself ( adding to the stack)
![](https://hackmd.io/_uploads/SkFpfvlNY.png)


stack overflow
result value/ as a final value  

![](https://hackmd.io/_uploads/Sk_NmPlVt.png)

grep memory if we dont know how big it's going to be. -> heap

for stack it needs to be a fixed size. 



Function call:

• Where to go after the function has finished

• A pointer

• Space for it's parameters

• Space for it's own variables 


Mutability, variables copying and cloning
------------------------------------------

introducing clone. -> for copying -> to be able to put on the stack.




### BOX
A pointer type for heap allocation. 

- `Box<T>` 

casually referred to as a 'box', provides the simplest form of heap allocation in Rust. Boxes provide ownership for this allocation, and drop their contents when they go out of scope.


freeing?
box is an owned pointer it ownes a value that it points to,
things are dropped(freed) when they go out of scope.
at the end of the scope of the own variable it gets dropped 




## **Instalation**

// for mac and linux Users

'curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh '
verify version rustc --version or rustc -V
rustup update

## **functions**

fn main(){

    println("hello world");
}



---

## everything on rust is stackor orientated 

---
 


## SINGLE LINKED LIST

Why use a linked list:

what does a linked list:

- `cargo new linked-list --lib`
- `cd linked-list`


the way that a linked list works, you take the head of a linked list.
you construct a new node.and you make the former head of a linked list 
the next link in that node.
HEAD-> new head to be another node pointing to the 


----------------

test unit test. 


unit test 
a piece ot the code / not mandatory. 

integration test

how your code intgrate with another code.
as it was used by other code 
use the libary -> plonk
if it somehting is not public then the intregration test can not test the implementation. 


unit test. (only if needed, tested only when needed)
in rust difference
is in the same file as your code.

//parent 
use super:: that the upper model is included ()



let x = Som(10);
let u: 
unwrapp // dont use unwrap

expect 


no semiclom => last value so that means it is the return value
 
 
 quationsmark as qoute 
if you try this 
- unwrappes 
imidate return
 if i reterun 
 
 converse a error
 
 if you cant convert the error then use expect
 
 io error -> look up.
 
 test us always std otherwise they cant ust std
 std::error::Error you can convert every error to std error 
 
 boxed dynamic error, any type implement type  value return value
 bxed -> the value is a box that points to allocate that can be anything value return is still  box. 
 
 
 more readable
 type  Gass = u64;
struct *Gas(u64) self.0
type 
new type pattern 


join -

as ref 

string c always a refereence to bytes 
if you take slice of it you point to the first 
characters 
vector

let a : vec
slice 


slice -> generic reference 


into is a conversion
we need different types 


learn match