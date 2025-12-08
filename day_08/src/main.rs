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

// Because of bad problem statement I created and relied on a recursive connection test
// With the actual problem, a complete connection matrix can be maintained which is faster
fn connect(connections: &mut Vec<bool>, a: usize, b: usize) {
    let n = connections.len().isqrt();
    assert_eq!(n * n, connections.len());
    // every connection of a to every connection of b
    let mut a_conn: Vec<usize> = vec![];
    let mut b_conn: Vec<usize> = vec![];
    for i in 0..n {
        if connections[b * n + i] {
            b_conn.push(i);
        }
        if connections[a * n + i] {
            a_conn.push(i);
        }
    }
    for ac in &a_conn {
        for bc in &b_conn {
            connections[ac * n + bc] = true;
            connections[bc * n + ac] = true;
        }
    }
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
    for i in 0..len {
        for j in 0..len {
            connections.push(i == j);
        }
    }

    // sort pairings by distance
    pairings.sort_by(|a, b| a.d2.cmp(&b.d2));
    // for p in &pairings {
    //     println!("{p:?}");
    // }

    // Add first <iterations> connections while skipping already connected nodes (avoid loops)
    // This is stupid, but actually i should be used... bad problem statement
    for i in 0..iterations {
        let a = pairings[i].a;
        let b = pairings[i].b;
        connect(&mut connections, a, b);
    }

    // Every group gets the id of its first element's index
    for i in 0..len {
        // if this node is already part of a group, skip
        if nodes[i].group_id != -1 {
            continue;
        }
        nodes[i].group_id = i as i32;
        for j in 0..len {
            if connections[i * len + j] {
                if nodes[j].group_id != -1 && nodes[j].group_id != i as i32 {
                    println!("Unexpected group id connection! {i} {j}");
                }
                // this is connected to node[i]
                nodes[j].group_id = i as i32;
            }
        }
    }

    let mut counts = vec![0; len];
    for i in 0..len {
        counts[nodes[i].group_id as usize] += 1;
    }

    let mut acc: u64 = 1;
    // this is wasted work
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
