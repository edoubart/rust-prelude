#![allow(unused_variables)]
const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    let season: &str = "summer";

    #[allow(unused_assignments)]
    let mut points_scored: i32 = 28;
    points_scored = 35;

    let event_time: &str = "06:00";
    let event_time: i32 = 6;

    println!("Direct Interpolation... Season: {season}, Points Scored: {points_scored}, Event Time: {event_time} and Touchdown Points: {TOUCHDOWN_POINTS}");
    println!("Sequential Arguments... Season: {}, Points Scored: {}, Event Time: {} and Touchdown Points: {}", season, points_scored, event_time, TOUCHDOWN_POINTS);
    println!("Numeric Arguments... Season: {0}, Points Scored: {1}, Event Time: {2} and Touchdown Points: {3}", season, points_scored, event_time, TOUCHDOWN_POINTS);

    let _favorite_beverage: &str = "water";
}
