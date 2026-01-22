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

### Functions

### Comments

### Control Flow