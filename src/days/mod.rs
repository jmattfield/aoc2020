mod util;
pub use util::grid;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;

use lazy_static;

lazy_static::lazy_static! {
    static ref BUILDERS: Vec<DayRunner> = vec![
        (day_01::part_one, day_01::part_two),
        (day_02::part_one, day_02::part_two),
        (day_03::part_one, day_03::part_two),
        (day_04::part_one, day_04::part_two),
        (day_05::part_one, day_05::part_two),
        (day_06::part_one, day_06::part_two),
        (day_07::part_one, day_07::part_two),
        (day_08::part_one, day_08::part_two),
        (day_09::part_one, day_09::part_two),
        (day_10::part_one, day_10::part_two),
        (day_11::part_one, day_11::part_two),
        (day_12::part_one, day_12::part_two),
        (day_13::part_one, day_13::part_two),
        (day_14::part_one, day_14::part_two),
        (day_15::part_one, day_15::part_two),
        (day_16::part_one, day_16::part_two),
        (day_17::part_one, day_17::part_two),
        (day_18::part_one, day_18::part_two),
        (day_19::part_one, day_19::part_two),
        (day_20::part_one, day_20::part_two),
    ];
}

type DayRunner = (fn(&[&str]), fn(&[&str]));

pub fn run(runner: DayRunner, data: &[&str]) {
    println!("\nPart One\n========");
    (runner.0)(&data);
    println!("\nPart Two\n========");
    (runner.1)(&data);
}

pub fn days_implemented() -> u8 {
    BUILDERS.len() as u8
}

pub fn get_runner(day: u8) -> DayRunner {
    BUILDERS[(day - 1) as usize]
}
