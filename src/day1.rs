use std::{
    collections::{hash_map::Entry, HashMap},
    path::Path,
};

pub fn solve_part1(path: &Path) -> anyhow::Result<()> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let input = std::fs::read_to_string(path)?;

    for (x, y) in input
        .split('\n')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.split_once(' '))
        .flatten()
    {
        list1.push(x.trim().parse::<i32>()?);
        list2.push(y.trim().parse::<i32>()?);
    }

    list1.sort_unstable();
    list2.sort_unstable();

    let mut sum = 0;
    for (x, y) in list1.iter().zip(list2.iter()) {
        sum += (x - y).abs();
    }

    println!("Day 1, part 1 solution: {sum}");

    Ok(())
}

pub fn solve_part2(path: &Path) -> anyhow::Result<()> {
    let mut list1 = Vec::new();
    let mut list2 = HashMap::<i32, u32>::new();

    let input = std::fs::read_to_string(path)?;

    for (x, y) in input
        .split('\n')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.split_once(' '))
        .flatten()
    {
        list1.push(x.trim().parse::<i32>()?);

        match list2.entry(y.trim().parse::<i32>()?) {
            Entry::Occupied(mut occupied_entry) => {
                *occupied_entry.get_mut() += 1;
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
    }

    let mut sum = 0;
    for x in &list1 {
        let multiplier = list2.get(x).unwrap_or(&0);
        sum += x * (*multiplier as i32);
    }

    println!("Day 1, part 2 solution: {sum}");

    Ok(())
}
