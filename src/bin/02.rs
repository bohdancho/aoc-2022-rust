use core::panic;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let ans: i32 = input
        .lines()
        .map(|round| {
            let chars: Vec<_> = round.chars().collect();
            let (opp, me) = (chars[0], chars[2]);

            get_shape_value(me) + get_clash_score(opp, me)
        })
        .sum();

    Some(ans as u32)
}

fn get_shape_value(shape: char) -> i32 {
    (shape as i32 - 'X' as i32) + 1
}

fn get_draw_response(opp: char) -> char {
    char::from_u32((opp as i32 - 'A' as i32 + 'X' as i32).try_into().unwrap()).unwrap()
}

fn get_winning_response(opp: char) -> char {
    match opp {
        'A' => 'Y',
        'B' => 'Z',
        'C' => 'X',
        _ => panic!("bad opp shape {}", opp),
    }
}

fn get_losing_response(opp: char) -> char {
    match opp {
        'A' => 'Z',
        'B' => 'X',
        'C' => 'Y',
        _ => panic!("bad opp shape {}", opp),
    }
}

fn get_clash_score(opp: char, me: char) -> i32 {
    let drawing = match opp {
        'A' => 'X',
        'B' => 'Y',
        'C' => 'Z',
        _ => panic!("bad opp shape {}", opp),
    };
    let winning = get_winning_response(opp);

    if me == drawing {
        return 3;
    }
    if me == winning {
        return 6;
    }
    0
}

pub fn part_two(input: &str) -> Option<u32> {
    let ans: i32 = input
        .lines()
        .map(|round| {
            let chars: Vec<_> = round.chars().collect();
            let (opp, outcome) = (chars[0], chars[2]);

            let me = match outcome {
                'X' => get_losing_response(opp),
                'Y' => get_draw_response(opp),
                'Z' => get_winning_response(opp),
                _ => panic!(),
            };

            get_shape_value(me) + get_clash_score(opp, me)
        })
        .sum();

    Some(ans as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
