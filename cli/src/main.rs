mod common;

use clap::Parser;
use clap_derive::Parser;
use common::*;
use curriculum::*;

#[derive(Debug, Parser)]
#[clap(version)]
struct Cli {
    /// nearest courses number
    #[arg(short, long, default_value_t = 5)]
    nearest: usize,
    /// nearest courses in hours
    #[arg(short, long, default_value_t = 24)]
    in_hours: usize,
}

fn main() {
    let cli = Cli::parse();
    let courses = courses();
    let chosen = chosen();

    println!("Nearest {} courses:", cli.nearest);
    for course in courses
        .iter()
        .nearest(DefaultTimeProvider, cli.nearest)
        .filter_by_name(&chosen)
    {
        println!(
            "{}: {}",
            course.nearest_period(DefaultTimeProvider).unwrap(),
            course
        );
    }

    println!("Upcoming courses in {} hours:", cli.in_hours);
    for course in courses
        .upcoming(DefaultTimeProvider, TimeDelta::hours(cli.in_hours as i64))
        .filter_by_name(&chosen)
    {
        println!(
            "{}: {}",
            course.nearest_period(DefaultTimeProvider).unwrap(),
            course
        );
    }
}
