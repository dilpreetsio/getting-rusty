
## Intro

Rust is a static compiled language like C & C++ which uses ahead of time compilation unlike python and javascript.

Why learn?
- Its basically one language which has the performance of C & C++ with the modernity of a language like Java.
- One can create a server, client apps, lower language library or an embedded level app all using rust.
## Getting hands dirty:

```
fn main() {
	println!("Hello world!");
}
```

This is a simple rust program which prints hello world. The program always starts with the main function in rust, it gets executed first. It has no parameters and it doesn't return anything.

In the program above `println!` is a macro and not a function

## Cargo

Cargo is a package manager for rust similar to pip for python and npm for javascript.

What can cargo do?
- It can build the code
- Download dependencies (external libraries)
- Build the dependencies

In rust the packages of code are referred to as crates.
### All about cargo.toml

```
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]

```

The package section contains the name of the project, the version and the editor of rust used. The dependencies section contains the crates used in the project
##### Create a new project using cargo
```
cargo new {project_name}
```
##### Build the project

```
cargo build
```

This creates a new folder called `target` which contains the binary of the application. The program can be executed using `rustc /target/debug/{project name}`
##### Build and run the project using cargo run
```
cargo run
```
##### Check if the code compiles without building the project of the code.
```
cargo check
```
##### Building for release
```
cargo build --release
```
The release executables can be located at `target/release`


## Types
Rust is a static typed programming language which means that the type of a variable cannot change during the run-time. Unlike Javascript, rust wont let you add a string and a number type.


- Default type of a numerical data is `i32` i.e a signed integer 32 type




## Programming concepts in `rust`

##### Keywords
These are the reserved words for the language. We cannot use them for naming variables or functions. eg. `break`, `continue` etc

##### Variables
In Rust by default the variables are immutable, i.e we cannot change their values after declaring them.
We can declare variables in rust using the `let` keyword

```
let planets = 9;
```

However this creates a immutable variable i.e the value of the variable cannot be changed. In rust all the variables are *immutable* by default. To create a mutable variable we must use the keyword `mut` while declaring the variable.

```
let mut planets = 9;
```

##### Constants
Constants similar to immutable variables cannot be changed after they are declared. However there are differences between variables and constants.

- You cannot declare a constant with `mut` keyword.
- Constants can only be set to constant expression and not the result, that can be computed at runtime

Constants area valid for the entire time when the program runs within the scope it is declared in.

```
const HELLO = "WORLD";
```
##### Shadowing
Shadowing lets you declare a new variable with the same name as a previous variable. Once a new variable with the same name is declared it overshadows the previous one with in the scope it is being used.

```
let x = 4;

let x = x + 1;

{
	let x = 3; // it overshadows x within the scope
	println!(x);  // this results in the value being 3
}

println!(x); // this results in the value being 5
```

Shadowing and mutable variables are different once redeclared the new variable will remain immutable. Shadowing also lets us change the variable type when redeclaring the variable which is not possible with mutable variables.

```
let spaces = "    "; // string type
let spaces = spaces.len(); // integer type
```

This cannot be done with mutable variables as the type is set when declaring the variable which cannot be changed after declaration.


##### Data types
Every value in rust is of a certain data type as it is a static typed language which means the rust compiler will set the data types at the compile time which don't change at the runtime.

###### Scaler types
These represent a single value such as numbers, booleans etc. Rust has four primary scaler types

- **Integer**
	Integers like used to represent a number without fractional part. In rust it can be signed and unsigned. They length supported in rust varies from 8 bit to 128 bit. In addiction to these length we have `isize` & `usize` which depend on the architecture of the computer and can be 32 or 64 bit sized.

- **Floating point**
	Rust has 2 types of floating point numbers `f32` & `f64` which are 32 bits and 64 bits in size respectively. The default type is `f64` i.e 64 bits .

- **Booleans**
	Rust provides a `bool` type as in other languages

- **Characters**
	As in C & C++ we have a character type which can be declared using single quotes.
	The character types are of 4 byte size & represents more than ASCII, including Chinese, Japanese, emojis and more.

	```let c: char = 'c';```


###### Compound types
Compound types can group multiple values into one type. Rust has two primitive compound types. Tuple and arrays

- **Tuple**
	They can be used when you want to group together a number of different value types into one type. Tuples have a fixed length i.e the size of a tuple cannot be changed after declaring it.

```
fn main() {
	let tup: (i32, bool, char) =
}
```
Match statements
It matches the return value from a function and tries to match to an arm that is satisfy the condition.

By default all the variables are immutable similar to a const type in javascript, to create a mutable variable you have to use `mut` keyword while declaring the variable.
