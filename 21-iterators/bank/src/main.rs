/***********
 * Structs *
 ***********/

// Attributes
#[derive(Debug)]
struct Account {
    id: u32,
    holder: String,
    balance: i32,
}

// Inherent Implementations
impl Account {
    // Associated Functions
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;

        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;

        self.balance
    }

    fn summary(&self) -> String {
        format!("{} has a balance of {}", self.holder, self.balance)
    }
}

// Attributes
#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

// Inherent Implementations
impl Bank {
    // Associated Functions
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts
            .iter()
            .map(|account| account.balance)
            .sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("me"));

    account.deposit(500);
    account.withdraw(250);

    bank.add_account(account);

    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
}
