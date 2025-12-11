use std::collections::HashMap;

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
struct Device {
    id: String,
    next: Vec<String>,
    value: i64,
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

fn clear_values(hm: &mut HashMap<&str, Device>) {
    let mut keys: Vec<&str> = vec![];
    for k in hm.keys() {
        keys.push(k);
    }
    for k in keys {
        hm.get_mut(k).unwrap().value = -1;
    }
}

fn ways_to_id2(from: &str, to: &str, hm: &mut HashMap<&str, Device>) -> i64 {
    if from == to {
        return 1;
    }
    let d = hm.get(from).expect(&format!("Unexpected id found: {from}"));
    if d.value != -1 {
        return d.value;
    }
    // // special case
    // if d.id == "out" {
    //     return 0;
    // }
    let mut sum = 0;
    let vec = d.next.clone();
    for n in vec {
        sum += ways_to_id2(&n, to, hm);
    }

    let d = hm
        .get_mut(from)
        .expect(&format!("Unexpected id found: {from}"));
    d.value = sum;
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

fn solve2(filepath: &str) -> i64 {
    // read lines into vector of nodes
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let mut hm: HashMap<&str, Device> = HashMap::new();
    for l in f.lines() {
        let id = l.split(':').next().unwrap();
        let mut d = Device {
            id: String::from(id),
            next: vec![],
            value: -1,
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
    // Special device that doesn't lead anywhere
    let d = Device {
        id: String::from("out"),
        next: vec![],
        value: -1,
    };
    hm.insert("out", d);

    // Order is svr -> fft -> dac -> out
    let svr_fft = ways_to_id2("svr", "fft", &mut hm);
    println!("{svr_fft}");
    clear_values(&mut hm);
    let fft_dac = ways_to_id2("fft", "dac", &mut hm);
    println!("{fft_dac}");
    clear_values(&mut hm);
    let dac_out = ways_to_id2("dac", "out", &mut hm);
    println!("{dac_out}");
    svr_fft * fft_dac * dac_out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("example"), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2("example2"), 2);
    }
}
