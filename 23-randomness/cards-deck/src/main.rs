use rand::{rng, Rng};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

#[derive(Copy, Clone, Debug)]
enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

#[derive(Copy, Clone, Debug)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}

#[derive(Debug)]
struct Card {
    suit: Option<Suit>,
    rank: Rank,
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

// Associated Functions
impl Deck {
    // Constructor (the name `new` is up to us)
    fn new() -> Self {
        const ALL_SUITS: [Suit; 4] = [
            Suit::Clubs,
            Suit::Spades,
            Suit::Hearts,
            Suit::Diamonds
        ];
        const ALL_RANKS_MINUS_JOKER: [Rank; 13] = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace
        ];

        let mut cards: Vec<Card> = Vec::with_capacity(52);
        //for suit in &ALL_SUITS {
        //    for rank in &ALL_RANKS_MINUS_JOKER {
        //      let card = Card {
        //          rank: *rank,
        //          suit: Some(*suit),
        //      };
        //      cards.push(card);
        //    }
        //}

        for suit in ALL_SUITS.into_iter() {
            for rank in ALL_RANKS_MINUS_JOKER.into_iter() {
              let card = Card {
                  suit: Some(suit),
                  rank,
              };
              cards.push(card);
            }
        }

        Self {
            cards,
        }
    }
}

// Methods
impl Deck {
    fn shuffle(&mut self) -> &mut Deck {
        let mut my_rng: ThreadRng = rng();

        self.cards.shuffle(&mut my_rng);

        self
    }

    fn insert_jokers(&mut self) -> &mut Deck {
        let mut my_rng: ThreadRng = rng();

        let joker_1 = Card {
            suit: None,
            rank: Rank::Joker,
        };
        let card_count = self.cards.len();
        let joker_1_index: usize = my_rng.random_range(0..card_count).try_into().unwrap();
        self.cards.insert(joker_1_index, joker_1);

        let joker_2 = Card {
            suit: None,
            rank: Rank::Joker,
        };
        let joker_2_index: usize = my_rng.random_range(0..card_count).try_into().unwrap();
        self.cards.insert(joker_2_index, joker_2);

        //for _ in 0..2 {
        //    let random_index = my_rng.random_range(0..self.cards.len());
        //    let joker_card: Card = Card {
        //        suit: None,
        //        rank: Rank::Joker,
        //    };
        //    self.cards.insert(random_index, joker_card);
        //}

        self
    }

    fn delete_random_card(&mut self) -> &mut Deck {
        let mut my_rng: ThreadRng = rng();

        let should_delete_card: bool = my_rng.random_bool(0.65);
        println!("Should delete card? {:#?}", should_delete_card);

        if should_delete_card {
            let card_count = self.cards.len();
            let card_to_delete_index: usize = my_rng.random_range(0..card_count).try_into().unwrap();
            println!("Card to delete index: {:#?}", card_to_delete_index);
            self.cards.remove(card_to_delete_index);

            //let random_index: usize = my_rng.random_range(0..self.cards.len());
            //self.cards.remove(random_index);
        }

        self
    }
}

fn main() {
    let mut deck: Deck = Deck::new();

    println!("Before shuffle: {:#?}", deck);

    deck.shuffle();

    println!("After shuffle: {:#?}", deck);

    deck.insert_jokers();

    println!("After jokers insert: {:#?}", deck);

    deck.delete_random_card();

    println!("After deletion: {:#?}", deck);

    for _ in 0..10 {
        deck.delete_random_card();
    }

    let card_count = deck.cards.len();
    println!("Card count after deletion: {:#?}", card_count);
}
