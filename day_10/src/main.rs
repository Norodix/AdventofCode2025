use good_lp::{
    Constraint, Expression, Solution, SolverModel, constraint, default_solver, variable, variables,
};
use regex::Regex;
use std::{collections::HashMap, hash::Hash, io::Write};

// This is a graph search problem
// each state of the indicator lights is a node and each button press connects nodes
fn main() {
    let now = std::time::Instant::now();
    println!("{}", solve("input"));
    let elapsed = now.elapsed();
    println!("Part 1 time: {}us", elapsed.as_micros());
    let now = std::time::Instant::now();
    println!("{}", solve2("input"));
    let elapsed = now.elapsed();
    println!("Total time to compute: {}us", elapsed.as_micros());
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

// Too many steps for BFS
// Ideas:
//     - greedy backtracking
//     - maybe some vector matrix multiplication
//       - it is actually a linear equation but the bases are not guaranteed independent
//     - reduce the problem recursively -> memo friendly
//       [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
//       How many ways can I add up 0 to 3 -> fix it for the run, what is left?
//       go from smalles to largest
//     - is there a way to simplify the problem space? combine toggles or sth?

#[derive(Debug)]
struct JoltMachine {
    jolts: Vec<u32>,
    toggles: Vec<Vec<u32>>,
}

impl JoltMachine {
    // returns true if valid
    fn dec(&mut self, toggle_index: usize) -> bool {
        for ti in &self.toggles[toggle_index] {
            let ti = *ti as usize;
            if self.jolts[ti] == 0 {
                return false;
            }
        }
        for ti in &self.toggles[toggle_index] {
            let ti = *ti as usize;
            self.jolts[ti] -= 1;
        }
        true
    }

    fn inc(&mut self, toggle_index: usize) -> bool {
        for ti in &self.toggles[toggle_index] {
            let ti = *ti as usize;
            self.jolts[ti] += 1;
        }
        true
    }

    fn is_zero(&self) -> bool {
        for j in &self.jolts {
            if *j > 0 {
                return false;
            }
        }
        true
    }
}

// What is the minimum number of steps to reduce this jolt machine
// TODO add memo
fn reduce_machine(jm: &mut JoltMachine, hm: &mut HashMap<Vec<u32>, u64>) -> u64 {
    let cached = hm.get(&jm.jolts);
    match cached {
        Some(val) => return *val,
        None => (),
    }
    let mut min = std::u64::MAX - 1;
    if jm.is_zero() {
        return 0;
    }
    for t in 0..jm.toggles.len() {
        let good_step = jm.dec(t);
        if !good_step {
            // This would cause something to go below 0 -> invalid
            continue;
        }
        let new_min = reduce_machine(jm, hm);
        if new_min < min {
            min = new_min;
        }
        jm.inc(t);
    }
    hm.insert(jm.jolts.clone(), min + 1);
    min + 1
}

fn solve_machine(m: JoltMachine) -> u64 {
    variables! {vars: 0 <= x[m.toggles.len()] (integer); }

    let mut cnstr: Vec<Constraint> = vec![];
    let mut total: Expression = 0.into();
    for xi in &x {
        total += xi;
    }
    let mut solution = vars.minimise(total).using(default_solver);
    for j in 0..m.jolts.len() {
        let e: &mut Expression = &mut Expression::with_capacity(100);
        for t in 0..m.toggles.len() {
            let coeff = if m.toggles[t].contains(&(j as u32)) {
                1
            } else {
                0
            };
            e.add_mul(coeff, x[t]);
        }
        let e: Expression = e.clone();
        let e = e.eq(m.jolts[j]);
        println!("{:?}", &e);
        // cnstr.push(e);
        solution = solution.with(e);
    }
    let s = solution.solve().unwrap();
    let mut sum = 0;
    for xi in &x {
        sum += s.value(*xi) as u64;
    }
    sum
}

fn solve2(filepath: &str) -> u64 {
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let re_toggles = Regex::new(r"\(([^)]*)\)").unwrap();
    let re_jolts = Regex::new(r"\{([^}]*)\}").unwrap();
    let mut machines: Vec<JoltMachine> = vec![];
    for l in f.lines() {
        let mut m = JoltMachine {
            jolts: vec![],
            toggles: vec![],
        };

        let captures = re_toggles.captures_iter(l);
        for capture in captures {
            let numstrs: Vec<&str> = capture.get(1).unwrap().as_str().split(',').collect();
            let nums: Vec<u32> = numstrs.into_iter().map(|x| x.parse().unwrap()).collect();
            m.toggles.push(nums);
        }

        let jolts: Vec<u32> = re_jolts.captures(l).unwrap()[1]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        m.jolts = jolts;
        machines.push(m);
    }

    // println!("Machines: ");
    // Too low ->15375
    let mut sum = 0;
    for m in machines {
        // println!("Lights: {}\tToggles: {}", m.jolts.len(), m.toggles.len());
        sum += solve_machine(m);
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

    #[test]
    fn zerotest() {
        let mut jm = JoltMachine {
            jolts: vec![0, 0, 0],
            toggles: vec![vec![0]],
        };
        assert_eq!(jm.dec(0), false);
        assert_eq!(jm.is_zero(), true);
        jm.inc(0);
        assert_eq!(jm.is_zero(), false);
        assert_eq!(jm.dec(0), true);
        assert_eq!(jm.is_zero(), true);
    }

    #[test]
    fn basictest() {
        let mut hm: HashMap<Vec<u32>, u64> = HashMap::new();
        let mut jm = JoltMachine {
            jolts: vec![3, 3, 0],
            toggles: vec![vec![0, 1]],
        };
        assert_eq!(reduce_machine(&mut jm, &mut hm), 3);
    }

    #[test]
    fn mediumtest() {
        let mut hm: HashMap<Vec<u32>, u64> = HashMap::new();
        let mut jm = JoltMachine {
            jolts: vec![2, 4, 0],
            toggles: vec![vec![1], vec![0, 1]],
        };
        assert_eq!(reduce_machine(&mut jm, &mut hm), 4);
    }

    #[test]
    fn hardtest() {
        // Same as example 1
        let mut hm: HashMap<Vec<u32>, u64> = HashMap::new();
        let mut jm = JoltMachine {
            jolts: vec![3, 5, 4, 7],
            toggles: vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ],
        };
        assert_eq!(reduce_machine(&mut jm, &mut hm), 10);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2("example"), 33);
    }
}
