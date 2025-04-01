use std::io::stdin;

#[derive(Debug)]
struct Vault {
    password: String,
    treasure: String,
}

impl Vault {
    // Option 1:
    fn unlock<F>(self, procedure: F) -> Option<String>
        where
            F: FnOnce() -> String,
        {
            let user_password: String = procedure();
            // Cannot invoke `procedure` twice!
            //procedure();
            if user_password == self.password {
                Some(self.treasure)
            } else {
                None
            }
        }

    // Option 2:
    //fn unlock<F: FnOnce() -> String>(self, procedure: F) -> Option<String> {
    //    ...
    //}

    // Option 3:
    //fn unlock(self, procedure: impl FnOnce() -> String) -> Option<String> {
    //    ...
    //}
}

fn main() {
    let vault: Vault = Vault {
        password: String::from("P@ssw0rd"),
        treasure: String::from("Gold"),
    };

    // Option 1:
    //let mut user_input: String = String::new();
    //println!("Please provide a password to crack the vault.");
    //stdin().read_line(&mut user_input);
    //user_input = user_input.trim().to_string();

    //// `impl FnOnce() -> String`
    //let hack = || user_input;

    // Option 2:
    // `impl Fn() -> String`
    let hack = || {
        let mut user_input: String = String::new();
        println!("Please provide a password to crack the vault.");
        stdin().read_line(&mut user_input);
        user_input.trim().to_string()
    };

    let extraction: Option<String> = vault.unlock(hack);
    println!("{:?}", extraction);
}
