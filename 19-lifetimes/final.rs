/*
 * This function does not need lifetime annotations because it accepts an
 * immutable reference as parameter, only one, and returns a completely separate
 * new datatype (decoupled separate data). No risk of dangling reference here.
 */
fn double_the_length<T>(collection: &Vec<T>) -> usize {
    collection.len() * 2
}

/*
 * This function does not need lifetime annotations because it accepts an
 * immutable reference as parameter, only one, and return an immutable
 * reference that still refers to the reference parameter. We don't need
 * lifetime annotations because the Borrow Checker (compiler part) has
 * everything it needs to figure out what the returned reference is associated
 * with. There is no new data being created in the body of the function and if
 * we returned a reference to that, that would be a dangling reference.
 */
fn last_two<T>(collection: &[T]) -> &[T] {
    let two_from_the_end: usize = collection.len() - 2;
    &collection[two_from_the_end..]
}

/*
 * This function needs lifetime annotations because it accepts two immutable
 * references as parameters, and returns an immutable reference. Multiple
 * reference parameters make that the Borrow Checker (compiler part) does not
 * know if the returned reference's lifetime relates to the referent's lifetime
 * of the first parameter or the second parameter.
 */
fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("Announcement: {}", announcement);
    &text[0..5]
}

/*
 * This function needs lifetime annotations because it accepts multiple
 * immutable references (3) as parameters, and returns an immutable reference.
 * Multiple reference parameters make that the Borrow Checker (compiler part)
 * does not know if the returned reference's lifetime relates to the referent's
 * lifetime of the first, the second or the third parameter. We do not need to 
 * mark the third parameter with the same lifetime annotation, because we have
 * no potential of returning `target`. `target` is just being used as the basis
 * for a calculation. Ultimately, the only thing that can be returned is either
 * `first` or `second`, and they are connected, so it makes sense to just mark
 * them with the same lifetime annotation and then connect that single lifetime
 * annotation to the final returned string slice.
 */
fn find_string_that_has_content<'a, 'a>(
    first: &'a str,
    second: &'a str,
    target: &str
) -> &'a str {
    let mut result: &str = "";

    if first.contains(target) {
        result = first;
    }
    else if second.contains(target) {
        result = second;
    }

    result
}

fn main() {
    println!("{}", double_the_length(&vec![1, 2, 3]));
    println!("{}", double_the_length(&vec![1, 2, 3, 4]));

    println!("{:?}", last_two(&vec![1, 2, 3]));
    println!("{:?}", last_two(&vec![1, 2, 3, 4, 5, 6]));

    println!("{}", first_five("refrigerator", "Hello"));

    println!(
        "{}",
        find_string_that_has_content("programming", "dining", "gram")
    );
}
