use std::any::type_name;

#[allow(dead_code)]

fn print_type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

#[allow(dead_code)]
pub fn variables() {
    let name = String::from("mishal turkane");
    let blood_group = "AB+";
    let roll_no: u32 = 34;
    let age: u8 = 24;
    let married: bool = false;
    let bank_balance: i64 = -69;

    println!("Value: {}, Type: {}", name, print_type_of(&name));
    println!("Value: {}, Type: {}", blood_group, print_type_of(&blood_group));
    println!("Value: {}, Type: {}", roll_no, print_type_of(&roll_no));
    println!("Value: {}, Type: {}", age, print_type_of(&age));
    println!("Value: {}, Type: {}", married, print_type_of(&married));
    println!("Value: {}, Type: {}", bank_balance, print_type_of(&bank_balance));
}