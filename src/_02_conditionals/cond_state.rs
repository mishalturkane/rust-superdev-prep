#[allow(dead_code)]
pub fn account_status(a:bool) -> bool {
    if a {
        true
    } else {
        false
    }
}

#[allow(dead_code)]
pub fn grade(marks:i32){
    
   
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