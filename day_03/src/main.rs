fn main() {
    println!("{}", solve("input"));
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

fn get_max(nums: Vec<u32>) -> u32 {
    let mut max_index = 0;
    let mut sum = 0;
    let mut max = 0;
    for i in 0..(nums.len() - 1) {
        if nums[i] > max {
            max = nums[i];
            max_index = i;
        }
    }
    sum += max * 10;
    max = 0;
    for i in (max_index + 1)..nums.len() {
        if nums[i] > max {
            max = nums[i];
        }
    }
    sum += max;
    sum
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
}
