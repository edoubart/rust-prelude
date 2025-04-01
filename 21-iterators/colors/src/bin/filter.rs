/***********
 * Structs *
 ***********/

// Attributes
#[derive(Debug)]
struct Account {
    balance: i32
}

/*****************
 * Main Function *
 *****************/
fn main() {
    let accounts: Vec<Account> = vec![
        Account { balance: 0 },
        Account { balance: 10 },
        Account { balance: -15 },
        Account { balance: 27 },
        Account { balance: -3 }
    ];

    let negative_accounts = accounts
        .iter()
        // Simple iterator that filters the elements of 'iter' with 'predicate'.
        // Just be careful with "Type Annotation".
        .filter(|account| account.balance < 0)
        .collect::<Vec<_>>();

    println!("Accounts with negative balance: {:#?}", negative_accounts);
}
