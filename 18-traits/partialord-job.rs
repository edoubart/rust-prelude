/*
 * Cargo Crates
 */
use std::cmp::Ordering;

/*
 * Structs
 */
struct Job {
    salary: u32,
    commute_time: u32,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.salary == other.salary
    }
}

impl Eq for Job {}

/*
 * Implement the PartialOrd Trait for Struct:
 *
 * pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
 * where
 *     Rhs: ?Sized,
 * {
 *     // Required method
 *     fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
 */
impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Option 1:
        self.salary.partial_cmp(&other.salary)

        // Option 2:
        //match self.salary.partial_cmp(&other.salary) {
        //    Some(Ordering::Equal) => Some(Ordering::Equal),
        //    Some(Ordering::Less) => Some(Ordering::Less),
        //    Some(Ordering::Greater) => Some(Ordering::Greater),
        //    None => None,
        //}

        // Option 3:
        //if self.salary == other.salary {
        //    Some(Ordering::Equal)
        //}
        //else if self.salary < other.salary {
        //    Some(Ordering::Less)
        //}
        //else if self.salary > other.salary {
        //    Some(Ordering::Greater)
        //}
        //else {
        //    None
        //}
    }
}

fn main() {
    // Jobs
    let long_commute_job: Job = Job {
        salary: 100_000,
        commute_time: 2,
    };
    let short_commute_job: Job = Job {
        salary: 75_000,
        commute_time: 1,
    };

    println!("{}", long_commute_job > short_commute_job);
    println!("{}", long_commute_job < short_commute_job);
    println!("{}", long_commute_job == short_commute_job);
    println!("{}", long_commute_job >= short_commute_job);
    println!("{}", long_commute_job <= short_commute_job);
    // Methods: .lt(..), .le(..), .gt(..) and .ge(..)
}
