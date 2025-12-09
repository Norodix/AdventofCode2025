#[derive(Debug)]
struct Pairing {
    a: usize,
    b: usize, // index of Node
    area: i64,
}

struct Node {
    pos: [i64; 2],
}

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
    let mut nodes: Vec<Node> = vec![];
    for l in f.lines() {
        let mut p = l.split(',');
        let n: Node = Node {
            pos: [
                p.next().unwrap().parse().unwrap(),
                p.next().unwrap().parse().unwrap(),
            ],
        };
        nodes.push(n);
    }
    let len = nodes.len();

    // create n*n vector of each pairing
    // precompute areas
    let mut pairings: Vec<Pairing> = vec![];
    let mut max_area = 0_i64;
    for i in 0..len {
        for j in (i + 1)..len {
            let area = ((nodes[i].pos[0] - nodes[j].pos[0]).abs() + 1)
                * ((nodes[i].pos[1] - nodes[j].pos[1]).abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("example"), 50);
    }
}
