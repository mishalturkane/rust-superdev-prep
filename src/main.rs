mod _09_errors;

fn main() {
    match _09_errors::divide(1.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {:?}", err),
    }
}