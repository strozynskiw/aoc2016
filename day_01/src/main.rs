use std::fs;
use std::collections::HashSet;

// Warning
// due to lack of experience with RUST this solution is a little bit lame

fn part_1(input: &str) -> i32 {
    let orders :Vec<&str> = input.split(", ").collect();
    let mut position = (0 as i32, 0 as i32);

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut direction_index :i32 = 0;

    for order in orders {
        direction_index += match order.chars().nth(0).unwrap() {
            'R' => 1,
            'L' => -1,
            _ => panic!("Incorrect rotation")
        };

        //TODO try to rotate directions
        if direction_index < 0 {
            direction_index = 3;
        }
        else if direction_index > 3
        {
            direction_index = 0;
        }

        let distance_str = order.get(1..).unwrap();
        let distance = distance_str.parse::<i32>().unwrap();

        position = (position.0 + distance*(directions[direction_index as usize].0) , position.1 + distance*(directions[direction_index as usize].1));
    }

    position.0.abs() + position.1.abs()
}

fn part_2(input: &str) -> i32 {
    let orders :Vec<&str> = input.split(", ").collect();
    let mut position = (0 as i32, 0 as i32);

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut direction_index :i32 = 0;

    let mut visited_positions :HashSet<(i32, i32)> = HashSet::new();

    for order in orders {
        direction_index += match order.chars().nth(0).unwrap() {
            'R' => 1,
            'L' => -1,
            _ => panic!("Incorrect rotation")
        };

        //TODO try to rotate directions
        if direction_index < 0 {
            direction_index = 3;
        }
        else if direction_index > 3
        {
            direction_index = 0;
        }

        let distance_str = order.get(1..).unwrap();
        let distance = distance_str.parse::<i32>().unwrap();

        let old_position = position;
        position = (position.0 + distance*(directions[direction_index as usize].0) , position.1 + distance*(directions[direction_index as usize].1));

        let mut i = old_position.0;
        let mut j = old_position.1;

        loop
        {
            loop
            {
                if visited_positions.contains(&(i, j)) {
                    return i.abs() + j.abs();
                }
                visited_positions.insert((i, j));
                j += (position.1 - old_position.1).signum();
                if j == position.1 {break;}
            }
            i += (position.0 - old_position.0).signum();
            if i == position.0 {break;}
        }
    }
    0
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    print!("Result1: {}\n", part_1(&content));
    print!("Result2: {}\n", part_2(&content));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(5, part_1("R2, L3"));
        assert_eq!(2, part_1("R2, R2, R2"));
        assert_eq!(12, part_1("R5, L5, R5, R3"));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(4, part_2("R8, R4, R4, R8"));
    }
}
