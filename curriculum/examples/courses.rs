#[path = "../common/common.rs"]
mod common;

use common::{chosen, courses};
use curriculum::{DefaultTimeProvider, Query as _, TimeDelta};

fn main() {
    let courses = courses();
    let chosen = chosen();

    println!("Nearest 5 courses:");
    for course in courses
        .iter()
        .filter_by_name(&chosen)
        .nearest(DefaultTimeProvider, 5)
    {
        println!(
            "{}: {}",
            course.nearest_period(DefaultTimeProvider).unwrap(),
            course
        );
    }

    println!("Upcoming courses in 24 hours:");
    for course in courses
        .filter_by_name(&chosen)
        .upcoming(DefaultTimeProvider, TimeDelta::hours(24))
    {
        println!(
            "{}: {}",
            course.nearest_period(DefaultTimeProvider).unwrap(),
            course
        );
    }
}
