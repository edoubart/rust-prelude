/*
 * pub fn retain<F>(&mut self, mut f: F)
 * where
 *     F: FnMut(char) -> bool,
 * {
 *     ...
 * }
 */

fn main() {
    let mut game_console: String = String::from("PlayStation");
    let mut deleted_characters: String = String::new();

    // FnMut(char) -> bool
    let closure = |character: char| {
        let is_not_a: bool = character != 'a';

        if is_not_a {
            true
        } else {
            deleted_characters.push(character);
            false
        }
    };
    game_console.retain(closure);
    println!("{game_console}");
    println!("{deleted_characters}");

    //// Fn(char) -> bool
    //let closure = |character: char| character != 'a';
    //// FnMut > Fn
    //game_console.retain(closure);
    //println!("{game_console}");
}
