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
        Account { balance: 10 }
    ];

    /*******************
     * Type Annotation *
     *******************/
    // Option 1 (favorite here for the sake of consistency):
    let balances: Vec<i32> = accounts
        .iter()
        .map(|account| account.balance)
        .collect();

    // Option 2:
    //let balances = accounts
    //    .iter()
    //    .map(|account| account.balance)
    //    .collect::<Vec<_>>();

    // Option 3 (usual favorite):
    //let balances = accounts
    //    .iter()
    //    .map(|account| account.balance)
    //    .collect::<Vec<i32>>(); <- "Turbofish" :)

    println!("Balances: {:#?}", balances);
}
