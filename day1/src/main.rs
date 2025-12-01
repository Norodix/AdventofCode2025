fn main() {
    let solution = solve("input");
    println!("The solution is {solution}");
}

fn solve(filepath: &str) -> i32 {
    let mut cnt = 0;
    let mut indicator = 50;
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    for line in f.lines() {
        if line.len() < 2 {
            continue;
        }
        let sign: &str = &line[..1];
        let sign = match sign {
            "R" => 1,
            "L" => -1,
            _ => 0,
        };
        let num: i32 = line[1..].parse().unwrap();
        // println!("{line} -> {sign} * {num}");
        indicator += num * sign;
        indicator = indicator.rem_euclid(100);
        if indicator == 0 {
            cnt += 1;
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let eg = solve("example");
        assert_eq!(eg, 3);
        println!("Example solution is {eg}");
    }
}
