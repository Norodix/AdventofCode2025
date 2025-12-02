fn main() {
    let solution = solve("input");
    println!("{solution}");
    let solution2 = solve2("input");
    println!("{solution2}");
}

fn num_digits(number: u64) -> u64 {
    let mut n = number;
    let mut cnt = 0;
    while n > 0 {
        n /= 10;
        cnt += 1;
    }
    cnt
}

fn check_repeat(number: u64) -> bool {
    let n_size = num_digits(number);
    if n_size % 2 != 0 {
        return false;
    }
    let div = 10u64.pow((n_size / 2) as u32);
    number / div == number % div
}

fn solve(filepath: &str) -> u64 {
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let mut sum = 0;
    let ranges = f.split(",");
    for range in ranges {
        let mut r = range.split("-");
        let lower: u64 = r.next().unwrap().trim().parse().unwrap();
        let higher: u64 = r.next().unwrap().trim().parse().unwrap();
        for id in lower..=higher {
            if check_repeat(id) {
                sum += id;
            }
        }
    }
    sum
}

fn solve2(filepath: &str) -> u64 {
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let eg = solve("example");
        assert_eq!(eg, 1227775554);
        println!("Example solution is {eg}");
    }

    #[test]
    fn test_repeat_check() {
        assert_eq!(check_repeat(11), true);
        assert_eq!(check_repeat(101), false);
        assert_eq!(check_repeat(111), false);
        assert_eq!(check_repeat(6464), true);
        assert_eq!(check_repeat(123123), true);
    }

    // #[test]
    // fn example2() {
    //     let eg = solve2("example");
    // }
}
