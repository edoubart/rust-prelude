/*
 * Structs
 */
#[derive(Debug)]
struct GasStation {
    snack_count: u32,
    manager: String,
    employee_count: u32,
}

fn main() {
    let mut points: [i32; 5] = [3, 8, 1, 11, 5];
    println!("Is sorted (before)? {}", points.is_sorted());

    points.sort();
    println!("{:?}", points);
    println!("Is sorted (after)? {}", points.is_sorted());

    points.reverse();
    println!("{:?}", points);
    println!("Is sorted (after reverse)? {}", points.is_sorted());

    // A > a
    let mut exercises: [&str; 3] = ["squat", "bench", "Deadlift"];
    //let mut exercises: [&str; 3] = ["squat", "bench", "deadlift"];
    exercises.sort();
    println!("{:?}", exercises);

    /*
     * Gas Stations:
     */
    let mobil: GasStation = GasStation {
        snack_count: 100,
        manager: String::from("Meg Mobil"),
        employee_count: 3,
    };

    let exxon: GasStation = GasStation {
        snack_count: 130,
        manager: String::from("Eric Exxon"),
        employee_count: 4,
    };

    let shell: GasStation = GasStation {
        snack_count: 50,
        manager: String::from("Shane Shell"),
        employee_count: 2,
    };

    let mut stops: [GasStation; 3] = [mobil, exxon, shell];
    println!("Stops (before): {:?}", stops);
    // error[E0277]: the trait bound `GasStation: Ord` is not satisfied
    //stops.sort();
    stops.sort_by_key(|station| station.snack_count);
    println!("Stops (after): {:?}", stops);

    println!("Stops (before): {:?}", stops);
    stops.sort_by_key(|station| !(station.employee_count as i32));
    //stops.sort_by_key(|station| !station.employee_count);
    println!("Stops (after): {:?}", stops);
}
