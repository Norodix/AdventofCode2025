use std::collections::HashMap;

// This is a graph search problem
// each state of the indicator lights is a node and each button press connects nodes
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

#[derive(Debug)]
struct Device {
    id: String,
    next: Vec<String>,
    value: u64,
}

fn ways_to_id(from: &str, to: &str, hm: &HashMap<&str, Device>) -> u64 {
    if from == to {
        return 1;
    }
    let d = hm.get(from).expect(&format!("Unexpected id found: {from}"));
    let mut sum = 0;
    for n in 0..d.next.len() {
        let next = &d.next[n];
        sum += ways_to_id(&next, to, hm);
    }
    sum
}

fn solve(filepath: &str) -> u64 {
    // read lines into vector of nodes
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let mut hm: HashMap<&str, Device> = HashMap::new();
    for l in f.lines() {
        let id = l.split(':').next().unwrap();
        let mut d = Device {
            id: String::from(id),
            next: vec![],
            value: 0,
        };
        let mut connections = l.split(' ');
        connections.next();
        let mut op: Option<&str> = connections.next();
        while op.is_some() {
            let n = op.unwrap();
            // enable to help dot file graph generation
            // println!("{id} -> {n}");
            d.next.push(String::from(op.unwrap()));
            op = connections.next();
        }
        hm.insert(id, d);
    }
    ways_to_id("you", "out", &hm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("example"), 5);
    }
}
