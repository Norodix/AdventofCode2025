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
    check_repeat_n(number, 2)
}

fn check_repeat_all(number: u64) -> bool {
    let n_size = num_digits(number);
    for i in 2..=n_size {
        if check_repeat_n(number, i) {
            return true;
        }
    }
    false
}

// Check if there is a repeating pattern n times
fn check_repeat_n(number: u64, n: u64) -> bool {
    let n_size = num_digits(number);
    if n_size % n != 0 {
        return false;
    }
    let pattern_len = n_size / n;
    let div = 10u64.pow(pattern_len as u32);
    for index in 1..n {
        let digs = (number / div.pow(index as u32)) % div;
        if digs != number % div {
            return false;
        }
    }
    return true;
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
    let mut sum = 0;
    let ranges = f.split(",");
    for range in ranges {
        let mut r = range.split("-");
        let lower: u64 = r.next().unwrap().trim().parse().unwrap();
        let higher: u64 = r.next().unwrap().trim().parse().unwrap();
        for id in lower..=higher {
            if check_repeat_all(id) {
                sum += id;
            }
        }
    }
    sum
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

    #[test]
    fn test_repeat_n_check() {
        assert_eq!(check_repeat_n(111, 3), true);
        assert_eq!(check_repeat_n(111, 2), false);
        assert_eq!(check_repeat_n(101, 3), false);
        assert_eq!(check_repeat_n(6464, 2), true);
        assert_eq!(check_repeat_n(123123123, 2), false);
    }

    #[test]
    fn test_repeat_all_check() {
        assert_eq!(check_repeat_all(111), true);
        assert_eq!(check_repeat_all(11), true);
        assert_eq!(check_repeat_all(121212), true);
        assert_eq!(check_repeat_all(123123123), true);
        assert_eq!(check_repeat_all(9090), true);
        assert_eq!(check_repeat_all(101), false);
        assert_eq!(check_repeat_all(10), false);
        assert_eq!(check_repeat_all(121112), false);
        assert_eq!(check_repeat_all(123113123), false);
    }

    #[test]
    fn example2() {
        let eg = solve2("example");
        assert_eq!(eg, 4174379265);
    }
}
