advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|supply| supply.parse::<u32>().unwrap())
                .sum()
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
