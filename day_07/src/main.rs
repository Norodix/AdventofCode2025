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

// Part 2 Algorithm:
// Loop through beam splitters line by line
// Each beam splitter looks up and sums up all the beams above it until the next splitter
// the sum is added to its left and right neighbors
// At the end, sum up all the beams up to the nearest beam splitter for each column
enum Tile {
    Count(u64),
    Splitter,
}

fn sum_above(tiles: &Vec<Tile>, cols: usize, i: usize) -> u64 {
    let mut sum = 0;
    let mut i = i as i64;
    let cols = cols as i64;
    i -= cols; // start 1 tile above
    while i > 0 {
        match tiles[i as usize] {
            Tile::Splitter => break,
            Tile::Count(x) => sum += x,
        }
        i -= cols;
    }
    sum
}

fn solve2(filepath: &str) -> u64 {
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let cols = f.chars().position(|x| x == '\n').unwrap();
    let mut tiles: Vec<Tile> = f
        .chars()
        .filter(|x| *x != '\n')
        .map(|x| match x {
            '.' => Tile::Count(0),
            'S' => Tile::Count(1),
            '^' => Tile::Splitter,
            _ => panic!("Unexpected char found: {x}"),
        })
        .collect();
    for i in cols..tiles.len() {
        match tiles[i] {
            Tile::Splitter => {
                let s = sum_above(&tiles, cols, i);
                if let Tile::Count(c) = tiles[i - 1] {
                    tiles[i - 1] = Tile::Count(c + s);
                }
                if let Tile::Count(c) = tiles[i + 1] {
                    tiles[i + 1] = Tile::Count(c + s);
                }
            }
            _ => (),
        }
    }

    // go through the last row and sum everything above it
    let mut sum = 0;
    for i in (tiles.len() - cols)..tiles.len() {
        sum += sum_above(&tiles, cols, i);
    }
    sum
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
        assert_eq!(solve2("example"), 40);
    }
}
