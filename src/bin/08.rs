use std::collections::HashSet;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let y_len = matrix.len();
    let x_len = matrix[0].len();

    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..y_len {
        let iter = (0..x_len).map(|x| (x, y));
        mark_visible(&mut visible, &matrix, iter.clone());
        mark_visible(&mut visible, &matrix, iter.rev());
    }
    for x in 0..x_len {
        let iter = (0..y_len).map(|y| (x, y));
        mark_visible(&mut visible, &matrix, iter.clone());
        mark_visible(&mut visible, &matrix, iter.rev());
    }
    for y in (0..y_len).rev() {
        let iter = (0..x_len).map(|x| (x, y));
        mark_visible(&mut visible, &matrix, iter.clone());
        mark_visible(&mut visible, &matrix, iter.rev());
    }
    for x in (0..x_len).rev() {
        let iter = (0..y_len).map(|y| (x, y));
        mark_visible(&mut visible, &matrix, iter.clone());
        mark_visible(&mut visible, &matrix, iter.rev());
    }

    Some(visible.len() as u32)
}

fn mark_visible(
    visible: &mut HashSet<(usize, usize)>,
    matrix: &[Vec<u32>],
    iter: impl Iterator<Item = (usize, usize)>,
) {
    let mut highest = -1;
    for (x, y) in iter {
        let curr = matrix[y][x] as i32;
        if curr > highest {
            highest = curr;
            visible.insert((x, y));
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let y_len = matrix.len();
    let x_len = matrix[0].len();

    let mut best_score = 0;
    for y in 0..y_len {
        for x in 0..x_len {
            let key = matrix[y][x];
            let score = get_view_score(&matrix, (0..x).rev().map(|x| (x, y)), key)
                * get_view_score(&matrix, (0..y).rev().map(|y| (x, y)), key)
                * get_view_score(&matrix, (x + 1..x_len).map(|x| (x, y)), key)
                * get_view_score(&matrix, (y + 1..y_len).map(|y| (x, y)), key);

            best_score = best_score.max(score);
        }
    }

    Some(best_score)
}

fn get_view_score(
    matrix: &[Vec<u32>],
    iter: impl Iterator<Item = (usize, usize)>,
    key: u32,
) -> u32 {
    let mut score = 0;
    for (x, y) in iter {
        score += 1;
        if key <= matrix[y][x] {
            break;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
