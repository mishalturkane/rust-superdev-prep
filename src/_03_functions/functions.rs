#[allow(dead_code)]
pub fn take_something_return_something(a: i32, b: i32) -> i32 {
    a + b
}

#[allow(dead_code)]
pub fn take_nothing_return_nothing(){
    let a = 10; 
    let b = 20;
    let sum = a + b;
    println!("tnrn fn - the sum of a&b is: {}", sum);
}