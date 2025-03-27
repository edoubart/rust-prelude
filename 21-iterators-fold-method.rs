/*
 * Cargo Crates
 */
use std::collections::HashMap;

/*
 * Structs
 */
struct SupportStaff {
    day: String,
    employee: String,
}

fn main() {
    let earnings: [i32; 4] = [4, 7, 9, 13];

    /*
     * The `.fold()` method exhausts an iterator to build up and produce a
     * single value at the end of the iteration.
     *
     * ```
     * .fold(
     *     starting_value,
     *     |accumulator, current_element| { ... }
     *  )
     * ```
     */
    let sum: i32 = earnings
        .into_iter()
        .fold(0, |total, current| {
            println!("Total: {}, current: {}", total, current);
            total + current
        });
    println!("{}", sum);

    let week: [SupportStaff; 3] = [
        SupportStaff {
            day: String::from("Monday"),
            employee: String::from("Brian"),
        },
        SupportStaff {
            day: String::from("Tuesday"),
            employee: String::from("Cam"),
        },
        SupportStaff {
            day: String::from("Wednesday"),
            employee: String::from("Walter"),
        },
    ];

    let map: HashMap<String, String> = week
        .into_iter()
        .fold(HashMap::new(), |mut data, entry| {
            data.insert(entry.day.clone(), entry.employee.clone());
            data
        });
    println!("{:?}", map);
}
