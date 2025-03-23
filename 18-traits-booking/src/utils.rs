// Custom Modules
use crate::lodging::{Accommodation, Description};
//use super::lodging::{Accommodation, Description};

/*
 * Utility Functions
 */

// Entity can be of any type as long as it implements the Accommodation trait.
// Syntactic Sugar
//fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
//    entity.book(guest, 1);
//}

// Trait Bound (Generic) syntax
pub fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

// Option 1:
//fn mix_and_match(
//    // The first parameter will be a mutable reference to some type that
//    // implements both, Accommodation and Description traits.
//    first: &mut (impl Accommodation + Description),
//    // The second parameter promises to implement only the Accommodation trait.
//    second: &mut impl Accommodation,
//    guest: &str,
//) {
//    first.book(guest, 1);
//    first.get_description();
//
//    second.book(guest, 1);
//}

// Option 2:
//fn mix_and_match<T: Accommodation + Description, U: Accommodation>(
//    first: &mut T,
//    second: &mut U,
//    guest: &str,
//) {
//    first.book(guest, 1);
//    second.book(guest, 1);
//}

// Option 3 (where clause):
pub fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    first.book(guest, 1);
    second.book(guest, 1);
}
