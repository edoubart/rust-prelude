use chrono::{
    NaiveDate,
    NaiveDateTime,
    NaiveTime,
    TimeDelta
};
use chrono::prelude::*;
use chrono_tz::America::Los_Angeles;
use chrono_tz::Tz;
use std::ops::{Add, Sub};

fn main() {
    let birthday: Option<NaiveDate> = NaiveDate::from_ymd_opt(1991, 4, 12);
    //let birthday: Option<NaiveDate> = NaiveDate::from_ymd_opt(1991, 4, 32); // Wrong date -> None
    //let birthday: &'static str = "1991-04-12";
    //let birthday = birthday.parse::<NaiveDate>().expect("Unable to parse NaiveDate from string");
    println!("{birthday:?}");
    let birthday: NaiveDate = birthday.unwrap();
    println!("{birthday:?}");

    let five_seconds: Option<TimeDelta> = TimeDelta::new(5, 0); // secs, nanos
    println!("{five_seconds:?}");

    let invalid: Option<TimeDelta> = TimeDelta::new(5, 1_000_000_000);
    println!("{invalid:?}");

    let negative_five_seconds: Option<TimeDelta> = TimeDelta::new(-5, 0); // secs, nanos
    println!("{negative_five_seconds:?}");

   let five_minutes: TimeDelta = TimeDelta::minutes(5); 
    println!("{five_minutes:?}");

   let five_hours: TimeDelta = TimeDelta::hours(5); 
    println!("{five_hours:?}");

   let five_days: TimeDelta = TimeDelta::days(5); 
    println!("{five_days:?}");

   let five_weeks: TimeDelta = TimeDelta::weeks(5); 
    println!("{five_weeks:?}");
    println!("{}", five_weeks.num_days());
    println!("{}", five_weeks.num_hours());
    println!("{}", five_weeks.num_minutes());

    let total_duration: TimeDelta = five_weeks + five_days + five_hours + five_minutes;
    println!("{total_duration:?}");

    println!(
        "{} weeks, {} days, {} hours, {} minutes",
        total_duration.num_weeks(),
        total_duration.num_days(),
        total_duration.num_hours(),
        total_duration.num_minutes(),
    );

    let birthday: NaiveDate = NaiveDate::from_ymd_opt(1991, 4, 12).unwrap();
    println!("{}", birthday.add(TimeDelta::days(5)));
    println!("{}", birthday + TimeDelta::days(5));

    println!("{}", birthday.add(TimeDelta::weeks(2) + TimeDelta::days(5)));
    println!("{}", birthday + TimeDelta::weeks(2) + TimeDelta::days(5));

    println!("{}", birthday - TimeDelta::weeks(3));

    //println!("{}", birthday + TimeDelta::days(100_000_000));

    let four_thirty_am: Option<NaiveTime> = NaiveTime::from_hms_opt(4, 30, 0);
    println!("{:?}", four_thirty_am);

    let four_thirty_pm: Option<NaiveTime> = NaiveTime::from_hms_opt(16, 30, 0);
    println!("{:?}", four_thirty_pm);

    let day: NaiveDate = NaiveDate::from_ymd_opt(1969, 7, 20).unwrap();
    let time: NaiveTime = NaiveTime::from_hms_opt(20, 17, 0).unwrap();
    let moon_landing: NaiveDateTime = NaiveDateTime::new(day, time);
    println!("{moon_landing}");

    println!("{}", moon_landing + TimeDelta::days(1000));
    println!("{}", moon_landing + TimeDelta::days(1000) + TimeDelta::minutes(45));

    let system_time: DateTime<Local> = Local::now();
    let utc_time: DateTime<Utc> = Utc::now();
    println!("{}", system_time.date_naive());
    println!("{}", utc_time.date_naive());
    println!("{}", system_time.time());
    println!("{}", utc_time.time());
    println!("{}", system_time.year());
    println!("{}", utc_time.year());
    println!("{}", system_time.month());
    println!("{}", utc_time.month());
    println!("{}", system_time.day());
    println!("{}", utc_time.day());
    println!("{}", system_time.hour());
    println!("{}", utc_time.hour());
    println!("{}", system_time.minute());
    println!("{}", utc_time.minute());
    println!("{}", system_time.second());
    println!("{}", utc_time.second());
    println!("{}", system_time.offset());
    println!("{}", utc_time.offset());

    let local_time: DateTime<Local> = Local::now();
    //let utc_time: DateTime<Utc> = local_time.with_timezone(&Utc);
    let la_time: DateTime<Tz> = local_time.with_timezone(&Los_Angeles);
    println!("{}", local_time);
    //println!("{}", utc_time);
    println!("{}", la_time);

    let someday: &'static str = "31-Oct-1995 18:07:54 -0600";
    let dt = DateTime::parse_from_str(someday, "%d-%b-%Y %H:%M:%S %z"); // format string
    println!("{dt:?}");

    let utc_time: DateTime<Utc> = Utc::now();
    println!("{}", utc_time.format("%m-%d-%Y"));
    println!("{}", utc_time.format("%m/%d/%Y"));
    println!("{}", utc_time.format("%m/%d/%y"));
    println!("{}", utc_time.format("%b %d, %y %H:%M:%S"));
    println!("{}", utc_time.format("%A %I:%M %p %Z"));
}
