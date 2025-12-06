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
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let operators: Vec<char> = f
        .lines()
        .last()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let rows = f.lines().count(); // including the operator
    let f: Vec<char> = f.chars().collect();
    let mut nums: Vec<u64> = vec![];
    let cols = f.len() / rows;
    let mut op_index = 0;
    let mut sum = 0;
    for c in 0..cols {
        let mut s = String::from("");
        for r in 0..(rows - 1) {
            s.push(f[r * cols + c]);
        }
        let s = s.trim();
        if !s.is_empty() {
            nums.push(s.parse().unwrap());
        } else {
            // consume the numbers collected in nums so far
            sum += nums
                .iter()
                .copied()
                .reduce(|acc, e| {
                    if operators[op_index] == '+' {
                        acc + e
                    } else {
                        acc * e
                    }
                })
                .unwrap();
            nums.clear();
            // Increment operator index
            op_index += 1;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("example"), 4277556);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2("example"), 3263827);
    }
}
