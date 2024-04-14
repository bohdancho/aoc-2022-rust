advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let ans = input
        .lines()
        .filter(|line| {
            let bounds = &line
                .split([',', '-'])
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>()[..];
            let [a_lo, a_hi, b_lo, b_hi] = bounds else {
                panic!("bad bounds")
            };

            (a_lo <= b_lo && a_hi >= b_hi) || (a_lo >= b_lo && a_hi <= b_hi)
        })
        .count();

    Some(ans as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let ans = input
        .lines()
        .filter(|line| {
            let bounds = &line
                .split([',', '-'])
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>()[..];
            let [a_lo, a_hi, b_lo, b_hi] = bounds else {
                panic!("bad bounds")
            };

            (a_lo <= b_hi && a_hi >= b_lo) || (b_lo <= a_hi && b_hi >= a_lo)
        })
        .count();

    Some(ans as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
