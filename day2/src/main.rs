#![feature(iter_array_chunks)]

const INPUT: &str = include_str!("./input.txt");
type Number = i16;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    INPUT
        .lines()
        .map(|line| {
            let nums = line
                .split_ascii_whitespace()
                .filter_map(|x| x.parse::<i16>().ok())
                .collect::<Vec<i16>>();
            safe(&nums)
        })
        .filter(|x| *x)
        .count()
}

fn part2() -> usize {
    INPUT
        .lines()
        .map(|line| {
            let nums = line
                .split_ascii_whitespace()
                .filter_map(|x| x.parse::<Number>().ok())
                .collect::<Vec<_>>();
            if safe(&nums) {
                return true;
            }

            // iterate over all combinations removing one element
            let mut nums_sub = Vec::with_capacity(nums.len() - 1);
            for i in 0..nums.len() {
                nums.clone_into(&mut nums_sub);
                nums_sub.remove(i);

                if safe(&nums_sub) {
                    return true;
                }
            }

            false
        })
        .filter(|x| *x)
        .count()
}

fn safe(nums: &[Number]) -> bool {
    let inc = nums.windows(2).all(|p| p[0] < p[1]);
    let dec = nums.windows(2).all(|p| p[0] > p[1]);
    let dist = nums
        .windows(2)
        .all(|p| (p[0] - p[1]).abs() > 0 && (p[0] - p[1]).abs() < 4);

    (inc || dec) && dist
}
