fn main() {
    // An English character occupies 1 byte in memory where an emoji occupies 4
    // bytes.
    let seafood: &str = "Oyster🦪";

    /*
     * The `.bytes()` method returns an iterator of the raw bytes.
     */
    for byte in seafood.bytes() {
        //for byte: u8 in seafood.bytes() {
        println!("{}/", byte);
    }
    println!("{}", seafood);
    println!("{}", seafood.bytes().len());

    /*
     * The `.chars()` method returns an iterator of the Unicode characters.
     */
    for character in seafood.chars() {
        //for character: char in seafood.chars() {
        println!("{}/", character);
    }
    println!("{}", seafood);
    println!("{}", seafood.chars().count());
}
