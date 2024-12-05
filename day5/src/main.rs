use fnv::FnvHashMap;
use std::cmp::Ordering;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    // iterate over the input only once, reuse this iterator for both parts
    let mut lines = INPUT.lines();

    // parse the rules - top of the file
    let rules = lines
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|s| {
            let (a, b) = s.split_once('|').unwrap();
            [a.parse::<u8>().unwrap(), b.parse::<u8>().unwrap()]
        })
        .fold(FnvHashMap::default(), |mut m, [item, page]| {
            let values: &mut Vec<u8> = m.entry(item).or_default();
            values.push(page);
            m
        });

    // parse over the pages - bottom of the file
    let pages = lines
        .map(|s| {
            s.split(',')
                .filter_map(|s| s.parse::<u8>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&rules, &pages));
    println!("Part 2: {}", part2(&rules, &pages));
}

fn part1(rules: &FnvHashMap<u8, Vec<u8>>, pages: &[Vec<u8>]) -> usize {
    // iterator of only VALID pages
    let valid_pages = pages.iter().filter(|page| {
        page.iter().enumerate().all(|(i, item)| {
            if let Some(set) = rules.get(item) {
                page[..i].iter().all(|n| !set.contains(n))
            } else {
                true
            }
        })
    });

    valid_pages
        .map(|page| page[page.len() / 2] as usize)
        .sum::<usize>()
}

fn part2(rules: &FnvHashMap<u8, Vec<u8>>, pages: &[Vec<u8>]) -> usize {
    // iterator of only INVALID pages
    let invalid_pages = pages.iter().filter(|page| {
        !page.iter().enumerate().all(|(i, item)| {
            if let Some(set) = rules.get(item) {
                page[..i].iter().all(|n| !set.contains(n))
            } else {
                true
            }
        })
    });

    // fix the order of the invalid pages, and sum the middle element of each page
    invalid_pages
        .map(|page| {
            let mut next = page.clone();
            next.sort_by(|a, b| {
                if rules.contains_key(a) && rules[a].contains(b) {
                    Ordering::Less
                } else if rules.contains_key(b) && rules[b].contains(a) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            next
        })
        .map(|page| page[page.len() / 2] as usize)
        .sum::<usize>()
}
