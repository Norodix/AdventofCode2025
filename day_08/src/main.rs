#[derive(Debug)]
struct Node {
    pos: [i32; 3],
    group_id: i32,
}

impl Node {
    fn distance2(&self, other: &Self) -> i64 {
        let x = (self.pos[0] - other.pos[0]) as i64;
        let y = (self.pos[1] - other.pos[1]) as i64;
        let z = (self.pos[2] - other.pos[2]) as i64;
        x * x + y * y + z * z
    }
}

// Pairs is a connection matrix
// a, b and exclude are indeces of this matrix
fn is_connected(a: usize, b: usize, pairs: &Vec<bool>, exclude: usize) -> bool {
    let n = pairs.len().isqrt();
    assert_eq!(n * n, pairs.len());
    // Check if this is paired with other
    if pairs[a * n + b] {
        return true;
    }

    for i in 0..n {
        if i == a {
            // pointless to check self again
            continue;
        }
        if i == exclude {
            // never go backwards
            continue;
        }
        if pairs[a * n + i] {
            // is a neighbor already
            if is_connected(i, b, pairs, a) {
                return true;
            }
        }
    }

    false
}

#[derive(Debug)]
struct Pairing {
    a: usize,
    b: usize, // index of Node
    d2: i64,  // distance squared
}

fn main() {
    println!("{}", solve("input", 1000));
    // println!("{}", solve2("input"));
}

fn solve(filepath: &str, iterations: usize) -> u64 {
    // read lines into vector of nodes
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let mut nodes: Vec<Node> = vec![];
    for l in f.lines() {
        let mut p = l.split(',');
        let n: Node = Node {
            pos: [
                p.next().unwrap().parse().unwrap(),
                p.next().unwrap().parse().unwrap(),
                p.next().unwrap().parse().unwrap(),
            ],
            group_id: -1,
        };
        nodes.push(n);
    }
    let len = nodes.len();

    // create n*n vector of each pairing
    // precompute distances
    let mut pairings: Vec<Pairing> = vec![];
    for i in 0..len {
        for j in (i + 1)..len {
            let p = Pairing {
                a: i,
                b: j,
                d2: nodes[i].distance2(&nodes[j]),
            };
            pairings.push(p);
        }
    }

    // TODO duplicate info, could be improved
    // create n*n connection vector
    let mut connections: Vec<bool> = vec![];
    for _i in 0..len {
        for _j in 0..len {
            connections.push(false);
        }
    }

    // sort pairings by distance
    pairings.sort_by(|a, b| a.d2.cmp(&b.d2));
    // for p in &pairings {
    //     println!("{p:?}");
    // }

    // Add first <iterations> connections while skipping already connected nodes (avoid loops)
    // This is stupid, but actually i should be used... bad problem statement
    let mut connections_made = 0;
    let mut i = 0;

    while i < iterations {
        let a = pairings[i].a;
        let b = pairings[i].b;
        // if a not connected to b in other ways
        if is_connected(a, b, &connections, a) {
            println!("Skipped {a} {b}");
        } else {
            connections[a * len + b] = true;
            connections[b * len + a] = true;
            connections_made += 1;
            println!("Connected {a} {b}");
        }
        i += 1;
    }

    // Every group gets the id of its first element's index
    for i in 0..len {
        // if this node is already part of a group, skip
        if nodes[i].group_id != -1 {
            continue;
        }
        nodes[i].group_id = i as i32;
        for j in 0..len {
            // would be faster to compute indirect connection table
            if is_connected(i, j, &connections, i) {
                // this is connected to node[i]
                nodes[j].group_id = i as i32;
            }
        }
    }
    // println!("{nodes:?}");

    let mut counts = vec![0; len];
    for i in 0..len {
        counts[nodes[i].group_id as usize] += 1;
    }
    // println!("{counts:?}");

    let mut acc: u64 = 1;
    counts.sort();
    counts.reverse();
    for i in 0..3 {
        println!("{i}->{}", counts[i]);
        acc *= counts[i] as u64;
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("example", 10), 40);
    }
}
