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