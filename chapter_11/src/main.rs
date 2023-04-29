//Box<T> Data Type
//Store data on heap instead of on the stack
//Smart Pointer: Provied additional functionality beyone references
//Box<T> has ownership of the data it points to
//When Box<T> goes out of scope, it deallocates the heap memory

use std::mem;

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    //standard implementation of Shuttle
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 83598.0,
    };
    println!("vehicle size on stack: {} bytes", mem::size_of_val(&vehicle)); //comes out to 40 bytes

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    println!("boxed_vehicle size on stack: {} bytes", mem::size_of_val(&boxed_vehicle));
    println!("boxed_vehicle size on heap: {} bytes", mem::size_of_val(&*boxed_vehicle));

    let unboxed_vehicle: Shuttle = *boxed_vehicle;

    println!("unboxed_vehicle size on stack: {} bytes", mem::size_of_val(&unboxed_vehicle));
    //Dereference Operator
    //represented with * symbol
    //when applied to a pointer it denotes the pointed-to location
}
