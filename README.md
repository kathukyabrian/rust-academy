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

### The Slice Type
- slices let you reference a contiguous sequence of elements in a collection.
- a slice is a kind of reference - it does not have ownership

#### String Slices
- reference to a contiguous sequence elements of a String and it looks like this

```rust
let s = String::from("hello world");
// h - 0
// e - 1
// l - 2
// l - 3
// o - 4
//   - 5
// w - 6
// o - 7
// r - 8
// l - 9 
// d - 10

let hello = &s[0..5];
let world = &s[6..11];
```

- rather than a reference to the entire string, hello is a reference to a portion of the String  specified in the extra [0..5] bit in the form of [starting_index..ending_index], where starting index of the first position in the slice and ending_index is one more than the last position in the slice
- internally, the slice data structure stores the starting position and the length of the slice which corresponds to ending_index - starting_index
- if you want to start at index 0, you can drop the starting index i.e
```rust
let s = String::from("hello");

let slice = &s[..2];
```

- if you want to end at the last element, you can also drop the ending index i.e
```rust
let s = String::from("hello");
let slice = &s[3..];
```

- the type that signifies a string slice is written as __&str__

#### String Literals as Slices
- string literals are slices
- this explains why they are immutable


## Using Structs to Structure Related Data
### Defining and Instantiating Structs
- to declare
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
- to create an instance
```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("some@example.com"),
        sign_in_count: 1,
    };
}
```

- to get a specific value, use the dot notation eg user1.email

- if the instance is mutable, we can change a value by using the dot notation and assigning into a particular field.

```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("some@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("another@example.com");
}
```

- NOTE: the entire instance must be mutable, Rust doesn't allow us to mark only certain fields as mutable/

### Using the Field Init Shorthand
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}
```

### Creating Instances with Struct Update Syntax
- helps in creating a struct from another of the same type then changing a few fields
```rust
fn main() {
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```
- the __..user1__ myst come last to specify that any remaining fields should get their values from the corresponding fields in user1 

### Creating Different Types with Tuple Structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

- the 2 structs Color and Point are of different types - they cannot be used for each other.
- they can be destructured like so:
```rust
let Point(x,y,z) = origin;
```

### Defining Unit-Like Structs
- you can define structs that don't have any fields!
- can be useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself.

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

### Ownership of Struct Data
- in the User struct definition we used the owned String type rather than the &str string slice type
- that was a deliberate choice since we want each instance of this struct to own all of its data and for the data to be valid for as long as the entire struct is valid.
- it's also possible for structs to store references to data owned by something else, but to do use requires the use of __lifetimes__
- lifetimes ensure that the data referenced by a struct is valid for as long as the Struct is
- example
```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User{
        active: true,
        username: "some@example.com",
        email: "some@example.com",
        sign_in_count: 1
    };
}
```

- the compiler will complain that it needs lifetime specifiers

### Method Syntax
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle{
        width: 30,
        height: 30,
    };

    println!("The area of the rectangle is {} square pixels", rect.area());
}
```

- to define the function within the context of the Rectangle, we start an impl block for Rectangle.
- everything within the impl block will be associated with the Rectangle type.
- in the signature for area, we use &self instead of rectangle: &Rectangle.
- the &self is actually short for self: &Self - it is a mandatory argument for such methods

### Associated Functions
- all functions defined within an impl block are called __associated functions__ because they are associated with the type named after the impl
- we can define associated functions that don't have self as their first parameter because they don't need an instance of the type to work with
- associated functions that aren't methods are often used for constructors that will return a new instance of the struct
- these are often called new but ne isn't a special name and isn't inbuilt into the language

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height size,
        }
    }
}
```

- the Self keywords in the return type and in the body of the function are aliases of the type that appears after the impl keyword which in this case is Rectangle
- to call this associated function we use the :: syntax with the struct name

```rust
let sq = Rectangle::square(3);
```

## Enums and Pattern Matching
### Defining an Enum
- give you a way of saying a value is one of possible set of values
```rust
enum IpAddrKind {
    V4,
    V6,
}
```

- to create instances
```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

- to use in a function
```rust
fn route(ip_kind: IpAddrKind){}
```

- enums can take an optional argument like so:
```rust
enum IpAddr{
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```
- here, we attach data to each variant of the enum directly. the name of each enum variant that we define also becomes a function that constructs an instance of the enum
- another advantage of using an enum is that each variant can have its own types and amounts of associated data. example:
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
``` 
- in this case an IPV4 addr could take four numeric components that have values between 0 and 255
- just like we are able to define methods on structs, we can do the same on enums using __impl__

```rust
impl Message {
    fn call(&self){

    }
}

let m = Message::Write(String::from("hello"));

m.call();
```

### The Option Enum
- encodes the very common scenario in which a value could be something or it could be nothing.
- an example: if you request the first item in a non-empty list, you would get a value.
- if you request the first item in an empty list, you would get nothing.
- Rust doesn't have the null feature that many other programming languages have. Null is a value that means there is no value there. In languages with null, variables can always be in one of two states, null or not null.
- the problem with a null value is that if you try to use a null value as a non null value, you will get an error of some kind.
- the way Rust treats this is with the Option enum which is defined as follows in the standard library
```rust
enum Option<T> {
    None,
    Some(T),
}
```
- examples using Option:
```rust
let some_number = Some(5);
let some_char = Some('c');

let absent_number: Option<i32> = None;
```
- note that i32 and Option\<i32> are different types
- the next question will be, how do we get the T from Option<T>
- the short answer is that you will need code that handles each variant: Some<T> and None 
- the __match__ expression is a contrl flow construct that does just this when used with enums 

### The Match Control Flow Construct
- much like the switch statement in other languages
```rust
enum Coin {
    Penny, 
    Nickel,
    Dime, 
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### Patterns that Bind to Values
```rust
#[derive(Debug)]
enum UsState {
    Alabama, 
    Alaska,
    // more
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```
- From 1999 to 2008, the US minted quarters with different designs for each of the 50 states - no other coins got the state design - this can be represented as shown above
- if we wanted to print the state from which a Quarter came from:
```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

#### The Option<T> match Pattern
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

#### Matches are Exhaustive
- the arm's patterns must cover all possibilities
- failure to do so will lead to an error

#### Catch All Patterns and the _ Placeholder
- sometimes we have specific matches that need to do sth specific and maybe all other matches to do the same thing
- example

```rust
let dice_roll = 9;

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat(){}

fn remove_fancy_hat(){}

fn move_player(num_spaces: u8){}
```

- we have 2 special arms for 3 and 7 and an __other__ arm that will match anything else - in this case the match is still exhaustive 
- please note that we have to put the catch-all arm last because the patterns are evaluated in order
- Rust has a pattern we can use if we want a catch-all but don't want to use the value in the catch-all pattern - the pattern is *_*

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

### Concise Control with if let and let...else
- the __if let__ syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest. consider this program
```rust
let config_max = Some(3u8);

match config_max {
    Some(max) => println!("The maximum value is configured to be {max}"),
    _ => (),
}
```
- if the value is Some, we print out the value in the Some variant by binding the value to the variable max in the pattern; we don't want to do anything with the None value
- we've had to add an handler for the None which is some boilerplate
- this can be rewritten using __if let__
```rust
let config_max = 3u8;
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
```
- the syntax if let takes a pattern and an expression separated by an equal sign - it works the same way as match where the expression is given to the match and the pattern is the first arm
- the code in the if let only runs if the value matches the pattern
- we can include an __else__ with an __if let__. the block of code that goes with the else is the same that would go with _

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State Quarter from {state:?}!"),
    _ => count += 1;
}
```

- this can be written as
```rust
let mut count = 0;

if let Coin::Quarter(state) = coin {
    println!("State Quarter from {state:?}!");
}else{
    count += 1;
}
```

## Common Collections
- unlike the builtin array and tuple types, data stored in these collections resides in the heap - this means that the amount of data that is stored doesn't have to be known at compile time - it can grow/shrink as the program runs
- 3 types to be discussed
    - vector -> allows you to store a variable number of values next to each other
    - string -> a collection of characters
    - hash map -> allows you to associate a value with a specific key

### Storing Lists of Values with Vectors
- allow you to store more than one value in a single data structure that puts all the values next to each other in memory
- vectors can only store values of the same type

#### Creating a New Vector
```rust
let v: Vec<i32> = Vec::new();
```
- vectors are implemented using generics
- vectors can be created with initial values
```rust
let v = vec![1,2,3];
```

#### Updating a Vector
```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

#### Reading Elements of Vectors
- there are 2 ways:
    - indexing
    - using get method
```rust
let vec = vec![1,2,3,4,5];
let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element"),
} 
```
- when we use the get option, we get an Option<&T> that we can use with match
- the 2 ways are provided so that you can choose how a program behaves when you try to use an index value outside the range of existing elements    

```rust
let v = vec![1,2,3,4,5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```
- when we run this program, the first example will cause the program to panic because it references a nonexistent element - best option if you want your application to crash
- the second example would match the None case without crashing

#### Iterating Over the Values in a Vector
```rust
let v = vec![100,32,57];
for i in &v {
    println("{i}");
}
```

- we can also iterate through mutable references 
```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

#### Using an Enum to Store Multiple Types
- vectors can only store values that are of the same type
- for this we can use enums
```rust
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadSheetCell::Int(3),
    SpreadSheetCell::Text(String::from("hello")),
    SpreadSheetCell::Float(10.12),
]
```

- all elements are of type __enum SpreadSheetCell__

### Storing UTF-8 Encoded Text with Strings
#### Defining Strings
- Rust has only 1 string type in the core language, which is the string slice __str__ that is usually seen in its borrowed from &str

#### Creating a New String
```rust
let mut s = String::new();
```
- this creates a new empty string called s, into which we can then load data
- if we have initial data with which we want to start the string, we can use __to_string__ method
```rust
let data = "initial contents";

let s = data.to_string();

let s = "initial contents".to_string();
```

- the above code is the same as
```rust
let s = String::from("initial contents");
```
- strings are UTF-8 encoded

#### Updating a String
- a string can grow in size and contents can change

##### Appending with push_str or push
```rust
let mut s = String::from("foo");
s.push_str("bar");
```
- the push_str method takes a string slice because we don't necessarily want to take ownership of the parameter

- the push method takes a single character as parameter and adds it to the String
```rust
let mut s = String::from("lo");
s.push('l');
```

##### Concatenating with + or format!
- if you want to combine 2 existing strings, you can use the + operator

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
```
- so why did we move s1 and used a reference for s2?:
    - this has to do with the signature of the method that's called when we use the + operator, it used the add method whose signature looks like
    ```rust
    fn add(self, s:&str) -> String {}
    ```
- format syntax is more readable
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{s1}-{s2}-{s3}");
```
- the format macro works luke println! but instead of printing the output, it returns a string with the contents

#### Indexing into Strings
- in many other programming languages, accessing individual characters by referencing them by index is a valid and common operation
- we don't do that here, if you attempt you will get an error

##### Internal Representation
- A String is a wrapper over a Vec<u8>
- example
```rust
let hello = String::from("Hola");
```
- in this case, len will be 4 which means that the vector stroing the string is 4 bytes long - each letter takes 1 byte when encoded in UTF-8
- the following line is not the same case
```rust
let hello = String::from("Здравствуйте");
```
- these are not 12 bytes, they are 24 - this is because each unicode scalar value in that string takes 2 bytes of storage
- therefore, an index into the string's bytes will not always correlate to a valid Unicode scalar value. consider this
```rust
let hello = String::from("Здравствуйте");
let answer = &hello[0];
```
- when encoded in UTF-8, the first byte of the first character(3) is 208 and the second is 151 - so it would seem that the answer should infact be 208, but 208 is not a valid character on its own
- therefore, to avoid returning unexpected value and causing bugs that might not be discovered immediately, Rust doesn't compile string indexing
- another reason Rust doesn't allow us to index a String to get a character is that indexing operations are expected to always take constant time (O(1)) - it is not possible to guarantee that performance with a String because Rust would have to walk through the contents to determine how many valid characters are there.

#### Slicing Strings
- rather than indexing using [] with a single number, you can use [] with  a range to create a string slice containing particular bytes
```rust
let hello = String::from("Здравствуйте");
let s = &hello[0..4]; // because each character is 2 bytes, we shall only get the first 2 characters
```
- here, s will be a &str that contains the first 4 bytes of the string - earlier we mentioned that each of these characters was 2 bytes, which means s will be __Зд__

#### Iterating Over Strings
- the best way is to be explicit on whether we want characters or bytes
- for individual unicode scalar values, use the chars method

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

- alternatively, the bytes method returns each raw byte
```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

### Storing Keys with Associated Values in Hash Maps
- stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory

#### Creating a New Hash Map
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```
- note: we need to import the HashMap - it is not brought up to the prelude automatically
- all keys must have the same type; the same for values

#### Accessing Values in a Hash Map
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```
- the get method returns an Option<&V>, if there's no value for that key in the hash map, get will return None
- this program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, then unwrap_or to set score to 0 if scores doesn't have an entry for the key.

---
- we can iterate over each key-value pair
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for(key, value) in &scores {
    println!("{key} : {value}");
}
```

#### Managing Ownership in Hash Maps
- for types that implement the Copy trait like i32, the values are copied into the hashmap
- for owned values like String, the values will be moved and the hashmap will be the owner of those values
```rust
use std::collections::HashMap;

let field_name = String::from("Favourite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point - they have been moved into the HashMap
```

#### Updating a HashMap
- when you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned 
    - you could replace the old value with the new value, completely disregarding the old value
    - you could keep the old value and ignore the new value, only adding the new value if the key doesn't already have a value

##### Overwriting a value
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{scores:?}");

```

##### Adding a Key and Value Only If a Key Isn’t Present
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{scores:?}");

```


##### Updating a Value Based on the Old Value

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{map:?}");

```

## Error Handling
- deals with issues when something goes wrong
- errors are grouped into 2:
    - recoverable
    - unrecoverable
- recoverable errors like a file not found error can be fixed by for example asking the user to reupload the file
- unrecoverable errors are usually bugs that would require the program to be stopped
- most languages don't distinguish between these two kind of errors and handle both in the same way using mechanisms such as exceptions
- Rust doesn't have exceptions - instead it has the type Result<T,E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable errors

### Unrecoverable Errors with panic!
- sometimes bad things happen and there's nothing you can do about it.
- in this case, rust has a __panic!__ macro
- there are 2 ways to cause a panic in practice:
    - by taking an action that causes our code to panic eg accessing an array past the end
    - by explicitly calling the panic! macro
- in both cases, we cause a panic
- by default, these panics will print a failure message, unwind, clean up the stack and quit
- via an env variable, you can have rust display the call stack when a panic occurs to make it easier to track down the source of the panic.

#### What exactly is this unwinding process
- when a panic occurs, the program starts unwinding
    - rust walks back up the stack and cleans up the data from each function it encounters - this proess of cleaning up is a lot of work - rust therefore allows you to choose the alternative of immediately aborting which ends the program without cleaning up
- memory that the program was using will then need to be cleaned up by the os
- if in your project, you need to make the resultant binary as small as possible, you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file eg
```toml
[profile.release]
panic = 'abort'
```

```rust
fn main() {
    panic!("crash and burn");
}
```
- in this case, the panic log is in our code, sometimes it will not
- in such a case we can use the backtrace of the function
```rust
fn main() {
    let v = vec![1,2,3];
    v[99];
}
```
- in this case, the program will panic because the vector only has 3 elements but we are trying to access the 100th element
- in C, attempting to read beyond the end of a data structure is undefined behavior - you might get whatever is at the location in memory that would correspond to that element in the data structure - even though the memory doesn't belong to that structure - this is called a __buffer overread__ and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn't be allowed to that is stored after the data structure
- Rust tries to fix this problem by failing

- with backtrace, we are able to see the complete steps that led to the error

### Recoverable Errors with Result
- not all errors are serious enough to require the program to stop entirely
- we have a Result enum that is defined as follows
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
- T and E are generic type parameters - T represents the type that will be returned in success cases while E represents the type of the error that will be returned in a failure case within the Err variant
- lets attempt calling a function that returns a Result value because the function could fail

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

- the return type of File::open is a Result<T, E>. 
- the generic parameter T has been filled by the implementation of File::open with the type of the success value __std::fs:File__ which is a file handle
- the type of E used in the error value is __std::io::Error__
- so this mean that the call to File::open might succeed and return a file handle that we can read from and write to and that the function might also fail 
- if the File::open call succeeds, the value in the greeting_file_result will be an instance of Ok that contains a file handle while if it is an error the value in greeting_file_result will be an instance of Err that contains more information about the kind of error that occurred.
- we need to use match to handle the 2 cases, Ok and Err
```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    }
}
```

- for specific error handling, we can match the error also
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}
```
- Error is a struct provided by the standard library - it has method kind that we can call to get an io::ErrorKind value
- the enum io::ErrorKind is an enum provided by the std library and has variants representing the different kind of errors that might result from an io operation
- the issue with the above example is that there's a lot of match - the solution to this is using closures

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
```

#### Shortcuts for Panic on Error
- there is a shortcut called unwrap()
- if the Result of the value is the Ok variant, unwrap will return the value inside the Ok and if it is the Err variant, it will call the panic! macro
```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

- there's a method called expect() that lets us choose the panic! error message

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

- in production-quality code, Rustaceans prefer .expect over .unwrap to give more context about why the operation is expected to always succeed

#### Propagating Errors
- when a function's implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do - this is known as __propagating__ the error and gives control to the calling method.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

#### The ? Operator Shortcut
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```
- works the same as the match
- eliminates the boiler plate
- to use it, the return type of the function has to be Result<T, E>

### To panic! or Not to panic!
#### Guidelines
- advisable to panic when its possible your code could end up in a bad state
    -  a bad state is when some assumption, guarantee, contract or invariant has been broken
- panic! is often appropriate when you're calling external code that is out of your control and returns an invalid state that you have no way of fixing

## Generic Types, Traits and Lifetimes
- functions can take parameters of a generic type instead of a concrete type like i32 or String -> __generics__
- __traits__ define behaviour in a generic way. you can combine traits with generic types to constrain a generic type to accept only those types that have a particular behavior as opposed to just any type
- __lifetimes__ are a variety of generics that give the compiler information about how references relate to each other - lifetimes allow us to give the compiler enough info about borrowed values so that it can ensure that references will be valid in more situations than it could without our help


### Removing Duplication by Extracting a Function
- consider
```rust
fn main() {
    let number_list = vec![34, 40, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}
```

- function checks the largets number in the list

- if we are asked to check for the largest number in another list, we could change it to:
```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}
```

- this code works but there's a problem - it is duplicated - tedious and error-prone
- to eliminate this problem we will create an abstraction 

```rust
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {result}");
}
```

- this works well, we have separated what changes from what does not - this can be achieved better with generics

### Generic Data Types
#### In Function Definitions
- assuming we have 2 functions, 1 that gets the largest i32 and another that gets the largest character as such:
```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34,50,25,100,65];
    let result = largest_i32(&number_list);
    println("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println("The largest char is {result}");
}
```

- the function bodies have the same code so we can eliminate this duplication
```rust
fn largest<T>(list: &[T]) -> &T{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}
```

- if we compile this code, we will get an error
```rust
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T` with trait `PartialOrd`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```
- the help text mentions ```std::cmp::PartialOrd``` which is a trait
- this means that the body of largest might not work for all possible types of T
- because we will be comparing values of type T in the body, we can only use types whose values can be ordered
- the solution in this case is to restrict the types valid for T to only those that implement ```PartialOrd```

#### In Struct Definitions
```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point{x : 5, y: 10};
    let float = Point{ x: 1.0, y: 4.0};
}
```
- like in the above case, the values of x and y have to be of the same type. if that is not the case, the code will fail.
- support for multiple generic types exist
```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point {x: 5, y: 10};
    let both_float = Point {x: 1.0, y: 4.0};
    let integer_and_float = Point { x: 5, y: 4.0};
}
```

#### In Enum Definitions
```rust
// example 1
enum Option<T> {
    Some(T),
    None,
}

// example 2
enum Result<T, E>{
    Ok(T),
    Err(E),
}
```
#### In Method Definitions
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

#### Performance of Code Using Generics
- doesn't make code run slower that it could with concrete types
- rust achieves this by doing __monomorphization__ of the code using generics at compile time
- monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled
- assuming we have such code
```rust
let integer = Some(5);
let float = Some(5.0);
```

- the following could be generated:
```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

### Defining Shared Behavior with Traits
- traits are like interfaces in other programming languages

#### Defining a Trait
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

#### Implementing a Trait on a Type
```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

- assuming that the trait was under an aggregator crate, we can use it this way:
```rust
use aggregator::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
}
```

#### Using Default Implementations
- sometimes its useful to have default behaviour for some or all methods in a trait instead of requiring implemenations for all methods
- in such cases, the definition would look like this:
```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```
- to use the default implementation, we define an empty implementation
```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}

use aggregator::{self, NewsArticle, Summary};

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}
```

#### Using Traits as Parameters
```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
- this accepts any type that implements the speficied trait
- in the body of notify, we can call any methods that come from the specified trait eg summarize
- any other type that doesn't implement summary is not allowed


##### Trait Bound Syntax
- the ```impl Trait``` syntax works for straight-forward cases but is actually syntax sugar for a longer form known as __trait bound__
```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
- this is the longer form of the example above
- this works better for more complex cases because if we had 2 parameters, the shorter form discussed above would look like this:
```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
```
- the longer format would look like:
```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {}
```

### Returning Types That Implement Traits
```rust
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}
```
- the ability to specify a return type only by the trait it implements is useful in the context of closures and iterators
- limitation: you can use ```impl Trait``` if you are returning a single type. the example below wouldn't work
```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            repost: false,
        }
    }
}
```

### Validating References with Lifetimes
- lifetimes ensure that references are valid as long as we need them to be.
- every reference in Rust has a lifetime - the scope for which that reference is valid.

#### Dangling References
- main aim of lifetimes is to prevent dangling references - they could cause a program to reference data other than the data it's intended to reference. example:
```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println("r: {r}");
}
```
- this code won't compile because r has gone out of scope before we try to use it. the error looks like this
```rust
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:13
  |
5 |         let x = 5;
  |             - binding `x` declared here
6 |         r = &x;
  |             ^^ borrowed value does not live long enough
7 |     }
  |     - `x` dropped here while still borrowed
8 |
9 |     println!("r: {r}");
  |                   - borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```
- it says that variable x does not live long enough - the reason is that x will be out of scope when the inner scope ends
- but r is still valid for the outer scope - its scope is larger - we can say it lives longer
- if rust allowed this code to work, r would be referencing memory that was deallocated when x went out of scope
- so how does Rust determine that this code is invalid? it uses a borrower checker

#### The Borrower Checker