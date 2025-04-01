/*
 * Traits
 */
pub trait TicketSeller {
    fn sell_ticket(&mut self);
}

/*
 * Structs
 */
#[derive(Debug, Eq, PartialEq)]
pub struct Museum {
    pub paintings: Vec<String>,
    pub revenue: u32,
}

impl Museum {
    const MAXIMUM_CAPACITY: usize = 3;

    /// Creates a new Museum instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use testing::attractions::Museum;
    ///
    /// let museum: Museum = Museum::new();
    /// let empty_vec: Vec<String> = Vec::new();
    /// assert_eq!(museum.paintings, empty_vec);
    /// assert_eq!(museum.revenue, 0);
    /// ```
    pub fn new() -> Self {
        Self {
            paintings: vec![],
            revenue: 0,
        }
    }

    /// Buys a painting for the museum.
    ///
    /// # Examples
    ///
    /// ```
    /// use testing::attractions::Museum;
    ///
    /// let mut museum: Museum = Museum::new();
    /// museum.buy_painting("Mona Lisa");
    /// assert_eq!(museum.paintings, vec!["Mona Lisa".to_string()]);
    /// ```
    pub fn buy_painting(&mut self, painting: &str) {
        if self.paintings.len() >= Self::MAXIMUM_CAPACITY {
            panic!("Museum does not have storage space for another painting!");
        }

        self.paintings.push(painting.to_string());
    }

    fn has_impressive_collection(&self) -> bool {
        self.paintings.len() > 2
    }
}

impl TicketSeller for Museum {
    fn sell_ticket(&mut self) {
        if self.has_impressive_collection() {
            self.revenue += 35;
        } else {
            self.revenue += 25;
        }
    }
}

#[derive(Debug)]
pub struct MovieTheatre {
    pub movies: Vec<String>,
    pub sales: u32,
}

impl MovieTheatre {
    pub fn new() -> Self {
        Self {
            movies: vec![],
            sales: 0,
        }
    }

    pub fn add_movie(&mut self, movie: &str) {
        self.movies.push(movie.to_string());
    }
}

impl TicketSeller for MovieTheatre {
    fn sell_ticket(&mut self) {
        self.sales += 15;
    }
}

#[cfg(test)]
mod tests {
    /*
     * To **test** is "to check the quality, performance, or reliability of
     * something".
     *
     * **Tests** are code that validates that our program code works as
     * expected.
     *
     * An **assertion** is a verification that a statement is valid. To
     * **assert** means to state a fact.
     *
     * A **regression** is a bug introduced into working software.
     *
     * A test is a plain Rust function annotated with a `#[test]` attribute.
     *
     * A **unit test** targets a small component of a program in isolation.
     *
     * An **integration test** tests the interaction of multiple components within
     * the program.
     *
     * The **`assert!`** macro validates that some condition or value is true.
     *
     * The **`assert_ne!`** macro validates that two values are not equal.
     */

    /*
     * Cargo Crates
     */
    use super::*;
    use pretty_assertions::assert_eq;

    //#[test]
    //fn print_success() {
    //    // The following doesn't get printed as the test passes.
    //    println!("Success inside the function");
    //    assert!(true);
    //}

    //#[test]
    //fn print_failure() {
    //    println!("Failure inside the function");
    //    assert!(false);
    //}

    //#[test]
    //fn result_example() -> Result<(), String> {
    //    Ok(())
    //    //Err(String::from("Failure"))
    //}

    //#[test]
    //fn hashmaps() {
    //    let mut one: std::collections::HashMap<&str, i32> =
    //        std::collections::HashMap::new();
    //    one.insert("A", 2);
    //    one.insert("B", 3);

    //    let mut two: std::collections::HashMap<&str, i32> =
    //        std::collections::HashMap::new();
    //    two.insert("B", 3);
    //    two.insert("C", 4);

    //    assert_eq!(one, two);
    //}

    #[test]
    // Classic Test functions version:
    //pub fn museum_sells_ticket_to_increase_revenue() {
    //    let mut museum: Museum = Museum::new();
    //    museum.sell_ticket();
    //    assert_eq!(
    //        museum.revenue,
    //        25,
    //        "The revenue from selling 1 ticket did not match expectations."
    //    ); // Better test
    //    //assert_ne!(museum.revenue, 0); // Worse test
    //}

    // Result Enum version:
    pub fn museum_sells_ticket_to_increase_revenue() -> Result<(), String> {
        let mut museum: Museum = Museum::new();
        museum.sell_ticket();

        if museum.revenue == 25 {
            Ok(())
        } else {
            Err(
                String::from(
                    "The revenue from selling 1 ticket did not match expectations."
                )
            )
        }
    }

    #[test]
    pub fn museum_can_sell_mutiple_tickets() {
        let mut museum: Museum = Museum::new();
        museum.sell_ticket();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 50);
        //assert_eq!(museum.revenue, 25); // Test fails!
    }

    #[test]
    // Classic Test functions version:
    //pub fn museum_can_have_impressive_collection() {
    //    let mut museum: Museum = Museum::new();
    //    museum.buy_painting("Mona Lisa");
    //    museum.buy_painting("The Starry Night");
    //    museum.buy_painting("Girl with a Pearl Earring");
    //    assert!(
    //        museum.has_impressive_collection(),
    //        "The museum did not have an impressive collection despite having more than 2 paintings."
    //    );
    //}

    // Result Enum version:
    pub fn museum_can_have_impressive_collection() -> Result<(), String> {
        let mut museum: Museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry Night");
        museum.buy_painting("Girl with a Pearl Earring");

        if museum.has_impressive_collection() {
            Ok(())
        } else {
            Err(
                String::from(
                    "The museum did not have an impressive collection despite having more than 2 paintings."
                )
            )
        }
    }

    #[test]
    //#[ignore] // Don't add that to final test suite!
    pub fn new_museums_are_equal() {
        let museum_1: Museum = Museum::new();
        let museum_2: Museum = Museum::new();
        assert_eq!(
            museum_1,
            museum_2,
            "Two new Museum instances were not found to be equal: {museum_1:?} // {museum_2:?}"
        );

        //let mut museum_1: Museum = Museum::new();
        //museum_1.sell_ticket();
        //let mut museum_2: Museum = Museum::new();
        //assert_eq!(museum_1, museum_2);
    }

    #[test]
    #[should_panic(expected = "storage space")]
    pub fn museum_prohibits_adding_painting_when_capacity_has_been_reached() {
        let mut museum: Museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry Night");
        museum.buy_painting("Girl with a Pearl Earring");
        museum.buy_painting("Water Lilies"); // Should panic here!
    }

    /*
     * **Test-driven development (TDD)** is a paradigm that encourages you to write
     * your tests firsts.
     *
     * The mantra of TDD is "Red, Green, Refactor":
     *   1. Write a test that fails (Red);
     *   2. Make the code work (Green);
     *   3. Eliminate redundancy (Refactor).
     *
     * Advantages:
     *   - TDD will guarantee that you have tests for your code.
     *   - TDD reduces overtesting (writing the same test multiple times).
     *   - TDD forces you to define your contract before you write the code.
     *   - TDD simplifies the complexity of both the tests and the implementation.
     */
    #[test]
    pub fn museum_with_impressive_art_collection_charges_more_for_admission() {
        let mut museum: Museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry Night");
        museum.buy_painting("Girl with a Pearl Earring");

        museum.sell_ticket();

        assert_eq!(museum.revenue, 35);
    }
}
