use std::path::Path;

mod day1;

fn main() -> anyhow::Result<()> {
    day1::solve_part1(Path::new("inputs/1/input.txt"))?;
    day1::solve_part2(Path::new("inputs/1/input.txt"))?;
    Ok(())
}
