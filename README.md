# RUST
## Installation
``` bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## First Program
- create a file called main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

- compile
```bash
rustc main.rs
```

- this will create an executable called main

- run the executable

```bash
./main
```

## Cargo
- build system and package manager
- comes preinstalled with rust 

### Create a New Cargo Application
```bash
cargo new hello_cargo
```

- creates a new directory called __hello_cargo__

- in that directory, there will be 2 files and 1 directory
    - Cargo.toml
    - src
        - main.rs

- it also initializes a new git repo along with a .gitignore file

- dependencies can be added to the toml file.

### Building and Running a Cargo Project
- build by running this command
```bash
cargo build
```
- this will create an executable in target/debug folder 
- running cargo build for the first time creates a new file at the top level called __Cargo.lock__ - it keeps track of the exact versions of dependencies in your project
- there is a single command to compile the code and run the resultant executable all in one command
```bash
cargo run
```
- there's also another command to check your code to make sure it compiles but doesn't produce an executable
``` bash
cargo check
```

### Building for Release
- when your project is ready for release, you can use 
```bash
cargo build --release
```
- this compiles it with optimizations
- this creates an executable in __target/release__ instead of __target/debug__

### Conventions
- with simple projects, cargo doesn't have advantages over using rustc

## Concepts
- variables in Rust are immutable by default - meaning that once we give a variable a value, it won't change.
- to make a variable mutable, we add mut before the variable name.

### crates
- crates are like dependencies
- they are added to the Cargo.toml file


## Common Programming Concepts
### Variables and Mutability
- by default, variables are immutable(constants)
- you can still make variables mutable(variable)
- to make a variable mutable, add __mut__ before the variable name.

#### Declaring Constants
- just like immutable variables, constants can't change values
- there a few differences between constants and variables
- constants aren't just immutable, they are always immutable.
- you declare them using const keyword instead of the let keyword, type of the value must be annotated.
- constants can be declared in any scope, including the global scope
- convention is to use all uppercase letters and separate words using underscores.
- example
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

####  Shadowing
- the act of overriding the value or type of a variable

```rust
let x = 10;

let x = 10 * 2;

println!("The value of x is now {x});
```

- here the value of x changes
- idea was to be able to perform operations on a variable without having to create a new variable
- you can even change the data type of a variable when shadowing e.g

```rust
let spaces = "   ";
let spaces = spaces.len();
```

- here we are changing from a string to a number using shadowing

### Data Types
- Rust is a statically typed language - it must know the types of all variables at compile time.
- the compiler can usually infer what type we want to use based on the value and how we use it.

#### Scalar Types
- represents a single value
- Rust has 4:
    - integers
    - floating point numbers
    - booleans 
    - characters

##### Integer Types
- a number without fractional part

|Length|Signed|Unsigned|
|--|--|--|
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit|i32|u32|
|64-bit|i64|u64|
|128-bit|i128|u128|
|Architecture-dependent|isize|usize|

- isize and usize depend on the architecture you are running - 64 bits or 32 bits

##### Floating-Point Types
- has 2 options f32 and f64
- default is f64
- example

```rust
fn main() {
    let x = 2.0; //f64

    let y: f32 = 3.0; //f32
}
```

##### Boolean Type
- 2 possible values
    - true
    - false

- are 1 byte in size

- example
```rust
fn main() {
    let t = true;

    let f: bool = false;
}
```

##### Character Type
- most primitive alphabetic type
- example
```rust
fn main() {
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
}
```

- we specify char literals with single quotes as opposed to strings which use double quotes.
- char type is 4 bytes and represents a unicode scalar

#### Compound Types
##### Tuples
- groups a number of values with a variety of types into 1 compound type
- they have a fixed length
- example:
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
- to get the individual values out of a tuple, we can use pattern matching to destructure a tuple value
```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```
- we can also access a tuple element by using a period(.) followed by the index of the value we want to access e.g
```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

##### Arrays
- all elements are of the same type
- example
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```
- useful when you know the number of elements will not need to change.

## Understanding Ownership
- most unique feature of the language
- enables Rust to make memory safety guarantees without needing a garbage collector

### What is Ownership?
- set of rules that govern how Rust manages memory
- some languages have garbage collectors that regularly look for no-longer used memory as the program runs
- in other languages, the programmer must explicitly allocate and free the memory
- Rust uses a third approach - memory is managed through a system of ownership with a set of rules that the compiler checks - if any of the rules are violated, the program won't compile.

### The stack and the heap
- both are parts of memory available to your code to use at runtime but they are structured in different ways.

#### stack
- the stack stores values in the order it gets them and removes the values in the opposite order(FILO)
- data stored on the stack must have a known fixed size
- data with an unknown size at compile time or a size that might change must be stored in the heap instead.

#### heap
- less organized 
- when you put data on the heap, you request a certain amount of space.
- the memory allocator finds an empty spot in the heap that is big enough, marks it as being in use and returns a __pointer__ - the address of the location - this process is called allocating on the heap
- because the pointer to the heap is known, fixed size, you can store the pointer on the stack, but when you want the actual data you must follow the pointer

#### cont'd
- pushing to the stack is faster than allocation on the heap because the allocator never has to search for a place to store new data.
- allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
- accessing data on the heap is generally slower than accessing data on the stack because you have to follow a pointer to get there.

#### concept
- when your code calls a function, the values passed into the function and the function's local variables get pushed onto the stack
- when the function is over, those values get popped off the stack.

### Ownership rules
1. each value in Rust has an owner.
1. there can only be one owner at a time.
1. when the owner goes out of scope, the value will be dropped.

### The String Type
- types of a known size can be stored on the stack and popped from the stack when they are no longer needed.
- the string type is quite different, it's size is not known
- the string type is stored on the heap.
- for string literals, a string value is hardcoded into the program - they are convenient - however, they are not suitable for every situation since they are __immutable__
- it is for this reason that Rust has a String type - this type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time
```rust
let s = String::from("hello");
```

- this kind of string can be mutated
```rust
let mut s = String::from("hello");
s.push_str(", world!");
println!("{s}");
```

### Memory and Allocation
- in the case of a string literal, we know the contents at compile time - so the text is hardcoded directly into the final executable - explains why string literals are fast and efficient.
- with the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the __heap__, unknown at compile time, to hold the contents. This means:
    - the memory must be requested from the memory allocator at runtime
    - we need a way of returning this memory to the allocator when we're done with our string.
- the first part of requesting the memory is done by us just like in any other programming language(when we call String::from)
- however, the second part is different. in languages with a garbage collector, the GC keeps track of and cleans up memory that isn't being used anymore.
- in other languages without a GC like C, it is the developer's role to clean up the memory - 2 common issues with this:
    - early cleaning of memory
    - double free
- Rust takes a different path: memory is automatically returned once the variable that owns it goes out of scope.

```rust
{
    let s = String::from("Hello"); //s is valid from here onwards

    // do stuff with s
}

// this scope is now over and s is no longer valid
```

- this is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope.
- when a variable goes out of scope, Rust calls a special function for us, the function is called __drop__ and it's where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

#### Variables and Data Interacting with Move
- multiple variables can interact with the same data in different ways in Rust
```rust
let x = 5;
let y = x;
```
- here, we are dealing with integers, and they have a fixed size and are therefore pushed to the stack.

```rust
let s1 = String::from("hello");
let s2 = s1;
```
- here, we are dealing with a string - it is not business as usual from the last example.
- a string is made up of 3 parts:
    - pointer to the memory that holds the contents of the string
    - a length
    - a capacity
- this group of dat is stored on the stack
- the length is how much memory in bytes, the contents of the string are currently using, the capacity is the total amount of memory in bytes that the String has received from the allocator
- when we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length and the capacity that are on the stack - we do not copy the data on the heap that the pointer references
- earlier, we said that when a variables goes out of scope, Rust automatically calls the drop function and cleans the heap memory for that variable - this is a problem.
- when s1 and s2 go out of scope, they will both try to free the same memory(__double free error__)
- to ensure memory safety, after the line;
```rust
let s2 = s1;
```
- Rust considers s1 as no longer valid - therefore Rust doesn't need to free anything when s1 goes out of scope
- you can think of it as a __move__

#### Scope and Assignment
- when you assign a completely new value to an existing variable, Rust will call drop and free the original value's memory immediately.

```rust
let mut s = String::from("hello");
s = String::from("ahoy");

println!("{s}, world");
```

#### Variables and Data Interacting with Clone
- if we want to deeply copy the heap data of the String; not just the stack data, we can use a method called __clone__

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {s1}, s2 = {s2}");
```

- note: for data stored on the stack,we can do a copy by assigning a variable to another - we do not need to clone. the reason is that such a call is inexpensive since we are dealing with the stack.

#### Stack only data: Copy
- these types implement copy
    - all integer types eg u32
    - boolean
    - floating-point types
    - character type
    - tuples if they only contain types that also implement copy

### Ownership and Functions
- the mechanics of passing a value to a function are similar to those when assigning a value to a variable.
- passing a variable to a function will move or copy just as assignment does.

```rust
fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // because i32 implements the copy trait, x does not move into the function, so its okay to use x afterward
}
// here x goes out of scope, then s. however because s's value was moved, nothing special happens

fn takes_ownership(some_string: String){ //some_string comes into scope 
    println!("{some_string}");
} // here, some_string goes out of scope and drop is called, the backing memory is freed.

fn makes_copy(some_integer: i32){ // some_integer comes into scope
    println!("{some_integer}");
} // here some_integer goes out of scope, nothing special happens
```

### Return Values and Scope
- returning values can also transfer ownership

```rust
fn main() {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
} // here s3 goes out of scope and is dropped, s2 was moved so nothing happens, s1 goes out of scope and is dropped

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it

    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String{
    // a_string comes into scope
    a_string //a_string is returned and moves out to the calling function
}
```

- the ownership of a variable follows the same pattern every time: assigning a value to another variable moves it.
- when a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.
- __complexity: taking ownership and returning ownership with every functionis a bit tedious; what if we want to let a function use a value but not take ownership?__

#### Solution 1: returning multiple values using tuple
```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of {s2} is {len}.");
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();

    (s, length)
}
```

##### Solution 2: References

### References and Borrowing
- a reference is like a pointer in that it's an address we can follow to access the data stored at that address
- unlike a pointer, a reference is guaranteed to point to a valid value for a particular type for the life of that reference
- here's how calculate_length() could be refactored using a reference instead of taking ownership of the value

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize{
    s.len();
}
```

- the ampersands represent references and they allow you to refer to some value without taking ownership of it.
- we call the action of creating a reference __borrowing__
- __what happens when we try to modify something that we borrowed?__

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String){
    some_string.push_str(", world");
}
```
- this will cause an error - just as variables are immutable by default, so are references.

#### Mutable References
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String){
    some_string.push_str(", world");
}
```
- mutable references have 1 big restriction - __if you have a mutable reference to a value, you can have no other references to that value__

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{r1}, {r2}");
```

- this will cause an error because we are referencing the same variable twice.
- the reason for this restriction is so that Rust can prevent data races at compile time. a __data race__ is similar to a race condition and happens when these 3 behaviours occur
    - two or more pointers access the same data at the same time
    - at least one of the pointers is being used to write to the data
    - there's no mechanism being used to synchronize access to the data
- multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else's reading of the data.
- NOTE: a reference's scope starts from where it is introduces and continues through the last time that reference is used

```rust
let mut s = String::from("hello");

let r1 = &s; // immutable reference
let r2 = &s; // immutable reference
println!("{r1} and {r2}");
// variable r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{r3}");
```

#### Dangling References
- in languages with pointers, it is easy to create a __dangling pointer__
- a dangling pointer is a pointer that references a location in memory that may have been given to someone else - by freeing some memory while preserving a pointer to that memory
- in Rust, the compiler guarantees that references will never be dangling reference
- if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

#### Rules of Reference
- at any given time, you can have either one mutable reference or any number of immutable references
- references must always be valid