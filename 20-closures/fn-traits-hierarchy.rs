/***********************
 * Fn Traits Hierarchy *
 ***********************
 *
 * ----------
 * | FnOnce |
 * ----------
 *     - Closure captures values by **move** (transferring ownership);
 *     - Closure will be invoked once.
 *
 *     ^
 *     |
 *
 * ---------
 * | FnMut |
 * ---------
 *     - Captures values by **mutable reference**;
 *     - Closure can be invoked multiple times.
 *
 *     ^
 *     |
 *
 * ------
 * | Fn | (Strictest)
 * ------
 *     - Closure captures values by **immutable reference** (read-only) or doesn't
 *   capture anything at all;
 *     - Closure can be invoked multiple times.
 *
 */

fn execute_thrice<F>(mut procedure: F)
//fn execute_thrice<F>(procedure: F)
where
    F: FnMut() // Looser
    //F: Fn() // Strictest
{
    procedure();
    procedure();
    procedure();
}

fn bake_cake() {
    println!("Hello chocolate!");
}

fn main() {
    // Fn()
    //let closure_1 = || println!("I'm the boss.");
    //execute_thrice(closure_1);

    //let bosses: Vec<&str> = vec!["Bob"];
    //// FnOnce()
    //let closure_2 = || {
    //    let employees: Vec<&str> = bosses;
    //};
    //execute_thrice(closure_2);

    //let mut new_bosses: Vec<&str> = vec!["Bob"];
    //// FnMut()
    //let closure_3 = || {
    //    new_bosses.push("Alice");
    //};
    //execute_thrice(closure_3);

    //execute_thrice(bake_cake);

    let option: Option<Vec<String>> = None;
    let collection: Vec<String> = option.unwrap_or_else(Vec::new);
    println!("{:?}", collection);
}
