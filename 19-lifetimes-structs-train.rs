#[derive(Debug)]
struct TrainSystem<'a> {
    name: &'a str,
    //name: String,
}

fn main() {
    let nj_transit: TrainSystem = {
        let name: String = String::from("NJ Transit");
        // Borrowed value `&name` does not live long enough!
        TrainSystem { name: &name } 
    }

    //let name: String = String::from("NJ Transit");
    //let nj_transit: TrainSystem = TrainSystem { name: &name };

    //let nj_transit: TrainSystem = TrainSystem {
    //    name: String::from("NJ Transit"),
    //};

    println!("{:#?}", nj_transit.name);
    //println!("{nj_transit:#?}");
}
