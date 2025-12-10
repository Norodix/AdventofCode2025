#[derive(Debug)]
struct Node {
    x: i64,
    y: i64,
}
impl Node {
    fn clone(&self) -> Node {
        Node {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Debug)]
struct Edge {
    a: Node,
    b: Node,
}

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

fn solve(filepath: &str) -> u64 {
    // read lines into vector of nodes
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let mut nodes: Vec<Node> = vec![];
    for l in f.lines() {
        let mut p = l.split(',');
        let n: Node = Node {
            x: p.next().unwrap().parse().unwrap(),
            y: p.next().unwrap().parse().unwrap(),
        };
        nodes.push(n);
    }
    let len = nodes.len();

    // create n*n vector of each pairing
    // precompute areas
    let mut max_area = 0_i64;
    for i in 0..len {
        for j in (i + 1)..len {
            let area =
                ((nodes[i].x - nodes[j].x).abs() + 1) * ((nodes[i].y - nodes[j].y).abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area as u64
}

fn do_segments_intersect(h: &Edge, e: &Edge) -> bool {
    // H is always horizontal and starts from 0
    if e.a.x == e.b.x {
        // edge is vertical ->
        // check one is bigger one is lower on y, both are smaller on x
        // (equal is caught in other check)
        let min_y = std::cmp::min(e.a.y, e.b.y);
        let max_y = std::cmp::max(e.a.y, e.b.y);
        let min_x = std::cmp::min(h.a.x, h.b.x);
        let max_x = std::cmp::max(h.a.x, h.b.x);
        // println!("miny: {min_y}, maxy: {max_y}, minx: {min_x}, maxx: {max_x}");
        // the strict smaller on min_y is very important
        // it makes sure that in each case coreners are counted correctly
        // otherwise U shapes and Z shapes and similar can cause issues
        if min_y < h.a.y && max_y >= h.a.y && min_x <= e.a.x && max_x >= e.a.x {
            return true;
        }
    } else {
        // edge is horizontal
        // if fully
        // only consider it inside if point is right on edge
        // no need to calculate it here, caught outside
    }
    false
}

fn is_vertex_on_edge(v: &Node, e: &Edge) -> bool {
    let mut on_edge = false;
    if v.x == e.a.x && v.x == e.b.x {
        // vertical, all x equal
        let min = std::cmp::min(e.a.y, e.b.y);
        let max = std::cmp::max(e.a.y, e.b.y);
        if v.y <= max && v.y >= min {
            on_edge = true;
        }
    }
    if v.y == e.a.y && v.y == e.b.y {
        // horizontal, all y equal
        let min = std::cmp::min(e.a.x, e.b.x);
        let max = std::cmp::max(e.a.x, e.b.x);
        if v.x <= max && v.x >= min {
            on_edge = true;
        }
    }

    on_edge
}

fn vertex_in_polygon(v: &Node, poly: &Vec<Edge>) -> bool {
    // use raycasting method
    // Since all points are positive, 0 is a good lower bound
    // Loop through edges and check if the line (0,vy) - (vx,vy) intersects it
    let mut cross_count = 0;
    for e in poly {
        if is_vertex_on_edge(v, e) {
            return true;
        }
        let h = Edge {
            a: Node { x: 0, y: v.y },
            b: Node { x: v.x, y: v.y },
        };
        if do_segments_intersect(&h, e) {
            // println!("{h:?} X {e:?}");
            cross_count += 1;
        }
    }
    // println!("Cross count for {v:?}: {cross_count}");
    if cross_count % 2 == 0 { false } else { true }
}

fn edge_in_polygon(e: &Edge, poly: &Vec<Edge>) -> bool {
    let dx = (e.b.x - e.a.x).signum();
    let dy = (e.b.y - e.a.y).signum();

    let mut c = e.a.clone();
    while !(c.x == e.b.x && c.y == e.b.y) {
        if !vertex_in_polygon(&c, poly) {
            return false;
        }
        c.x += dx;
        c.y += dy;
    }
    true
}

// helper to create edge
fn edge(x1: i64, y1: i64, x2: i64, y2: i64) -> Edge {
    Edge {
        a: Node { x: x1, y: y1 },
        b: Node { x: x2, y: y2 },
    }
}

fn solve2(filepath: &str) -> u64 {
    // read lines into vector of nodes
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let mut nodes: Vec<Node> = vec![];
    for l in f.lines() {
        let mut p = l.split(',');
        let n: Node = Node {
            x: p.next().unwrap().parse().unwrap(),
            y: p.next().unwrap().parse().unwrap(),
        };
        nodes.push(n);
    }
    let len = nodes.len();

    // create all edges from pairs of nodes
    let mut edges: Vec<Edge> = vec![];
    for n in 0..nodes.len() {
        edges.push(Edge {
            a: nodes[n].clone(),
            b: nodes[(n + 1) % nodes.len()].clone(),
        })
    }
    // loop through all pairings
    let mut max_area = 0_i64;
    for i in 0..len {
        for j in (i + 1)..len {
            let area =
                ((nodes[i].x - nodes[j].x).abs() + 1) * ((nodes[i].y - nodes[j].y).abs() + 1);
            if area < max_area {
                // No point to check if inside, too small anyway
                continue;
            }
            // Area is interesting, check the 4 corners first
            let mut corners_in = true;
            for x in [nodes[i].x, nodes[j].x] {
                for y in [nodes[i].y, nodes[j].y] {
                    let a = Node { x: x, y: y };
                    corners_in &= vertex_in_polygon(&a, &edges);
                }
            }
            if !corners_in {
                continue;
            }
            // Corners are in and area is big, check all edges
            let x1 = nodes[i].x;
            let y1 = nodes[i].y;
            let x2 = nodes[j].x;
            let y2 = nodes[j].y;
            let e1 = edge(x1, y1, x1, y2);
            let e2 = edge(x1, y2, x2, y2);
            let e3 = edge(x2, y2, x2, y1);
            let e4 = edge(x2, y1, x1, y1);
            let mut edges_in = true;
            for e in [e1, e2, e3, e4] {
                edges_in &= edge_in_polygon(&e, &edges);
                if !edges_in {
                    break;
                }
            }
            if edges_in {
                // everything is correct, this is the new max
                max_area = area;
            }
        }
    }

    max_area as u64
    //   if all 4 vertices are in polygon
    //       check all elements of edges are in polygon -> all rectagle is in polygon, consider it
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("example"), 50);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2("example"), 24);
    }

    #[test]
    fn draw_test() {
        let f = std::fs::read_to_string("example").expect("File could not be read");
        let mut nodes: Vec<Node> = vec![];
        for l in f.lines() {
            let mut p = l.split(',');
            let n: Node = Node {
                x: p.next().unwrap().parse().unwrap(),
                y: p.next().unwrap().parse().unwrap(),
            };
            nodes.push(n);
        }
        let len = nodes.len();

        // create all edges from pairs of nodes
        let mut edges: Vec<Edge> = vec![];
        for n in 0..nodes.len() {
            edges.push(Edge {
                a: nodes[n].clone(),
                b: nodes[(n + 1) % nodes.len()].clone(),
            })
        }
        for r in 0..10 {
            for c in 0..12 {
                let n = Node { x: c, y: r };
                if vertex_in_polygon(&n, &edges) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }

    #[test]
    fn intersect_test() {
        let h = Edge {
            a: Node { x: 0, y: 10 },
            b: Node { x: 10, y: 10 },
        };
        // matches but fully inside -> fail
        let e1 = Edge {
            a: Node { x: 3, y: 10 },
            b: Node { x: 7, y: 10 },
        };
        // crosses normally
        let e2 = Edge {
            a: Node { x: 5, y: 5 },
            b: Node { x: 5, y: 15 },
        };
        // matches and continues outside
        let e3 = Edge {
            a: Node { x: 3, y: 10 },
            b: Node { x: 13, y: 10 },
        };
        // both e2 e3 fail
        assert_eq!(do_segments_intersect(&h, &e1), false);
        assert_eq!(do_segments_intersect(&h, &e2), true);
    }
}
