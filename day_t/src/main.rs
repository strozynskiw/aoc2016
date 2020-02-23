use std::fs;
use std::time::Instant;

type Generated = Vec<Vec<usize>>;

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect()
}

fn part_1(input: &Generated) -> usize {
0
}

fn part_2(input: &Generated) -> usize {
0
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");

    let res1_start = Instant::now();
    let res1 = part_1(&generate(&content));
    let res1_stop = Instant::now();

    let res2_start = Instant::now();
    let res2 = part_2(&generate2(&content));
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
        //assert_eq!("1985", part_1(&generate("ULL\nRRDDD\nLURDL\nUUUD")));
    }
    #[test]
    fn test_part_2() {
        //assert_eq!("5DB3", part_2(&generate("ULL\nRRDDD\nLURDL\nUUUD")));
    }
}
