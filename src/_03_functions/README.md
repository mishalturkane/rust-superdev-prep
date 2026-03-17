A function in programming is a reusable block of code that takes inputs, performs a specific task, and (optionally) returns an output.

so there is 4 types of functions in general programmig and these are

- take_something_return_something
- take_nothing_return_nothing
- take_something_return_nothing
- take_nothing_return_something

so lets perform one operation to practice a types of functions in rust

operation is : addition
values are :  a = 10, b = 20
 

### 1) take_something_return_something

```rust
pub fn take_something_return_something(a: i32, b: i32) -> i32 {
    a + b
}