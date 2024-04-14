use core::panic;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<String> {
    let (stacks_raw, commands) = input.split_once("\n\n").unwrap();
    let stacks_lines: Vec<_> = stacks_raw.lines().map(str::as_bytes).collect();

    let mut stacks = vec![];
    for col in (1..stacks_lines[0].len()).step_by(4) {
        let mut stack = vec![];
        for row in (0..stacks_lines.len() - 1).rev() {
            let item = stacks_lines[row][col];
            if item == b' ' {
                break;
            }
            stack.push(item as char)
        }
        stacks.push(stack);
    }

    for command in commands.lines() {
        let [amt, from, to] = command
            .split(' ')
            .flat_map(str::parse)
            .collect::<Vec<usize>>()[..]
        else {
            panic!("bad commands");
        };
        let (from_idx, to_idx) = (from - 1, to - 1);

        for _ in 0..amt {
            if let Some(moved) = stacks[from_idx].pop() {
                stacks[to_idx].push(moved)
            }
        }
    }

    let mut ans = String::new();
    for stack in stacks.iter() {
        ans.push_str(&stack.last().unwrap().to_string())
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let ans: i32 = 0;

    Some(ans as u32)
}

#[cfg(test)]
mod tests {
    // use super::*;
    //
    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(String::));
    // }
    //
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(""));
    // }
}
