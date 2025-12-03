fn main() {
    println!("{}", solve("input"));
    println!("{}", solve2("input"));
}

fn solve(filepath: &str) -> u32 {
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let mut sum = 0;
    for l in f.lines() {
        let l = l.trim();
        let nums: Vec<u32> = l.chars().map(|x| x.to_digit(10).unwrap()).collect();
        sum += get_max(nums);
    }
    sum
}

fn solve2(filepath: &str) -> u64 {
    let f = std::fs::read_to_string(filepath).expect("File could not be read");
    let mut sum: u64 = 0;
    for l in f.lines() {
        let l = l.trim();
        let nums: Vec<u32> = l.chars().map(|x| x.to_digit(10).unwrap()).collect();
        sum += get_max_12(nums);
    }
    sum
}

fn get_max_12(nums: Vec<u32>) -> u64 {
    let mut sum: u64 = 0;
    let mut max_index: u32 = 0 as u32;
    let mut max: u32 = 0;
    for i in (0..12).rev() {
        let end: usize = nums.len() - i;
        let start: usize = max_index as usize;
        let offset: u32;
        (max, offset) = get_max_slice(&nums[start..end]);
        sum += (max as u64) * 10u64.pow(i as u32);
        max_index += offset + 1;
    }
    sum
}

fn get_max(nums: Vec<u32>) -> u32 {
    let mut max;
    let mut max_index: u32;
    let mut sum = 0;
    (max, max_index) = get_max_slice(&nums[0..(nums.len() - 1)]);
    sum += max * 10;
    let len = nums.len();
    let start: usize = max_index as usize + 1;
    (max, max_index) = get_max_slice(&nums[start..len]);
    sum += max;
    sum
}

// Return (max, index) tuple
fn get_max_slice(nums: &[u32]) -> (u32, u32) {
    let mut max_index: u32 = 0;
    let mut max = 0;
    for i in 0..nums.len() {
        if nums[i] > max {
            max = nums[i];
            max_index = i as u32;
        }
    }
    (max, max_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_max_test() {
        assert_eq!(
            get_max(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1,]),
            98
        );
        assert_eq!(
            get_max(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9,]),
            89
        );
        assert_eq!(
            get_max(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8,]),
            78
        );
        assert_eq!(
            get_max(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1,]),
            92
        );
    }

    #[test]
    fn example1() {
        assert_eq!(solve("example"), 357);
    }

    #[test]
    fn get_max_12_test() {
        assert_eq!(
            get_max_12(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1,]),
            987654321111
        );
        assert_eq!(
            get_max_12(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9,]),
            811111111119
        );
        assert_eq!(
            get_max_12(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8,]),
            434234234278
        );
        assert_eq!(
            get_max_12(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1,]),
            888911112111
        );
    }

    #[test]
    fn example2() {
        assert_eq!(solve2("example"), 3121910778619u64);
    }
}
