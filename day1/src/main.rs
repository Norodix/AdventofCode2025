fn main() {
    let solution = solve("input");
    println!("The solution is {solution}");

    let part2_solution = part2_solve("input");
    println!("The solution for part 2 is {part2_solution}");
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

fn part2_solve(filepath: &str) -> i32 {
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
        // For negative turns to make the division work,
        // map the indicator to its negative equivalent
        if sign < 0 {
            indicator = indicator - 100;
            indicator %= 100; // takes care of the -100 case
        }
        indicator += num * sign;
        // Simple division calculation returns the number of clicks
        let click = (indicator / 100).abs();
        indicator = indicator.rem_euclid(100);
        cnt += click;
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let eg = solve("example");
        assert_eq!(eg, 3);
        println!("Example solution is {eg}");
    }

    #[test]
    fn part2_example() {
        let eg = part2_solve("example");
        assert_eq!(eg, 6);
    }
}
