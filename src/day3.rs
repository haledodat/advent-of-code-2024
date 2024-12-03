use std::{
    collections::{hash_map::Entry, HashMap},
    path::Path,
};

use anyhow::Context;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Mode {
    Increase,
    Decrease,
}

pub fn solve_part1(path: &Path) -> anyhow::Result<()> {
    let input = std::fs::read_to_string(path)?;
    let regex = regex::Regex::new(r"mul\(\d{1,3},\d{1,3}\)")?;

    let mut sum = 0;
    for mat in regex.find_iter(&input) {
        let value = mat.as_str();
        let value = &value[4..value.len() - 1];
        let (x, y) = value.split_once(',').context("should be valid number pair")?;

        let x = x.parse::<u32>()?;
        let y = y.parse::<u32>()?;
        
        sum += x * y;
    }

    println!("Day 3, part 1 solution: {sum}");

    Ok(())
}

pub fn solve_part2(path: &Path) -> anyhow::Result<()> {
    let input = std::fs::read_to_string(path)?;
    let regex = regex::Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))")?;

    let mut enabled = true;
    
    let mut sum = 0;
    for mat in regex.find_iter(&input) {
        let value = mat.as_str();
        match value {
            "do()" => {enabled = true; continue;},
            "don't()" => {enabled = false; continue;},
            _ if !enabled => continue,
            _ => {}
        };

        let value = &value[4..value.len() - 1];
        let (x, y) = value.split_once(',').context("should be valid number pair")?;

        let x = x.parse::<u32>()?;
        let y = y.parse::<u32>()?;
        
        sum += x * y;
    }

    println!("Day 3, part 2 solution: {sum}");

    Ok(())
}
