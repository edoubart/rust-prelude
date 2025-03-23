/*
 * Enums
 */
//#[derive(PartialEq)]
enum Musician {
    SingerSongwriter(String),
    Band(u32),
}

// Shortcuts
use Musician::{Band, SingerSongwriter};

/*
 * Implement the PartialEq Trait for Struct:
 *
 * pub trait PartialEq<Rhs = Self>
 * where
 *     Rhs: ?Sized,
 * {
 *     // Required method
 *     fn eq(&self, other: &Rhs) -> bool;
 *
 *     // Provided method
 *     fn ne(&self, other: &Rhs) -> bool { ... }
 * }
 */
impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        match self {
            SingerSongwriter(name) => match other {
                SingerSongwriter(other_name) => name == other_name,
                Band(_) => false,
            }
            Band(members) => match other {
                SingerSongwriter(_) => false,
                Band(other_members) => members == other_members,
            }
        }
    }
}

fn main() {
    // Singer Songwriters:
    let rustin_bieber: Musician = SingerSongwriter("Rustin".to_string());
    let rustin_timberlake: Musician = SingerSongwriter("Rustin".to_string());
    let holly: Musician = SingerSongwriter("Holly".to_string());

    // Bands:
    let rust_no_one: Musician = Band(5);
    let unrustworthy: Musician = Band(4);
    let rust_for_vengeance: Musician = Band(5);

    println!("{}", rustin_bieber == holly);
    println!("{}", rustin_bieber == rustin_timberlake);
    println!("{}", rustin_bieber == rust_no_one);

    println!("{}", rust_no_one == unrustworthy);
    println!("{}", rust_no_one == rust_for_vengeance);
}
