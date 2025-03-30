/*
 * Cargo Crates
 */
use crate::attractions::TicketSeller;
//use crate::attractions::{MovieTheatre, TicketSeller};

/*
 * Structs
 */
#[derive(Debug)]
struct VenueManagement<T: TicketSeller> {
    venue: T,
    manager: Option<String>,
}

impl<T: TicketSeller> VenueManagement<T> {
    // Dependency Injection
    fn new(venue: T) -> Self {
        Self {
            venue,
            // Inline creation (completely coupled)
            //venue: MovieTheatre::new(),
            manager: None,
        }
    }

    fn hire_manager(&mut self, manager: &str) {
        self.manager = Some(manager.to_string());
    }

    fn make_money(&mut self) {
        self.venue.sell_ticket();
    }
}

#[cfg(test)]
mod tests {
    /*
     * Cargo Crates
     */
    use super::*;
    use pretty_assertions::assert_eq;

    /*
     * Structs
     */
    struct DummyVenue {}

    impl TicketSeller for DummyVenue {
        fn sell_ticket(&mut self) {}
    }

    #[test]
    fn venue_management_can_hire_manager() {
        let dummy_venue: DummyVenue = DummyVenue {};
        let mut venue_management: VenueManagement<DummyVenue> =
            VenueManagement::new(dummy_venue);
        //let movie_theatre: MovieTheatre = MovieTheatre::new();
        //let mut venue_management: VenueManagement<MovieTheatre> =
        //    VenueManagement::new(movie_theatre);
        venue_management.hire_manager("Mario");
        assert_eq!(venue_management.manager.unwrap(), String::from("Mario"));
    }
}
