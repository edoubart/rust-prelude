// Crates
/*
 * 'use' Statements:
 *   - Import code from crates (packages) or other files in the project;
 *   - It is possible to import multiple things on a single line using curly
 * braces.
 */
use rand::{rng, seq::SliceRandom};

// Attributes
/*
 * Attributes:
 *   - Provide extra instructions to the compiler;
 *   - 'derive' is an attribute. Tells the compiler to add additional code to
 * the struct;
 *   - 'Debug' is a trait. Has functions included that aid in debugging (like
 * printing a struct).
 */
#[derive(Debug)]
// Structs
/*
 * Structs:
 *   - Defines a collection of fields (data) that are related in some way;
 *   - Can be used to tie together data + functionality if we add an 'impl'
 * block.
 */
struct Deck {
    cards: Vec<String>
}

// Inherent Implementations
/*
 * Inherent Implementations:
 *   - Defines methods + associated functions (~= "class method", "static
 * method") tied to a struct;
 *   - The first argument determines whether we are making a method or an
 * associated function.
 */
// Adding functions to our struct.
impl Deck {
    // Associated Functions
    fn new() -> Self {
        // List of suits
        let suits = ["hearts", "spades", "diamonds"];
        // or
        //let suits = vec!["hearts", "spades", "diamonds"];

        // List of values
        let values = ["ace", "two", "three"];
        // or
        //let values = vec!["ace", "two", "three"];

        let mut cards = vec![];

        // Double nested for loop
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    // Methods, can be called on a struct instance.
    fn shuffle(&mut self) {
        // Random Number Generator

        // External crate use
        let mut rng = rng();

        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

// Main
/*
 * Main:
 *   - 'main' is called automatically when our program starts. It is the first
 * function that gets invoked when running a Rust program.
 */
fn main() {
    /*
     *   - 'let mut' is used when we want to make a variable that can be
     * reassigned or when we want to be able to change the value;
     *   - Here, 'deck' needs to be declared with the 'mut' keyword if we want
     * to change it in any way;
     *   - 'associated functions' called using the '::' syntax;
     *   - Here, we have a struct called 'Deck' above. 'Deck::new()' is how we
     * would call the 'new' function.
     */
    let mut deck = Deck::new();

    deck.shuffle();

    // Probably need to add error handling!
    /*
     *   - Error handling is critical; it is worth spending a lot of time
     * investing in it!
     */
    let cards = deck.deal(3);
    // or what happens if we do:
    //let cards = deck.deal(300);
    // ?!

    /*
     *   - {:#?} is a formatter. Prints a struct in a human-readable way.
     */
    println!("Here's your deck: {:#?}", deck);
    // or
    //println!("Here's your deck: {deck}");

    println!("Here's your hand: {:#?}", cards);
}
