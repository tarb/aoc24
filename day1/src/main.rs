use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> i32 {
    // 2 vecs to store the numbers
    let (mut nums1, mut nums2): (Vec<i32>, Vec<i32>) = INPUT
        .lines()
        .filter_map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect_tuple::<(i32, i32)>()
        })
        .unzip();

    // Sort the vecs
    nums1.sort();
    nums2.sort();

    // zip the vecs and calculate the difference
    nums1
        .iter()
        .zip(nums2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part2() -> i32 {
    // collect into map where keys are left number's and values are tuple's of if present in left and count in right
    let mut map = HashMap::with_capacity(1000);
    INPUT
        .lines()
        .filter_map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect_tuple::<(i32, i32)>()
        })
        .for_each(|(a, b)| {
            // inc counter for right list
            map.entry(b).or_insert((false, 0)).1 += 1;
            // mark present for left list
            map.entry(a).or_insert((true, 0)).0 = true;
        });

    map.iter()
        .filter(|(_, v)| v.0)
        .map(|(k, (_, count))| k * count)
        .sum::<i32>()
}
