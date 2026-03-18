pub fn result_example() {
    let a = 10.0;
    let b = 2.0;

    let result = devide(a, b);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}

fn devide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("Division by zero".to_string());
    }
    Ok(a / b)
}