use std::path::Path;

mod day1;
mod day2;
mod day3;

fn main() -> anyhow::Result<()> {
    // day1::solve_part1(Path::new("inputs/1/input.txt"))?;
    // day1::solve_part2(Path::new("inputs/1/input.txt"))?;

    // day2::solve_part1(Path::new("inputs/2/sample.txt"))?;
    // day2::solve_part1(Path::new("inputs/2/input.txt"))?;

    // day2::solve_part2(Path::new("inputs/2/sample.txt"))?;
    // day2::solve_part2(Path::new("inputs/2/input.txt"))?;

    // day3::solve_part1(Path::new("inputs/3/sample.txt"))?;
    // day3::solve_part1(Path::new("inputs/3/input.txt"))?;

    day3::solve_part2(Path::new("inputs/3/sample2.txt"))?;
    day3::solve_part2(Path::new("inputs/3/input.txt"))?;

    Ok(())
}
