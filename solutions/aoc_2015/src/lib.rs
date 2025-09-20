pub mod day17;
pub mod day16;
pub mod day15;
pub mod day14;
pub mod day13;
pub mod day12;
pub mod day11;
pub mod day10;
pub mod day09;
pub mod day08;
pub mod day07;
pub mod day06;
pub mod day05;
pub mod day04;
pub mod day03;
pub mod day02;
pub mod day01;
 
pub fn run(day: u8) -> anyhow::Result<()> {
    match day {
        17 => day17::solve()?,
        16 => day16::solve()?,
        15 => day15::solve()?,
        14 => day14::solve()?,
        13 => day13::solve()?,
        12 => day12::solve()?,
        11 => day11::solve()?,
        10 => day10::solve()?,
        9 => day09::solve()?,
        8 => day08::solve()?,
        7 => day07::solve()?,
        6 => day06::solve()?,
        5 => day05::solve()?,
        4 => day04::solve()?,
        3 => day03::solve()?,
        2 => day02::solve()?,
        1 => day01::solve()?,
        _ => println!("Day {{day}} not yet implemented for ''2015''."),
    }
    Ok(())
}
