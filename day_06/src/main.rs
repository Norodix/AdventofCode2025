fn main() {
    println!("{}", solve("input"));
    println!("{}", solve2("input"));
}

fn solve(filepath: &str) -> u64 {
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let argc = f.lines().count(); // including the operator
    let elemc = f.split_whitespace().count();
    let cols = elemc / argc;
    let rows_num = argc - 1;
    // let mut nums_iter = f.split_whitespace();
    // let mut nums: Vec<u64> = vec![];
    // for _ in 0..(rows_num * cols) {
    //     let n: u64 = nums_iter.next().unwrap().parse().unwrap();
    //     nums.push(n);
    // }
    let nums: Vec<u64> = f
        .split_whitespace()
        .take(rows_num * cols)
        .map(|s| s.parse().unwrap())
        .collect();
    let operators: Vec<char> = f
        .lines()
        .last()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let mut sum = 0;
    for i in 0..cols {
        sum += nums
            .iter()
            .copied()
            .skip(i)
            .step_by(cols)
            .reduce(|acc, e| {
                if operators[i] == '+' {
                    acc + e
                } else {
                    acc * e
                }
            })
            .unwrap();
    }

    sum
}

fn solve2(filepath: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("example"), 4277556);
    }
}
