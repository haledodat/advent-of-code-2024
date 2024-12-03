use std::path::Path;

use anyhow::Context;

fn lines(path: &Path) -> anyhow::Result<Vec<Vec<i32>>> {
    let input = std::fs::read_to_string(path)?;

    let lines = input
        .split('\n')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.split(' ')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(lines)
}

pub fn solve_part1(path: &Path) -> anyhow::Result<()> {
    let lines = lines(path)?;

    let mut safe_count = 0;
    'outer: for line in &lines {
        let mut increasing = None;
        for line in line.windows(2) {
            let x = line[0];
            let y = line[1];

            if !(1..=3).contains(&(x - y).abs()) {
                continue 'outer;
            }

            let mode = increasing.get_or_insert(x < y);

            match (y.cmp(&x), mode) {
                (std::cmp::Ordering::Less, true) => continue 'outer,
                (std::cmp::Ordering::Greater, false) => continue 'outer,
                (_, _) => {}
            }
        }
        safe_count += 1;
    }

    println!("Day 2, part 1 solution: {safe_count}");

    Ok(())
}

fn is_safe(mut line: impl Iterator<Item = i32>) -> anyhow::Result<bool> {
    let mut previous = line.next().context("empty line")?;
    let mut increasing: Option<bool> = None;
    for x in line {
        let diff = previous - x;
        if !(1..=3).contains(&diff.abs()) {
            return Ok(false);
        }
        if diff > 0 && increasing.is_some_and(|a| !a) {
            return Ok(false);
        }
        if diff < 0 && increasing.is_some_and(|a| a) {
            return Ok(false);
        }
        increasing = Some(diff > 0);
        previous = x;
    }
    Ok(true)
}

pub fn solve_part2(path: &Path) -> anyhow::Result<()> {
    let lines = lines(path)?;

    let mut safe_count = 0;
    'outer: for line in lines {
        if is_safe(line.iter().copied())? {
            safe_count += 1;
            continue;
        }

        for i in 0..line.len() {
            let iter = line
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(j, x)| (j != i).then_some(x));
            if is_safe(iter)? {
                safe_count += 1;
                continue 'outer;
            }
        }
    }

    println!("Day 2, part 2 solution: {}", safe_count);

    Ok(())
}
