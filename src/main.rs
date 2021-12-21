mod grid;
#[macro_use]
mod utils;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() {
    let day = 12;
    match day {
        1 => day_1::one().unwrap(),
        2 => day_2::two().unwrap(),
        3 => day_3::three().unwrap(),
        4 => day_4::four().unwrap(),
        5 => day_5::five().unwrap(),
        6 => day_6::six().unwrap(),
        7 => day_7::seven().unwrap(),
        8 => day_8::eight().unwrap(),
        9 => day_9::nine().unwrap(),
        10 => day_10::ten().unwrap(),
        11 => day_11::eleven().unwrap(),
        12 => day_12::twelve().unwrap(),
        _ => unreachable!(),
    }
}
