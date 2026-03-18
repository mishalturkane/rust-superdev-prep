#[derive(Debug)]
#[allow(dead_code)]
pub struct  Book {
    title: String,
    author: String,
    publication_year: u32,
}

#[allow(dead_code)]
pub fn structs() {
    let book = Book {
        title: String::from("Lets get rusty"),
        author: String::from("mishal"),
        publication_year: 2025,
    };

    println!("book struct is:\n{:#?}", book);
}

#[derive(Debug)]
#[allow(dead_code)]
struct TypeBook(String, String, u32);

#[allow(dead_code)]
pub fn tuple_type_book() {
    let book = TypeBook(String::from("Lets get rusty"), String::from("mishal"), 2025);
    println!("type book is:\n{:#?}", book);
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    fn is_square(&self) -> bool {
           self.width == self.height
       }
}

#[allow(dead_code)]
pub fn rectangle() {
    let rect = Rectangle {
        width: 20,
        height: 20,
    };

    println!("rectangle is:\n{:#?}", rect);
    println!("area: {}", rect.area());
    println!("perimeter: {}", rect.perimeter());
    println!("is square: {}", rect.is_square());
}
