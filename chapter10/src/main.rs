#[derive(Debug, Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

//implementation block
impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name //don't forget that the & is a borrow operator
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0,
        }
    }
}
fn main() {
    let mut vehicle = Shuttle::new("Endeavour");
    let mut vehicle2 = Shuttle::new("Challenger");

    let vehicle_name = vehicle.get_name();
    println!("vehicle_name is {}", vehicle_name);
    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is now {}", vehicle.propellant);
    vehicle2.add_fuel(500.0);
    println!("vehicle 2 propellant: {}", vehicle2.propellant);
}
