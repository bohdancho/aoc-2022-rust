advent_of_code::solution!(7);

pub fn part_one(_input: &str) -> Option<u32> {
    // let tree = parse_tree(input);

    let ans = 0;
    Some(ans as u32)
}

/* $ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e */

// #[derive(Default)]
// struct Dir {
//     dirs: HashMap<String, Rc<Dir>>,
//     files: Vec<u128>,
//     size: u128,
// }

// fn parse_tree(input: &str) -> Dir {
//     let tree = Dir::default();
//     let mut cursor: Rc<Dir> = tree.into();
//     input.strip_prefix("$ ").unwrap().split("$ ").map(|block| {
//         let mut lines = block.lines();
//         let cmd = lines.next().unwrap();
//         if let Some(dir) = cmd.strip_prefix("cd ") {
//             cursor.dirs.insert(dir.to_string(), Dir::default().into());
//             let hm = cursor.dirs.get(dir).unwrap();
//         }
//     });
//     tree
// }

pub fn part_two(_input: &str) -> Option<u32> {
    let ans: i32 = 0;

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
