/*
 * Cargo Crates
 */
use pretty_assertions::assert_eq;

/*
 * Custom Modules
 */
use testing::attractions::Museum;
use testing::management::VenueManagement;


#[test]
fn venue_management_interacts_with_venue() {
    let mut museum: Museum = Museum::new();
    museum.buy_painting("Mona Lisa");

    let mut venue_management: VenueManagement<Museum>
        = VenueManagement::new(museum);
    venue_management.make_money();
    assert_eq!(venue_management.venue.revenue, 25);
}
