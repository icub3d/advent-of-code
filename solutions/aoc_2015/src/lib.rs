pub mod day02;
pub mod day01;
 
pub fn run(day: u8) -> anyhow::Result<()> {
    match day {
        2 => day02::solve()?,
        1 => day01::solve()?,
        _ => println!("Day {{day}} not yet implemented for ''2015''."),
    }
    Ok(())
}
