fn main() {
    println!("{}", solve("input"));
    println!("{}", solve2("input"));
}

fn is_beam(c: &char) -> bool {
    *c == '|' || *c == 'S'
}

fn solve(filepath: &str) -> u64 {
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let cols = f.chars().position(|x| x == '\n').unwrap();
    let mut tiles: Vec<char> = f.chars().filter(|x| *x != '\n').collect();
    let mut splits = 0;
    for i in cols..tiles.len() {
        if is_beam(&tiles[i - cols]) {
            if tiles[i] == '^' {
                splits += 1;
                // no splitters are at the edge, no special case
                tiles[i - 1] = '|';
                tiles[i + 1] = '|';
            } else {
                tiles[i] = '|';
            }
        }
    }
    splits
}

fn solve2(filepath: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("example"), 21);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2("example"), 0);
    }
}
