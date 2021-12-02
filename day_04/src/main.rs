use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Instant;

#[derive(PartialEq, Debug)]
struct Operation {
    name: String,
    decrypted_name: String,
    sector: i32,
    checksum: String,
}

impl FromStr for Operation {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.*)-(\d+)\[(\w{5})\]").unwrap();
        }

        let cap = RE.captures(s).unwrap();

        let sector = cap[2].parse::<i32>()?;

        Ok(Operation {
            name: cap[1].replace("-", ""),
            decrypted_name: decrypt(&cap[1], sector as u32),
            sector,
            checksum: cap[3].to_owned(),
        })
    }
}

type Generated = Vec<Operation>;

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| Operation::from_str(l).unwrap())
        .collect()
}

fn find_checksum(input: &str) -> String {
    let mut collection: HashMap<char, i32> = HashMap::new();
    input.chars().for_each(|c| {
        let entry = collection.entry(c).or_insert(0);
        *entry += 1;
    });

    let mut values = collection.iter().collect::<Vec<(&char, &i32)>>();
    values.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());
    values.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    values.iter().map(|v| v.0).take(5).collect::<String>()
}

fn decrypt(s: &str, sector: u32) -> String {
    s.chars()
        .map(|c| match &c {
            '-' => ' ',
            _ => {
                let shift = c as u32 - b'a' as u32 + sector;
                ((shift % 26) as u8 + b'a') as char
            }
        })
        .collect::<String>()
}

fn part_1(input: &Generated) -> usize {
    input
        .iter()
        .filter(|v| v.checksum == find_checksum(&v.name))
        .fold(0, |acc, o| acc + o.sector as usize)
}

fn part_2(input: &Generated) -> i32 {
    input
        .iter()
        .find(|i| {
            println!("{}", i.decrypted_name);
            i.decrypted_name == "northpole object storage"
        })
        .expect("Not found")
        .sector
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");

    let res1_start = Instant::now();
    let res1 = part_1(&generate(&content));
    let res1_stop = Instant::now();

    let res2_start = Instant::now();
    let res2 = part_2(&generate(&content));
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
    fn test_generator() {
        assert_eq!(
            Operation {
                name: "aaaaabbbzyx".to_owned(),
                sector: 123,
                checksum: "abxyz".to_owned()
            },
            Operation::from_str("aaaaa-bbb-z-y-x-123[abxyz]").unwrap()
        );
    }

    #[test]
    fn test_checksum() {
        assert_eq!("abxyz", find_checksum("aaaaabbbzyx"));
        assert_eq!("abcde", find_checksum("gfedcbaabcdefg"));
    }

    #[test]
    fn test_part_1() {
        //assert_eq!("1985", part_1(&generate("ULL\nRRDDD\nLURDL\nUUUD")));
    }
    #[test]
    fn test_part_2() {
        //assert_eq!("5DB3", part_2(&generate("ULL\nRRDDD\nLURDL\nUUUD")));
    }
}
