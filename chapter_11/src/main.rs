//Generic Data Types
// use angle bracket to define generic type:
#[derive(Debug)]
struct Rectangle<T, U> {
    //multiple types require multiple generics
    width: T,
    height: U,
}

fn main() {
    let rect = Rectangle {
        width: 1.0,
        height: 3,
    };
    println!("rect is {:?}", rect);
}
//Generrics are a zero-cost abstraction
