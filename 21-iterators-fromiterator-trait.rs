/*
 * Cargo Crates
 */
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::RangeInclusive;

/*
 * Structs
 */
#[derive(Debug)]
struct Playlist {
    songs: Vec<String>,
    users: HashSet<String>,
}

impl FromIterator<(String, String)> for Playlist {
    // Required method
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut songs: Vec<_> = Vec::new();
        let mut users: HashSet<_> = HashSet::new();

        for (song, user) in iter {
            songs.push(song);
            users.insert(user);
        }

        Self { songs, users }
    }
}

fn main() {
    /*
     * pub trait FromIterator<A>: Sized {
     *     // Required method
     *     fn from_iter<T>(iter: T) -> Self
     *         where T: IntoIterator<Item = A>;
     * }
     */
    //let fifty_numbers: RangeInclusive<i32> = 1..=50;
    //println!("{:?}", fifty_numbers);

    //let results: Vec<i32> = Vec::from_iter(fifty_numbers.clone());
    //println!("{:?}", results);

    //let results: Vec<i32> = fifty_numbers.clone().collect::<Vec<i32>>();
    //println!("{:?}", results);

    //let unique_set: HashSet<_> = HashSet::from_iter(fifty_numbers.clone());
    //println!("{:?}", unique_set);

    //let unique_set: HashSet<i32> = fifty_numbers.clone().collect::<HashSet<i32>>();
    //println!("{:?}", unique_set);

    //let chars: [char; 5] = ['H', 'e', 'l', 'l', 'o'];
    //println!("{:?}", chars);
    //let greeting: String = String::from_iter(chars);
    //println!("{:?}", greeting);

    let songs: [(String, String); 3] = [
        (String::from("I Rust Go On"), String::from("Bob")),
        (String::from("A Rust of Wind"), String::from("Bob")),
        (String::from("A Rustworthy Man"), String::from("Sheila")),
    ];
    println!("{:?}", songs);

    let playlist: Playlist = Playlist::from_iter(songs);
    println!("{:?}", playlist);

    //let playlist: Playlist = songs.into_iter().collect::<Playlist>();
    //println!("{:?}", playlist);
}
