mod _00_intro;
mod _01_variables;
mod _03_loops;
mod _04_functions;

fn main() {

   let sum = _04_functions::take_something_return_something(10,20);
   println!("tsrs fn - the sum of a&b is: {}", sum);
   
   _04_functions::take_nothing_return_nothing();
   
   _04_functions::take_something_return_nothing(10, 20);
   
   let sum = _04_functions::take_nothing_return_something();
   println!("tnrs fn - the sum of a&b is: {}", sum);
}