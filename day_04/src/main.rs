fn main() {
    println!("{}", solve("input"));
    println!("{}", solve2("input"));
}

fn print_tiles(tiles: &Vec<char>, cols: usize) {
    for r in 0..(tiles.len() / cols) {
        for c in 0..cols {
            print!("{}", tiles[r * cols + c]);
        }
        print!("\n");
    }
    print!("\n");
}

fn cleanup(tiles: &mut Vec<char>, cols: usize) -> u32 {
    let mut counts = vec![0; tiles.len()];
    let cols = cols as i32;
    let rows = tiles.len() as i32 / cols;

    let mut valid = 0;
    for t in 0..tiles.len() {
        if tiles[t] != '@' {
            continue;
        }
        let mut cnt = 0;
        for r in -1i32..=1 {
            for c in -1i32..=1 {
                let t = t as i32;
                let row = t / cols;
                if row + r < 0 || row + r >= rows {
                    continue;
                }
                let col = t % cols;
                if col + c < 0 || col + c >= cols {
                    continue;
                }
                let index = t + r * cols + c;
                if index < 0 || index >= tiles.len() as i32 {
                    continue;
                }
                let index = index as usize;
                if tiles[index] == '@' {
                    cnt += 1;
                }
            }
        }
        counts[t] = cnt;
        // Fewer than four -> counting itself <= 4
        if cnt <= 4 {
            valid += 1;
            // println!("Valid at {},{}", t / cols as usize, t % cols as usize);
        }
    }
    for t in 0..tiles.len() {
        if counts[t] <= 4 && counts[t] > 0 {
            tiles[t] = '.';
        }
    }
    valid
}

fn solve(filepath: &str) -> u32 {
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let cols = f.find('\n').unwrap() as i32;
    let tiles: Vec<char> = f.chars().filter(|x| *x != '\n').collect();
    let rows = (tiles.len() as i32) / cols;
    print_tiles(&tiles, cols as usize);
    let mut counts = tiles.clone();

    let mut valid = 0;
    for t in 0..tiles.len() {
        if tiles[t] != '@' {
            continue;
        }
        let mut cnt = 0;
        for r in -1i32..=1 {
            for c in -1i32..=1 {
                let t = t as i32;
                let row = t / cols;
                if row + r < 0 || row + r >= rows {
                    continue;
                }
                let col = t % cols;
                if col + c < 0 || col + c >= cols {
                    continue;
                }
                let index = t + r * cols + c;
                if index < 0 || index >= tiles.len() as i32 {
                    continue;
                }
                let index = index as usize;
                if tiles[index] == '@' {
                    cnt += 1;
                }
            }
        }
        counts[t] = char::from_digit(cnt, 10).unwrap();
        // Fewer than four -> counting itself <= 4
        if cnt <= 4 {
            valid += 1;
            println!("Valid at {},{}", t / cols as usize, t % cols as usize);
        }
    }
    print_tiles(&counts, cols as usize);
    valid
}

fn solve2(filepath: &str) -> u32 {
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let cols = f.find('\n').unwrap();
    let mut tiles: Vec<char> = f.chars().filter(|x| *x != '\n').collect();
    let mut sum = 0;

    let mut valid = cleanup(&mut tiles, cols);
    while valid > 0 {
        print_tiles(&tiles, cols);
        sum += valid;
        valid = cleanup(&mut tiles, cols);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("example"), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2("example"), 43);
    }
}
