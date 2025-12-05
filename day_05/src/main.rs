fn main() {
    println!("{}", solve("input"));
    println!("{}", solve2("input"));
}

struct NumRange {
    min: u64,
    max: u64,
}

fn is_fresh(range: &Vec<NumRange>, n: u64) -> bool {
    for r in range {
        if n <= r.max && n >= r.min {
            return true;
        }
    }
    return false;
}

fn solve(filepath: &str) -> u32 {
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let mut parts = f.split("\n\n");
    let mut numranges: Vec<NumRange> = vec![];
    let ranges = parts.next().unwrap().split('\n');
    for r in ranges {
        let mut minmax = r.split('-');
        let range = NumRange {
            min: minmax.next().unwrap().parse().unwrap(),
            max: minmax.next().unwrap().parse().unwrap(),
        };
        numranges.push(range);
    }
    let nums = parts.next().unwrap().lines();
    let mut cnt = 0;
    for n in nums {
        let n: u64 = n.trim().parse().unwrap();
        if is_fresh(&numranges, n) {
            cnt += 1;
        }
    }
    cnt
}

fn solve2(filepath: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("example"), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2("example"), 43);
    }
}
