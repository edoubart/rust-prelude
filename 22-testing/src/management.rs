/*
 * Cargo Crates
 */
use crate::attractions::{MovieTheatre, TicketSeller};

/*
 * Structs
 */
#[derive(Debug)]
struct VenueManagement {
    venue: MovieTheatre,
    manager: Option<String>,
}

impl VenueManagement {
    fn new() -> Self {
        Self {
            // Inline creation
            venue: MovieTheatre::new(),
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

    #[test]
    fn venue_management_can_hire_manager() {
        let mut venue_management: VenueManagement = VenueManagement::new();
        venue_management.hire_manager("Mario");
        assert_eq!(venue_management.manager.unwrap(), String::from("Mario"));
    }
}
