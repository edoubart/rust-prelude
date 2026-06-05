use chrono::prelude::*;
use chrono::TimeDelta;

fn main() {
    let event_data = vec![
        (
            "2025**04**19 !! 16:00:00 -04:00",
            "Started Rust study session",
        ),
        ("2025**04**20 !! 08:05:30 -04:00", "Made breakfast"),
        ("ERR", "ERR"),
        ("2025**04**22 !! 22:10:45 -04:00", "Went to bed"),
        ("ERR", "ERR"),
        ("2025**04**25 !! 09:00:03 -04:00", "Resumed Rust study"),
    ];

    let format: &'static str = "%Y**%m**%d !! %H:%M:%S %z";

    //let events = event_data
    //    .into_iter()
    //    .filter_map(|(timestamp, message)| {
    //      let parse_datetime = DateTime::parse_from_str(timestamp, format);
    //      match parse_datetime {
    //          Ok(datetime /*: DateTime<FixOffset>*/) => Some((datetime.with_timezone(/*tz:*/ &Utc), message)),
    //          Err(_) => None,
    //      }
    //    })
    //    .collect::<Vec<(DateTime<Utc>, &str)>>();

    let events: Vec<_> = event_data
        .into_iter()
        .filter_map(|(timestamp, message)| {
          let parse_datetime = DateTime::parse_from_str(timestamp, format);
          match parse_datetime {
              Ok(datetime /*: DateTime<FixOffset>*/) => Some((datetime.with_timezone(/*tz:*/ &Utc), message)),
              Err(_) => None,
          }
        })
        .collect();
    println!("{:?}", events);

    let mut previous_event: Option<DateTime<Utc>> = None;

    for (utc_datetime, message) in events {
        let display_time = utc_datetime.format("%Y-%m-%d %H:%M:%S");
        println!("Event time: {display_time}");
        println!("Event message: {message}");

        if let Some(previous_datetime/*: DateTime<Utc>*/) = previous_event {
            let difference: TimeDelta = utc_datetime - previous_datetime;
            let hours: i64 = difference.num_hours();
            let minutes: i64 = difference.num_minutes() % 60;
            let seconds: i64 = difference.num_seconds() % 60;
            println!("Time since previous event: {}h {}m {}s", hours, minutes, seconds);
        }

        println!();

        previous_event = Some(utc_datetime);
    }
}
