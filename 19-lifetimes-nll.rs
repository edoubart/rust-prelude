/*
 * Non-Lexical Lifetimes (NLL):
 * https://rust-lang.github.io/rfcs/2094-nll.html
 */

/*
 * Before Non-Lexical Lifetimes (NLL) was introduced.
 */
fn bar() {
    let mut data = vec!['a', 'b', 'c'];
    let slice = &mut data[..]; // <-+ 'lifetime
    capitalize(slice);         //   |
    data.push('d'); // ERROR!  //   |
    data.push('e'); // ERROR!  //   |
    data.push('f'); // ERROR!  //   |
} // <------------------------------+

/*
 * Alternative solution back then.
 */
fn bar() {
    let mut data = vec!['a', 'b', 'c'];
    {
        let slice = &mut data[..]; // <-+ 'lifetime
        capitalize(slice);         //   |
    } // <------------------------------+
    data.push('d'); // OK
    data.push('e'); // OK
    data.push('f'); // OK
}
