/*
 * Helpers
 */
fn make_money(text: &mut String) {
    text.push_str("$$$");
}

fn trim_and_capitalize(text: &str) -> String {
    text.trim().to_uppercase()
}

fn elements(text: &str) -> Vec<&str> {
    text.split("!").collect::<Vec<&str>>()
}

//fn get_identity() -> String {
//    use std::io;
//
//    let mut result: String = String::new();
//
//    let mut first_name: String = String::new();
//    let mut last_name: String = String::new();
//    println!("Please enter your first name and press Enter.");
//    println!("First name:");
//    match io::stdin().read_line(&mut first_name) {
//        Ok(_) => {
//            result.push_str(&first_name.to_string().trim());
//
//            println!("Please enter your last name and press Enter.");
//            println!("Last name:");
//            match io::stdin().read_line(&mut last_name) {
//                Ok(_) => {
//                    result.push(' ');
//                    result.push_str(&last_name.to_string().trim());
//                }
//                Err(err) => {
//                    println!(
//                        "Something went wrong reading user's last name: {}",
//                        err
//                    );
//                }
//            }
//        }
//        Err(err) => {
//            println!("Something went wrong reading user's first name: {err}");
//        }
//    }
//
//    result
//}

fn get_identity() -> String {
    use std::io;

    let mut first_name: String = String::new();
    let mut last_name: String = String::new();
    let input: io::Stdin = io::stdin();
    println!("Please enter your first name and press Enter.");
    input
        .read_line(&mut first_name)
        .expect("Something went wrong reading user's first name.");
    println!("Please enter your last name and press Enter.");
    input
        .read_line(&mut last_name)
        .expect("Something went wrong reading user's last name.");

    format!("{} {}", first_name.trim(), last_name.trim())
}

fn main() {
    /*
     * Make Money
     */
    //let mut text: String = String::from("Make Money");
    //println!("text (before): {}", text);
    //make_money(&mut text);
    //println!("text (after): {}", text);

    /*
     * Trim and Capitalize
     */
    //let text: &str = "  a random text   ";
    //let new_text: String = trim_and_capitalize(text);
    //println!("new text: {}", new_text);

    /*
     * Elements
     */
    //let text: &str = "Gold!Silver!Platinum";
    //let text_elements: Vec<&str> = elements(text);
    //println!("text_elements: {:#?}", text_elements);

    /*
     * Get Identity
     */
    let name = get_identity();
    println!("{name}");
}
