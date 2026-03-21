pub fn ownership() {
    let s1 = String::from("hello");
    s1_in_stack(&s1);
    s1_data_in_heap(&s1);
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

fn s1_in_stack(s: &String) {
    let address = s.as_ptr();
    let len = s.len();
    let cap = s.capacity();
    println!("Stack Metadata -> Address: {:p}, Len: {}, Cap: {}", address, len, cap);
}

fn s1_data_in_heap(s: &String) {
    println!("address in heap: {:p}",s.as_ptr());
    for i in 0..s.len() {
        println!("Heap Data -> Index: {}, Value: {}", i, s.chars().nth(i).unwrap());
    }
}