use core::panic;
use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let ans: i32 = input
        .lines()
        .map(get_common_item_comp)
        .map(get_item_priority)
        .sum();

    Some(ans as u32)
}

fn get_common_item_comp(rucksack: &str) -> char {
    let mid = ((rucksack.len() as f32) / 2.0) as usize;
    let (comp_a, comp_b) = rucksack.split_at(mid);

    let set: HashSet<char> = comp_a.chars().collect();
    for char in comp_b.chars() {
        if set.get(&char).is_some() {
            return char;
        }
    }

    panic!("no common item");
}

fn get_common_item_group(group: Vec<&str>) -> char {
    let mut set: HashSet<char> = HashSet::from_iter(group[0].chars());
    for elf in group[1..].iter() {
        let mut common = HashSet::new();
        for char in elf.chars() {
            if set.get(&char).is_some() {
                common.insert(char);
            }
        }
        set = common;
    }

    *set.iter()
        .next()
        .unwrap_or_else(|| panic!("no common item found in group {:?}", group))
}

fn get_item_priority(item: char) -> i32 {
    let item = item as i32;

    if item < 'a' as i32 {
        return item - 'A' as i32 + 27;
    }

    item - 'a' as i32 + 1
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let group_inputs: Vec<&[&str]> = lines.chunks(3).collect();

    let ans: i32 = group_inputs
        .iter()
        .map(|&r| get_common_item_group(r.to_vec()))
        .map(get_item_priority)
        .sum();

    Some(ans as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(70));
    }
}
