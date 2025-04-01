/*
 * Cargo Crates
 */
use std::collections::HashMap;

fn main() {
    let mut todos: HashMap<&str, bool> = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    //for (todo, completion_status) in todos {
    ////for (todo: &str, completion_status: bool) in todos {
    //    println!(
    //        "Task: {}. Complete: {}",
    //        todo, completion_status
    //    );
    //}

    //// error[E0382]: borrow of moved value: `todos`
    //println!("{:?}", todos);

    //for (todo, completion_status) in &todos {
    ////for (todo: &&str, completion_status: &bool) in &todos {
    //    println!(
    //        "Task: {}. Complete: {}",
    //        todo, completion_status
    //    );
    //}

    //println!("{:?}", todos);

    for (_, completion_status) in &mut todos {
    // A String Slice (`&str`) is not mutable in Rust.
    //for (_, completion_status: &mut bool) in &todos {
        *completion_status = true;
    }

    println!("{:?}", todos);
}
