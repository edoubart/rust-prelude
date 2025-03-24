#[derive(Debug)]
// Avoid dangling references:
struct TravelPlan<'a, 'b> {
    from: &'a str,
    to: &'b str,
}
//struct TravelPlan<'a> {
//    from: &'a str,
//    to: &'a str,
//}

fn main() {
    let from: String = String::from("Portland");

    let plan: &str = figure_out_ending_point(&from);
    //let plan: &str = {
    //    let to: String = String::from("Bangor");

    //    let travel_plan: TravelPlan = TravelPlan {
    //        from: &from,
    //        // Borrowed value `&to` does not live long enough!
    //        to: &to,
    //    };

    //    travel_plan.from
    //};

    println!("{plan}");
}

fn figure_out_ending_point(from: &str) -> &str {
    let to: String = String::from("Bangor");

    let travel_plan: TravelPlan = TravelPlan {
        from: &from,
        to: &to,
    };

    travel_plan.from
}
