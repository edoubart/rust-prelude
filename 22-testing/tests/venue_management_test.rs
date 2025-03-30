/*
 * Cargo Crates
 */
use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

/*
 * Custom Modules
 */
use testing::attractions::{MovieTheatre, Museum};
use testing::management::VenueManagement;

#[fixture]
fn museum_with_three_paintings() -> Museum {
    let mut museum: Museum = Museum::new();

    museum.buy_painting("Mona Lisa");
    museum.buy_painting("The Starry Night");
    museum.buy_painting("Girl with a Pearl Earring");

    museum
}

#[fixture]
fn movie_theater_with_one_movie() -> MovieTheatre {
    let mut movie_theater: MovieTheatre = MovieTheatre::new();

    movie_theater.add_movie("Titanic");

    movie_theater
}

#[fixture]
fn museum_management(museum_with_three_paintings: Museum)
    -> VenueManagement<Museum> {
        VenueManagement::new(museum_with_three_paintings)
    }

#[fixture]
fn movie_theater_management(movie_theater_with_one_movie: MovieTheatre)
    -> VenueManagement<MovieTheatre> {
        VenueManagement::new(movie_theater_with_one_movie)
    }

#[rstest]
//#[test]
fn venue_management_interacts_with_museum(museum_with_three_paintings: Museum) {
    //let mut museum: Museum = Museum::new();
    //museum.buy_painting("Mona Lisa");

    let mut venue_management: VenueManagement<Museum>
        = VenueManagement::new(museum_with_three_paintings);

    venue_management.make_money();

    assert_eq!(venue_management.venue.paintings.len(), 3);
    assert_eq!(venue_management.venue.revenue, 35);
}

#[rstest]
fn venue_management_interacts_with_movie_theater(
    movie_theater_with_one_movie: MovieTheatre
) {
    let mut venue_management: VenueManagement<MovieTheatre>
        = VenueManagement::new(movie_theater_with_one_movie);

    venue_management.make_money();

    assert_eq!(venue_management.venue.sales, 15);
}
