/*
 * Structs
 */
#[derive(Debug)]
//#[derive(Debug, PartialEq)]
struct BusTrip {
    origin: String,
    destination: String,
    time: String,
}

impl BusTrip {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

/*
 * Implement the PartialEq Trait for Struct:
 *
 * pub trait PartialEq<Rhs = Self>
 * where
 *     Rhs: ?Sized,
 * {
 *     // Required method
 *     fn eq(&self, other: &Rhs) -> bool;
 *
 *     // Provided method
 *     fn ne(&self, other: &Rhs) -> bool { ... }
 * }
 */
impl PartialEq for BusTrip {
    fn eq(&self, other: &Self) -> bool {
        // Let's pretend two bus trips are equal if their itinerary are the same
        // regardless of time.
        self.origin == other.origin
            && self.destination == other.destination
    }
}

impl PartialEq<Flight> for BusTrip {
    fn eq(&self, other: &Flight) -> bool {
        // Let's pretend a bus trip and a flight are equal if they share the
        // same time.
        self.time == other.time
    }
}

#[derive(Debug)]
//#[derive(Debug, PartialEq)]
struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

/*
 * Implement the PartialEq Trait for Struct:
 *
 * pub trait PartialEq<Rhs = Self>
 * where
 *     Rhs: ?Sized,
 * {
 *     // Required method
 *     fn eq(&self, other: &Rhs) -> bool;
 *
 *     // Provided method
 *     fn ne(&self, other: &Rhs) -> bool { ... }
 * }
 */
impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        // Let's pretend two flights are equal if their itinerary are the same
        // regardless of time.
        self.origin == other.origin
            && self.destination == other.destination
    }
}

impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        // Let's pretend a flight and a bus trip are equal if they share the
        // same time.
        self.time == other.time
    }
}

fn main() {
    let flight_1: Flight = Flight::new("New York", "London", "08:00");
    let flight_2: Flight = Flight::new("New York", "London", "23:20");
    let flight_3: Flight = Flight::new("New York", "Los Angeles", "08:00");
    let flight_4: Flight = Flight::new("New York", "London", "08:00");
    println!("{}", flight_1 == flight_2);
    println!("{}", flight_1 == flight_4);
    println!("{}", flight_1.eq(&flight_2));
    println!("{}", flight_1.ne(&flight_2));
    println!("{}", flight_1 == flight_3);
    println!("{}", flight_1 != flight_3);

    let bus_trip_1: BusTrip = BusTrip::new("Los Angeles", "Tokyo", "08:00");
    println!("{}", flight_1 == bus_trip_1);
    println!("{}", bus_trip_1 == flight_1);
    println!("{}", bus_trip_1 == bus_trip_1);
}
