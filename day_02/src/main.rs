use std::fs;
use std::time::Instant;

enum Operation {
    Up,
    Down,
    Left,
    Right,
}
type Generated = Vec<Vec<Operation>>;

const PINPAD: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
const PINPAD2: [[char; 5]; 5] = [
    ['.', '.', '1', '.', '.'],
    ['.', '2', '3', '4', '.'],
    ['5', '6', '7', '8', '9'],
    ['.', 'A', 'B', 'C', '.'],
    ['.', '.', 'D', '.', '.'],
];

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'L' => Operation::Left,
                    'R' => Operation::Right,
                    'U' => Operation::Up,
                    'D' => Operation::Down,
                    _ => panic!("Incorrect char"),
                })
                .collect()
        })
        .collect()
}

fn part_1(input: &Generated) -> String {
    let mut position = (1, 1);

    let positions = input
        .iter()
        .map(|operations| {
            operations.iter().for_each(|o| match *o {
                Operation::Up if position.0 > 0 => position.0 -= 1,
                Operation::Down if position.0 < 2 => position.0 += 1,
                Operation::Left if position.1 > 0 => position.1 -= 1,
                Operation::Right if position.1 < 2 => position.1 += 1,
                _ => (),
            });
            position
        })
        .collect::<Vec<(i32, i32)>>();

    let mut result = "".to_owned();

    positions.iter().for_each(|v| {
        result.push(PINPAD[v.0 as usize][v.1 as usize]);
    });

    result
}

fn part_2(input: &Generated) -> String {
    let mut position = (2, 0);

    let positions = input
        .iter()
        .map(|operations| {
            operations.iter().for_each(|o| match *o {
                Operation::Up
                    if position.0 > 0
                        && PINPAD2[position.0 - 1_usize][position.1 as usize] != '.' =>
                {
                    position.0 -= 1
                }
                Operation::Down
                    if position.0 < 4
                        && PINPAD2[position.0 + 1_usize][position.1 as usize] != '.' =>
                {
                    position.0 += 1
                }
                Operation::Left
                    if position.1 > 0
                        && PINPAD2[position.0 as usize][position.1 - 1_usize] != '.' =>
                {
                    position.1 -= 1
                }
                Operation::Right
                    if position.1 < 4
                        && PINPAD2[position.0 as usize][position.1 + 1_usize] != '.' =>
                {
                    position.1 += 1
                }
                _ => (),
            });
            position
        })
        .collect::<Vec<(usize, usize)>>();

    let mut result = "".to_owned();

    positions.iter().for_each(|v| {
        result.push(PINPAD2[v.0 as usize][v.1 as usize]);
    });

    result
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");

    let data = generate(&content);

    let res1_start = Instant::now();
    let res1 = part_1(&data);
    let res1_stop = Instant::now();

    let res2_start = Instant::now();
    let res2 = part_2(&data);
    let res2_stop = Instant::now();

    print!(
        "Result1: {}\nResolved in: {:?}\n",
        res1,
        res1_stop.duration_since(res1_start)
    );
    print!(
        "Result2: {}\nResolved in: {:?}\n",
        res2,
        res2_stop.duration_since(res2_start)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!("1985", part_1(&generate("ULL\nRRDDD\nLURDL\nUUUD")));
        assert_eq!("6982", part_1(&generate("RR\nD\nDDDDDDDDDDDDDL\nUUU")));
        assert_eq!(
            "7314",
            part_1(&generate(
                "UUUUUUUUUUUDDDDDDDDDDDDDDDDDDDDDDDDDDL\nDDDRRRRRRRRRRRRUUUUUUUU\nLLLLLL\nUUUD"
            ))
        );
        assert_eq!("4792", part_1(&generate("LL\nDDD\nRRR\nUUUUUUL")));
    }
    #[test]
    fn test_part_2() {
        assert_eq!("5DB3", part_2(&generate("ULL\nRRDDD\nLURDL\nUUUD")));
    }
}
