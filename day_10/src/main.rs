use regex::Regex;
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
struct Machine {
    lights: u64,
    toggles: Vec<u64>,
}

fn get_shortest_path(m: &Machine) -> u64 {
    // I will use a hashmap to store the machine configuration map
    // the light state is going to be the key and the minimum steps to get here is the value
    let mut map: HashMap<u64, u64> = HashMap::new();
    let mut visitnodes: Vec<u64> = vec![0]; // Add starting position
    map.insert(0, 0);
    // Simple BFS search
    while visitnodes.len() > 0 {
        let node = visitnodes[0];
        for t in &m.toggles {
            let next = node ^ t;
            if next == m.lights {
                let thisval = map.get(&node).unwrap();
                return thisval + 1;
            }
            if map.contains_key(&next) {
                // Already included do nothing
            } else {
                let thisval = map.get(&node).unwrap();
                map.insert(next, *thisval + 1);
                visitnodes.push(next);
            }
        }
        visitnodes.remove(0);
    }
    0
}

fn solve(filepath: &str) -> u64 {
    // read lines into vector of nodes
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let re_lights = Regex::new(r"\[(.*)\]").unwrap();
    let re_toggles = Regex::new(r"\(([^)]*)\)").unwrap();
    let mut machines: Vec<Machine> = vec![];
    for l in f.lines() {
        let mut m = Machine {
            lights: 0,
            toggles: vec![],
        };
        // let lights = re_lights.find(l).map(|m| m.as_str()).unwrap();
        let lights = &re_lights.captures(l).unwrap()[1];
        let mut light_num = 0;
        for c in lights.chars().rev() {
            light_num = light_num << 1;
            if c == '#' {
                light_num |= 1;
            }
        }
        m.lights = light_num;
        let captures = re_toggles.captures_iter(l);
        for capture in captures {
            let nums = capture.get(1).unwrap().as_str();
            let mut toggles_num: u64 = 0;
            for n in nums.split(',') {
                let n: u64 = n.parse().unwrap();
                toggles_num |= 1_u64 << n;
            }
            m.toggles.push(toggles_num);
        }
        machines.push(m);
    }
    // println!("{machines:?}");
    // Machines are in usable format
    let mut sum = 0;
    for m in machines {
        // println!("{m:?}: {}", get_shortest_path(&m));
        sum += get_shortest_path(&m);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("example"), 7);
    }
}
