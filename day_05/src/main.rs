use std::time::Instant;

fn part_1(input: &str) -> String {
    let mut i = 0;
    let mut password: Vec<char> = Vec::new();

    loop {
        let test_str = format!("{}{}", input, i);
        let md5 = format!("{:x}", md5::compute(&test_str));

        if md5.chars().take(5).collect::<String>() == "00000" {
            password.push(md5.chars().skip(5).next().unwrap());
            if password.len() == 8 {
                break;
            }
        }
        i += 1;
    }

    password.iter().collect::<String>()
}

fn part_2(input: &str) -> String {
    let mut i = 0;
    let mut password: [char; 8] = [0 as char; 8];

    loop {
        let test_str = format!("{}{}", input, i);
        let md5 = format!("{:x}", md5::compute(&test_str));

        if md5.chars().take(5).collect::<String>() == "00000" {
            let position = md5.chars().skip(5).next().unwrap();
            let character = md5.chars().skip(6).next().unwrap();

            let pos = match position {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'a' => 10,
                'b' => 11,
                'c' => 12,
                'd' => 13,
                'e' => 14,
                'f' => 15,
                _ => panic!("Incorrect value"),
            };

            if (0..8).contains(&pos) && password[pos] == 0 as char {
                password[pos] = character;
            }

            if password.iter().filter(|&&c| c == 0 as char).count() == 0 {
                break;
            }
        }
        i += 1;
    }

    password.iter().collect::<String>()
}

fn main() {
    let res1_start = Instant::now();
    let res1 = part_1("ffykfhsq");
    let res1_stop = Instant::now();

    let res2_start = Instant::now();
    let res2 = part_2("ffykfhsq");
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
        assert_eq!("18f47a30", part_1("abc"));
    }
    #[test]
    fn test_part_2() {
        assert_eq!("05ace8e3", part_2("abc"));
    }
}
