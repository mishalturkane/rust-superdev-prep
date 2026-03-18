#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    name: String,
    blood_group: String,
    roll_no: u32,
    age: u8,
    married: bool,
    bank_balance: i64,
}

#[allow(dead_code)]
pub fn structs(){
    
    let person = Person {
        name: String::from("Mishal Turkane"),
        blood_group: String::from("AB+"),
        roll_no: 34,
        age: 22,
        married: false,
        bank_balance: 0,
    };
       
    println!("person struct is: {:?}",person);
    println!("person struct is:\n{:#?}", person);
    
}