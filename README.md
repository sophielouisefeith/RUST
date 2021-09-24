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


### BOX
A pointer type for heap allocation. 

- `Box<T>` 

casually referred to as a 'box', provides the simplest form of heap allocation in Rust. Boxes provide ownership for this allocation, and drop their contents when they go out of scope.


freeing?
box is an owned pointer it ownes a value that it points to,
things are dropped(freed) when they go out of scope.
at the end of the scope of the own variable it gets dropped 

## **Resources**

1. https://people.eecs.berkeley.edu/~apanda/assets/papers/hotos2017-rust.pdf
2. https://learning-rust.github.io/docs/a3.hello_world.html
3. https://towardsdatascience.com/you-want-to-learn-rust-but-you-dont-know-where-to-start-fc826402d5ba#6919
4. https://rust-unofficial.github.io/too-many-lists/first.html
https://www.youtube.com/watch?v=IiDHTIsmUi4


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


