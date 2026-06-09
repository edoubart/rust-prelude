use regex::Regex;

fn main() {
    //let re: Regex = Regex::new(r"ue").unwrap();
    //let text: &'static str = "My movie queue";

    //match re.find(text) {
    //    Some(data/*: Match<'_>*/) => {
    //        println!("{data:?}");
    //        println!("{} {} {}", data.start(), data.end(), data.as_str());
    //    }
    //    None => println!("No matches found"),
    //}

    //for data/*: Match<'_>*/ in re.find_iter(/*haystack:*/ text) {
    //    println!("{} {} {}", data.start(), data.end(), data.as_str());
    //}

    //let re: Regex = Regex::new(r"\d").unwrap(); // "\d" -> any digit character
    //let re: Regex = Regex::new(r"\D").unwrap(); // The uppercase letter is doing the opposite.
    //let re: Regex = Regex::new(r"\w").unwrap(); // "\w" -> any alphanumeric character
    //let re: Regex = Regex::new(r"\W").unwrap();
    //let re: Regex = Regex::new(r"\s").unwrap(); // "\s" -> any space character
    //let re: Regex = Regex::new(r"\S").unwrap();
    //let re: Regex = Regex::new(r"\b").unwrap(); // "\b" -> "boundary" ("word boundary")
    //let re: Regex = Regex::new(r"\bc").unwrap(); // word starting with c
    //let re: Regex = Regex::new(r"e\b").unwrap(); // word ending with e
    //let re: Regex = Regex::new(r".").unwrap(); // "meta-character", "." -> any character whatsoever
    //let re: Regex = Regex::new(r"i.").unwrap();
    //let re: Regex = Regex::new(r"\.").unwrap(); // "\." -> any period
    //let re: Regex = Regex::new(r"h\.").unwrap();
    //let re: Regex = Regex::new(r"\b[a]").unwrap(); // Set/sequence of characters to be matched
    //let re: Regex = Regex::new(r"\b[abcdefjhijkl]").unwrap();
    //let re: Regex = Regex::new(r"\b[a-l]").unwrap();
    //let re: Regex = Regex::new(r"\b[A-La-l]").unwrap();
    //let re: Regex = Regex::new(r"\b[A-La-l]\b").unwrap();
    //let re: Regex = Regex::new(r"\d{2}").unwrap(); // Number of preceding matches
    //let re: Regex = Regex::new(r"\d{3}").unwrap(); // Excatly 3
    //let re: Regex = Regex::new(r"\d{3,}").unwrap(); // At least 3
    //let re: Regex = Regex::new(r"\d{3,6}").unwrap();
    //let text: &'static str = "My ZIP code is 90210. I am very rich.";

    //For data/*: Match<'_>*/ in re.find_iter(/*haystack:*/ text) {
    //    println!("{:?}", data);
    //}

    //let re: Regex = Regex::new(r"\d{3}-\d{3}-\d{4}").unwrap();
    //let text: &'static str = "Hey, this is Bob. Your program sucks. I can be reached at 555-123-4567 and I'd appreciate a call.";

    //let re: Regex = Regex::new(r"\d{3}\s\d{3}\s\d{4}").unwrap();
    //let text: &'static str = "Hey, this is Bob. Your program sucks. I can be reached at 555 123 4567 and I'd appreciate a call.";

    // "\s{1,} is equivalent to "\s+"

    //let re: Regex = Regex::new(r"\d{3}\s{1,}\d{3}\s{1,}\d{4}").unwrap();
    //let text: &'static str = "Hey, this is Bob. Your program sucks. I can be reached at 555 123     4567 and I'd appreciate a call.";

    //let re: Regex = Regex::new(r"\d{3}\s+\d{3}\s+\d{4}").unwrap();
    //let text: &'static str = "Hey, this is Bob. Your program sucks. I can be reached at 555 123     4567 and I'd appreciate a call.";

    //let re: Regex = Regex::new(r"\d{3}(\s+|-)\d{3}(\s+|-)\d{4}").unwrap();
    //let text: &'static str = "Hey, this is Bob. Your program sucks. I can be reached at 555-123     4567 and I'd appreciate a call.";

    //let re: Regex = Regex::new(r"\d+$").unwrap();
    //let text: &'static str = "/users/1/posts/353";

    //let re: Regex = Regex::new(r"^/v\d+").unwrap();
    //let text: &'static str = "/v3/items/v2/prices/v9";

    //for data/*: Match<'_>*/ in re.find_iter(/*haystack:*/ text) {
    //    println!("{:?}", data);
    //}

    //let re: Regex = Regex::new(r"(\d+)(.+)(\w{2})").unwrap(); // (...) -> "Capture Group"
    //let text: &'static str = "123 Elm Street, Palm Springs, CA";
    //let captures = re.captures(text).unwrap();
    //println!("{}", &captures[0]);
    //println!("{}", &captures[1]);
    //println!("{}", &captures[2]);
    //println!("{}", &captures[3]);

    //let re: Regex = Regex::new(r"(?<street_number>\d+)(.+)(?<state>\w{2})").unwrap(); // (.+) ->
    //"Greedy Search"
    //let text: &'static str = "123 Elm Street, Palm Springs, CA";
    //let captures = re.captures(text).unwrap();
    //println!("{}", &captures["street_number"]);
    //println!("{}", &captures["state"]);

    //let re: Regex = Regex::new(r"(?<street_number>\d+)(.+?)(?<state>\w{2})").unwrap(); // "Non-greedy"
    //let text: &'static str = "123 Elm Street, Palm Springs, CA";
    //let captures = re.captures(text).unwrap();
    //println!("{}", &captures["street_number"]);
    //println!("{}", &captures["state"]);

    //let re: Regex = Regex::new(r"\d+").unwrap();
    //let text: &'static str = "I have 2 apples and 10 bananas";
    //let result = re.replace_all(/*haystack:*/ text, /*rep:*/ "some muber of");
    //println!("{result}");

    //let re: Regex = Regex::new(r"(?<count>)\d+").unwrap();
    //let text: &'static str = "I have 2 apples and 10 bananas";
    //let result = re.replace_all(/*haystack:*/ text, /*rep:*/ "$count delicious");
    //println!("{result}");

    let re: Regex = Regex::new(r"(?<count>\d+)\s(?<fruit>\w+)").unwrap();
    let text: &'static str = "I have 2 apples and 10 bananas";
    let result = re.replace_all(/*haystack:*/ text, /*rep:*/ "$fruit ($count)");
    println!("{result}");
}
