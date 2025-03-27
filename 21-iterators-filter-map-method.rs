fn main() {
    let stocks: [&str; 6] = [
        "nvda",
        "",
        "appl",
        "",
        "msft",
        "ggog",
    ];

    //let capitalized_stocks = stocks
    //    .iter()
    //    .filter(|stock: &&&str| {
    //        !stock.is_empty()
    //    })
    //    .map(|stock: &&str| stock.to_uppercase())
    //    .collect::<Vec<String>>();
    //println!("{:?}", capitalized_stocks);

    /*
     * The `.filter.map()` method both filters and transforms a subset of elements
     * from an iterator.
     */
    let capitalized_stocks = stocks
        .iter()
        .filter_map(|stock: &&str| { // Returns an Option Enum.
            if stock.is_empty() {
                None
            } else {
                Some(stock.to_uppercase())
            }
        })
        .collect::<Vec<String>>();
    println!("{:?}", capitalized_stocks);
}
