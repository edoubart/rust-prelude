use std::cell::Cell;

/*
 * **Interior mutability** is a Rust design pattern where a value can mutate its
 * internal state even when accessed through an immutable reference.
 */

//#[derive(Debug)]
//struct ConcertTicket {
//    section: String,
//    seat: String,
//    scanned: bool,
//}

#[derive(Debug)]
struct ConcertTicket {
    section: String,
    seat: String,
    scanned: Cell<bool>, // For simple values that implement the Copy trait only.
}

impl ConcertTicket {
    fn new(section: String, seat: String) -> Self {
        Self {
            section,
            seat,
            scanned: Cell::new(false),
        }
    }

    fn admit_attendee(&self) {
        self.scanned.set(/*val:*/ true); // `set(..)` is a method available on Cell.
    }
}

fn main() {
    //let mut ticket: ConcertTicket = ConcertTicket {
    //    section: String::from("A"),
    //    seat: String::from("3"),
    //    scanned: false,
    //};

    //ticket.scanned = true; // error[E0594]: cannot assign to `ticket.scanned`, as `ticket` is not declared as mutable
    //ticket.section = String::from("Kangaroo"); // Problem: adding `mut` to ticket is a "All or Nothing" approach

    /*
     * Partial mutability
     */
    let ticket: ConcertTicket = ConcertTicket::new(/*section:*/ String::from("A"), /*seat:*/ String::from("3"));

    //println!("{ticket:#?}");
    println!("{}", ticket.scanned.get());

    ticket.admit_attendee();

    //println!("{ticket:#?}");
    println!("{}", ticket.scanned.get());
}
