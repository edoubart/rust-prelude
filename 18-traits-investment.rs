/*
 * Traits
 */
// Supertrait (Parent Trait)
trait Investment<T> {
    // Getters (Read)
    fn amount(&self) -> T;

    // Associated Functions
    fn double_amount(&mut self);
}

// Taxable (Subtrait/Child Trait) inherits from the Investment Supertrait.
trait Taxable: Investment<f64> {
    // Associated Constants
    const TAX_RATE: f64 = 0.25;

    // Associated Functions
    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

/*
 * Structs
 */
#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Investment<f64> for Income {
    // Getters (Read)
    fn amount(&self) -> f64 {
        self.amount
    }

    // Associated Functions
    fn double_amount(&mut self) {
        self.amount *= 2.0;
    }
}

impl Taxable for Income {}

#[derive(Debug)]
struct Bonus {
    value: f64,
}

impl Investment<f64> for Bonus {
    // Getters (Read)
    fn amount(&self) -> f64 {
        self.value
    }

    // Associated Functions
    fn double_amount(&mut self) {
        self.value *= 2.0;
    }
}

impl Taxable for Bonus {
    // Associated Constants
    const TAX_RATE: f64 = 0.50;
}

#[derive(Debug)]
struct QualityTime {
    minutes: u32,
}

impl Investment<u32> for QualityTime {
    // Getters (Read)
    fn amount(&self) -> u32 {
        self.minutes
    }

    // Associated Functions
    fn double_amount(&mut self) {
        self.minutes *= 2;
    }
}

fn main() {
    let mut income = Income { amount: 50_000.50 };
    println!("Total tax owed (before): ${:.2}", income.tax_bill());
    income.double_amount();
    println!("Total tax owed (after): ${:.2}", income.tax_bill());

    let mut bonus = Bonus { value: 10_000.23 };
    println!("Total tax owed (before): ${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Total tax owed (after): ${:.2}", bonus.tax_bill());

    let mut weekend: QualityTime = QualityTime { minutes: 120 };
    println!("Relaxation time (before): {:.2} minutes", weekend.amount());
    weekend.double_amount();
    println!("Relaxation time (after): {:.2} minutes", weekend.amount());
}
