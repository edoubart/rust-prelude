use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;

/*
 * The `RefCell<T>` type is useful when you're sure your code follows the
 * borrowing rules but the compiler is unable to understand and guarantee that.
 */

#[derive(Debug)]
struct ConcertTicket {
    section: String,
    seat: String,
    scanned: bool,
}

impl ConcertTicket {
    fn new(section: String, seat: String) -> Self {
        Self {
            section,
            seat,
            scanned: false,
        }
    }
}

fn main() {
    /*
     * Immutable from the compiler's perspective.
     */
    let ticket: RefCell<ConcertTicket> = RefCell::new(
        ConcertTicket::new(/*section:*/ String::from("A"), /*seat:*/ String::from("3"))
    );

    //println!("{ticket:#?}");

    ////let my_borrow: Ref<'_, ConcertTicket> = ticket.borrow();
    //let mut my_borrow: RefMut<'_, ConcertTicket> = ticket.borrow_mut();
    //my_borrow.scanned = true;

    //println!("{}", my_borrow.seat);
    //println!("{}", my_borrow.scanned);

    //println!("{ticket:#?}");

    //drop(my_borrow);

    //println!("{ticket:#?}");

    //ticket.borrow_mut().scanned = true;

    //println!("{ticket:#?}");

    ////let one: RefMut<'_, ConcertTicket> = ticket.borrow_mut();
    ////let two: RefMut<'_, ConcertTicket> = ticket.borrow_mut(); // thread 'main' (46416) panicked at src/main.rs:55:49: RefCell already borrowed

    //let one: Ref<'_, ConcertTicket> = ticket.borrow();
    //let two: Ref<'_, ConcertTicket> = ticket.borrow();

    //println!("{ticket:#?}");

    ticket.borrow_mut().seat = String::from("D");
    println!("{}", ticket.borrow().seat);

    ticket.borrow_mut().seat = String::from("K");
    println!("{}", ticket.borrow().seat);
  
    {
        let mut one: RefMut<'_, ConcertTicket> = ticket.borrow_mut();
        one.seat = String::from("D");
    }
    println!("{}", ticket.borrow().seat);

    ticket.borrow_mut().seat = String::from("K");
    println!("{}", ticket.borrow().seat);
}
