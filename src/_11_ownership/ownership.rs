pub fn ownership() {
    let s1 = String::from("hello");
    let address_of_s1 = s1.as_ptr();
    let s2 = s1;
    let address_of_s2 = s2.as_ptr();
    
    if address_of_s1 == address_of_s2 {
        println!("{:p}", address_of_s1);
        println!("{:p}", address_of_s2);
        println!("s1 and s2 share the same memory location");
        
    }else {
        println!("s1 and s2 do not share the same memory location");
    }
    
    println!("now s2 data is: {}",s2);
    
}