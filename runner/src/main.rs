use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Year of the puzzle to run
    #[arg(short, long)]
    year: u16,

    /// Day of the puzzle to run
    #[arg(short, long)]
    day: u8,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("🎄 Advent of Code {} - Day {} 🎄", args.year, args.day);

    match args.year {
        2015 => aoc_2015::run(args.day)?,
        _ => println!("Year {} not yet implemented!", args.year),
    }

    Ok(())
}
