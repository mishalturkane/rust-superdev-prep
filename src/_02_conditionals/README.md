## Conditional Statements in Rust
### Conditional statements allow you to make decisions in your code. They control the flow of the program by executing specific blocks of code only if a certain condition is met.

Key Concepts:
if: The basic decision maker.

else if: Used to check multiple conditions.

else: The fallback block if no conditions are true.

if as an Expression: In Rust, if-else can return a value, which can be stored in a variable.

### 1) Boolean Decision (Simple If-Else)
This function takes a boolean value and returns the same based on a condition. It demonstrates how to return values directly from an if block.

```rust
#[allow(dead_code)]
pub fn account_status(a: bool) -> bool {
    if a {
        true
    } else {
        false
    }
}
```
### 2) Multi-Condition Logic (If-Else If-Else)
This function demonstrates how to handle multiple ranges using else if. It also shows the power of Rust's if as an expression, where the result of the if block is assigned directly to the grade variable.

```rust
#[allow(dead_code)]
pub fn grade(marks: i32) {
    let grade = if marks >= 95 {
        "A+"
    } else if marks >= 90 {
        "A"
    } else if marks >= 70 {
        "B"
    } else {
        "C"
    };

    println!("Grade: {}", grade);
}
```

Implementation in main.rs
To use these conditional logic functions, we first declare the module and then call them from the main function:

```rust
mod _02_conditionals;

fn main() {
    // Testing Account Status
    let result_active = _02_conditionals::account_status(true);
    println!("Account Active: {}", result_active);
    
    let result_inactive = _02_conditionals::account_status(false);
    println!("Account Active: {}", result_inactive);
    
    // Testing Grade Logic
    _02_conditionals::grade(95); // Output: A+
    _02_conditionals::grade(85); // Output: B
    _02_conditionals::grade(65); // Output: C
}
```