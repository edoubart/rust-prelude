#[derive(Debug)]
struct Account {
    balance: i32,
}

fn main() {
    let mut accounts: Vec<Account> = vec![
        Account { balance: 0 },
        Account { balance: 11 },
    ];

    match accounts.first_mut() {
        Some(account) => {
            account.balance = 30;
            println!("Account: {:#?}", account);
        },
        None => println!("No account found"),
    }
}
