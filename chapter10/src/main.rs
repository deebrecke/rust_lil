#[derive(Debug)]

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}
fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0,
    };

    vehicle.crew_size = 6;

    let vehicle2 = Shuttle {
        name: String::from("Discovery"),
        ..vehicle
    };
    vehicle.crew_size = 5;
    println!("vehicle is {:?}", vehicle);
    println!("vehicle2 is {:?}", vehicle2);
}
