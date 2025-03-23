/*
 * Cargo Crates
 */
use std::clone::Clone;

/*
 * Structs
 */
#[derive(Clone Debug)]
struct Appointment {
    doctor: String,
    start_time: String,
    end_time: String,
}

impl Appointment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: doctor.to_string(),
            start_time: start_time.to_string(),
            end_time: end_time.to_string(),
        }
    }
}

/*
 * Implement the Clone Trait on Struct:
 *
 * pub trait Clone: Sized {
 *     // Required method
 *     fn clone(&self) -> Self;
 *
 *     // Provided method
 *     fn clone_from(&mut self, source: &Self) { ... }
 * }
 */
//impl Clone for Appointment {
//    fn clone(&self) -> Self {
//        Self {
//            doctor: self.doctor.clone(),
//            start_time: self.start_time.clone(),
//            end_time: self.end_time.clone(),
//        }
//    }
//}

fn main() {
    let morning_appointment: Appointment = Appointment::new(
        "Dr. Andrews",
        "9:00AM",
        "10:00AM",
    );
    let replacement_appointment: Appointment = morning_appointment.clone();
    println!(
        "{} is seeing the patient from {} to {}.",
        replacement_appointment.doctor,
        replacement_appointment.start_time,
        replacement_appointment.end_time,
    );
    println!("{morning_appointment:#?}");
}
