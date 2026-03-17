#[allow(dead_code)]
pub fn infinite_loop() {
    let mut count = 0;
    loop {
        count += 1;
        println!("count: {}", count);
        if count == 5 {
            println!("Breaking at 5");
            break; 
        }
    }
}

#[allow(dead_code)]
pub fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

#[allow(dead_code)]
pub fn for_loop() {
   
    for i in 1..5 {
        println!("Value is: {}", i);
    }

   
    let fruits = ["Apple", "Banana", "Mango"];
    for fruit in fruits {
        println!("I like {}", fruit);
    }
}