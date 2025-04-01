#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Subscription::Free => {
                println!("You have limited access to the site.");
            }
            Subscription::Basic(price, months) => {
                println!("You have limited access to the site's premium feature for {price} for {months} months.");
            }
            Subscription::Premium { tier } => {
                println!("You have full access to the site's premium features. Your tier is {tier:?}.");
            }
            // Catch-all arm
            //_ => unreachable!("Unexpected variants!"),
        }
    }
}

fn main() {
    let free_subscription: Subscription = Subscription::Free;
    free_subscription.summarize();

    let basic_subscription: Subscription = Subscription::Basic(19.99, 6);
    basic_subscription.summarize();

    let premium_subscription: Subscription = Subscription::Premium {
        tier: Tier::Gold,
    };
    premium_subscription.summarize();
}
