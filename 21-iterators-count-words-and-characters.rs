/*
 * Cargo Crates
 */
use std::collections::HashMap;
use std::str::SplitWhitespace;

fn count_characters(text: &str) -> HashMap<char, u32> {
    let words: SplitWhitespace<'_> = text.split_whitespace();
    let mut counts: HashMap<char, u32> = HashMap::new();

    for word in words {
        for character in word.chars() {
            let count: &mut u32 = counts.entry(character).or_insert(0);
            *count += 1;
        }
    }

    counts
}

fn count_words(text: &str) -> HashMap<&str, u32> {
    let words: SplitWhitespace<'_> = text.split_whitespace();
    let mut counts: HashMap<&str, u32> = HashMap::new();

    for word in words {
        let count: &mut u32 = counts.entry(word).or_insert(0);
        *count += 1;
    }

    counts
}

fn main() {
    let text: &str = "Sally sells sea shells by the sea shore.";
    println!("{:#?}", count_words(text));
    println!("{:#?}", count_characters(text));
}
