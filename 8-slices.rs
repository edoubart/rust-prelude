fn main() {
    let mut cereals: [String; 5] = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];

    let first_two: &[String] = &cereals[0..2];
    dbg!(first_two);

    let mid_three: &[String] = &cereals[1..4];
    dbg!(mid_three);

    let last_three: &mut [String] = &mut cereals[2..];
    //dbg!(last_three);
    last_three[2] = String::from("Lucky Charm");
    dbg!(last_three);

    let cookie_crisp: &String = &cereals[0];
    dbg!(cookie_crisp);

    let cookie: &str = &cookie_crisp[0..6];
    println!("Cookie: {}", cookie);

    let cocoa_puffs: &String = &cereals[3];
    dbg!(cocoa_puffs);

    let puffs: &str = &cocoa_puffs[6..];
    println!("Puffs: {}", puffs);
}
