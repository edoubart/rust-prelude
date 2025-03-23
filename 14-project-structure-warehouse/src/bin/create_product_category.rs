// External Crates
use fake::{Fake, Faker};

// Custom Modules
use warehouse::ProductCategory;

fn main() {
    let random_category: ProductCategory = Faker.fake();
    println!("{:?}", random_category);
}
