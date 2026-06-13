/*
 * A *trait object* is an instance of some type that implements a specific trait.
 * -> Runtime Polymorphism
 */

trait Wearable {
    fn wear(&self) -> String;
}

#[derive(Debug)]
struct Pants {
    fabric: String,
    waist_size: u32,
}

impl Wearable for Pants {
    fn wear(&self) -> String {
        format!("{} {} pants", self.waist_size, self.fabric)
    }
}

struct Tie {
    color: String,
}

impl Wearable for Tie {
    fn wear(&self) -> String {
        format!("{} tie", self.color)
    }
}

fn main() {
    let pants = Pants {
        fabric: "Cotton".to_string(),
        waist_size: 34,
    };

    let tie = Tie {
        color: "Red".to_string(),
    };

    // dyn -> "dynamic"
    // Rust can predict the memory footprint of that Vector thanks to Box.
    let outfit: Vec<Box<dyn Wearable>> = vec![Box::new(pants), Box::new(tie)];

    //for item in outfit {
    //    println!("Putting on the {}", item.wear());
    //}

    let items: Vec<String> = outfit.iter().map(|item /*: &Box<dyn Wearable>*/| item.wear()).collect();
    println!("{items:#?}");
}
