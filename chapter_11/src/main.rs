//Generic Data Types
// use angle bracket to define generic type:
#[derive(Debug)]
struct Rectangle<T, U> {
    //multiple types require multiple generics
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 1,    //not specified u8--okay because 1 can be represented as a u8
        height: 3u8, //specified is okay too, but must be the same type as above: 3u16 will NOT work
    };
    println!("rect is {:?}", rect);
    println!("Width: {}", rect.get_width());
    println!("Perimeter: {}", rect.get_perimeter());
}
//Generics are a zero-cost abstraction
