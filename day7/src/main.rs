const INPUT: &str = include_str!("./input.txt");

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u64 {
    INPUT
        .lines()
        .filter_map(Calibration::new)
        .filter(|c| c.is_valid_p1())
        .map(|c| c.answer)
        .sum::<u64>()
}

fn part2() -> u64 {
    INPUT
        .lines()
        .filter_map(Calibration::new)
        .filter(|c| c.is_valid_p2())
        .map(|c| c.answer)
        .sum::<u64>()
}

struct Calibration {
    pub answer: u64,
    input: Vec<u64>,
}

impl Calibration {
    pub fn new(line: &str) -> Option<Self> {
        let mut nums = line
            .split(&[' ', ':'])
            .filter(|&c| !c.is_empty())
            .map(|num_str| num_str.parse::<u64>().unwrap());

        nums.next().map(|n| Self {
            answer: n,
            input: nums.collect(),
        })
    }

    pub fn is_valid_p1(&self) -> bool {
        Self::search_p1(self.answer, self.input[0], &self.input[1..])
    }

    fn search_p1(target: u64, current: u64, numbers: &[u64]) -> bool {
        if numbers.len() == 1 {
            return (numbers[0] * current == target) || (numbers[0] + current == target);
        }

        Self::search_p1(target, current + numbers[0], &numbers[1..])
            || Self::search_p1(target, current * numbers[0], &numbers[1..])
    }

    pub fn is_valid_p2(&self) -> bool {
        Self::search_p2(self.answer, self.input[0], &self.input[1..])
    }

    fn search_p2(target: u64, current: u64, numbers: &[u64]) -> bool {
        if numbers.len() == 1 {
            return (numbers[0] * current == target)
                || (numbers[0] + current == target)
                || (concat(current, numbers[0]) == target);
        }

        Self::search_p2(target, current + numbers[0], &numbers[1..])
            || Self::search_p2(target, current * numbers[0], &numbers[1..])
            || Self::search_p2(target, concat(current, numbers[0]), &numbers[1..])
    }
}

fn concat(mut a: u64, b: u64) -> u64 {
    if b == 0 {
        a *= 10;
    } else {
        let mut tb = b;
        while tb > 0 {
            tb /= 10;
            a *= 10;
        }
    }

    a + b
}

mod tests {
    #[test]
    fn test_concat() {
        assert_eq!(super::concat(1, 2), 12);
        assert_eq!(super::concat(123, 123), 123123);
        assert_ne!(super::concat(123, 123), 8924);
    }
}
