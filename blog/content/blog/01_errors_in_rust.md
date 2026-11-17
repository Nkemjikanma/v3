+++
title = "Errors in Rust"
date = 2026-03-27
description = "A short description"
slug = "errors-in-rust"
draft = true
[extra]
image = ""
+++

### Introduction
Error handling is a heavily opinionated front; how we choose to react (or recover) when something goes wrong in our code should vary from person to person and project. That being said, there are bound to be best practices that align our code to the principles of the language being used. Good error handling shouldn't be limited to following idiomatic patterns and principles, we must also consider; code readability and target; ie is your code interfacing humans(applications) or is your code interfacing other code(ie library code). 

At the end of the article it'll be clear that my current focus is on library error design as it is what has inspired this writing, but it should cover enough information to shed light on errors as whole; both for application and libraries.

### What is an error in Rust? 
In Rust, anything that implements the `Debug`, `Display` and `std::error::Error` traits is considered an error. This makes more sense when you have a look at the code definition of error in the Rust library; 
```rust,name=src/core/error.rs
pub trait Error: Debug + Display {
    // ...
    fn source(&self) -> Option<&(dyn Error + 'static)> { None }
    fn provide<'a>(&'a self, request: &mut Request<'a>) {}
}
```
> The code snipped above is very minimal; here's the [source](https://github.com/rust-lang/rust/blob/main/library/core/src/error.rs) for a more detailed view. 

Don't let the trait definition scare you; 
- `Error::source` means something like, "a reference to an error can live as long of the program". 
- `Error::provide` is part of unstable code and is still in [experimental](https://github.com/rust-lang/rust/issues/99301), so we won't worry about this today.  

Errors in Rust are better seen as return values unlike other languages where error handling is exception based. So we can't consider errors without talkig about `Result` enum.
```rust,linenos
pub enum Result<T, E> {
    Ok(T), 
    Err(E), 
}
```
#### The Library Error 

#### The Application Error


panic causes the current task to unwind, and in most cases, the entire program aborts.)*
