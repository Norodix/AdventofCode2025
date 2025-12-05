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

fn merge_ranges(a: &NumRange, b: &NumRange) -> NumRange {
    NumRange {
        min: std::cmp::min(a.min, b.min),
        max: std::cmp::max(a.max, b.max),
    }
}

fn is_overlapping(a: &NumRange, b: &NumRange) -> bool {
    if a.min <= b.max && a.min >= b.min
        || a.max <= b.max && a.max >= b.min
        || b.min <= a.max && b.min >= a.min
        || b.max <= a.max && b.max >= a.min
    {
        return true;
    }
    false
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

fn print_ranges(v: &Vec<NumRange>) {
    for v in v {
        println!("{}-{}", v.min, v.max);
    }
}

fn solve2(filepath: &str) -> u64 {
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
    // println!("Before");
    // print_ranges(&numranges);
    // Check for overlapping regions, replace them with merged
    let mut i = 0;
    let mut j = 0;
    'outer: while i < numranges.len() - 1 {
        j = i + 1;
        while j < numranges.len() {
            if is_overlapping(&numranges[i], &numranges[j]) {
                // println!(
                //     "{}-{} x {}-{}",
                //     numranges[i].min, numranges[i].max, numranges[j].min, numranges[j].max
                // );
                numranges.push(merge_ranges(&numranges[i], &numranges[j]));
                numranges.remove(j);
                numranges.remove(i);
                i = 0;
                j = 0;
                continue 'outer;
            }
            j += 1;
        }
        i += 1;
    }

    let mut sum: u64 = 0;
    for r in numranges {
        sum += r.max - r.min + 1;
    }
    sum
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
        assert_eq!(solve2("example"), 14);
    }
}
