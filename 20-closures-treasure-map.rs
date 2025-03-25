#[derive(Debug)]
struct Location {
    name: String,
    treasures: u32,
}

struct Map<'a> {
    locations: &'a [Location],
}

impl<'a> Map<'a> {
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location)
    {
        let final_index: usize = self.locations.len() - 1;
        let mut current_index: usize = 0;
        while current_index <= final_index {
            let current_location: &Location = &self.locations[current_index];
            action(current_location);
            current_index += 1;
        }
    }
}

fn main() {
    let locations: [Location; 2] = [
        Location {
            name: String::from("Enchanted Forest"),
            treasures: 5,
        },
        Location {
            name: String::from("Mystic Mountain"),
            treasures: 10,
        },
    ];

    let map: Map = Map { locations: &locations };

    let mut total_treasures: u32 = 0;

    map.explore(|location: &Location| {
        total_treasures += location.treasures;
    });

    println!("Total treasures collected: {total_treasures}");

    let mut location_names: Vec<String> = Vec::new();

    map.explore(|location: &Location| {
        location_names.push(location.name.clone());
    });

    println!("{location_names:?}");
}
