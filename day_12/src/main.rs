use regex::Regex;

fn main() {
    let now = std::time::Instant::now();
    println!("{}", solve("input"));
    let elapsed = now.elapsed();
    println!("Part 1 time: {}us", elapsed.as_micros());
    // let now = std::time::Instant::now();
    // println!("{}", solve2("input"));
    // let elapsed = now.elapsed();
    // println!("Total time to compute: {}us", elapsed.as_micros());
}

fn solve(filepath: &str) -> u64 {
    // read lines into vector of nodes
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let re_fit = Regex::new(r"(\d+)x(\d+): ([0-9 ]+)").unwrap();
    // Lets make it super dumb, check if it fits inside at all
    let mut correct = 0;
    for l in f.lines() {
        if re_fit.is_match(l) {
            let caps = re_fit.captures(l).unwrap();
            let x: u64 = caps[1].parse().unwrap();
            let y: u64 = caps[2].parse().unwrap();
            let grid = x * y;
            let max: u64 = caps[3]
                .split(' ')
                .map(|x| x.parse::<u64>().unwrap())
                .sum::<u64>()
                * 3
                * 3;
            if max <= grid {
                println!("{l} -> fits. Max {max}, vs size {grid}");
                correct += 1;
            } else {
                println!("{l} -> fails. Max {max}, vs size {grid}");
            }
        }
    }
    correct
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("example"), 2);
    }
}
