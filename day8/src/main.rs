use fnv::FnvHashMap;
use itertools::Itertools;
use std::cmp;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let dimensions = parse_dimensions(INPUT);
    let pairs = parse_pairs(INPUT);

    println!("Part 1: {}", part1(dimensions, &pairs));
    println!("Part 2: {}", part2(dimensions, &pairs));
}

fn part1((dim_x, dim_y): (i32, i32), vals: &[((i32, i32), (i32, i32))]) -> usize {
    vals.iter()
        .flat_map(|(a, b)| {
            let ((ax, ay), (bx, by)) = if a.0 < b.0 { (a, b) } else { (b, a) };
            let abs_x = (a.0 - b.0).abs();
            let abs_y = (a.1 - b.1).abs();

            if by > ay {
                [(ax - abs_x, ay - abs_y), (bx + abs_x, by + abs_y)]
            } else {
                [(ax - abs_x, ay + abs_y), (bx + abs_x, by - abs_y)]
            }
        })
        .filter(|&(x, y)| x >= 0 && y >= 0 && x < dim_x && y < dim_y)
        .unique()
        .count()
}

fn part2((dim_x, dim_y): (i32, i32), vals: &[((i32, i32), (i32, i32))]) -> usize {
    vals.iter()
        .flat_map(|(a, b)| {
            let ((ax, ay), (bx, by)) = if a.0 < b.0 { (a, b) } else { (b, a) };
            let abs_x = (a.0 - b.0).abs();
            let abs_y = (a.1 - b.1).abs();

            if by > ay {
                let min_count = cmp::min(ax / abs_x, ay / abs_y);
                let max_count = cmp::min((dim_x - ax) / abs_x, (dim_y - ay) / abs_y);

                let down = (0..=min_count).map(|i| (ax - (i * abs_x), ay - (i * abs_y)));
                let up = (0..=max_count).map(|i| (bx + (i * abs_x), by + (i * abs_y)));

                down.chain(up).collect::<Vec<_>>()
            } else {
                let min_count = cmp::min(ax / abs_x, (dim_y - ay) / abs_y);
                let max_count = cmp::min((dim_x - ax) / abs_x, ay / abs_y);

                let down = (0..=min_count).map(|i| (ax - (i * abs_x), ay + (i * abs_y)));
                let up = (0..=max_count).map(|i| (bx + (i * abs_x), by - (i * abs_y)));

                down.chain(up).collect::<Vec<_>>()
            }
        })
        .filter(|&(x, y)| x >= 0 && y >= 0 && x < dim_x && y < dim_y)
        .unique()
        .count()
}

fn parse_dimensions(input: &str) -> (i32, i32) {
    (
        input.lines().next().unwrap().len() as i32,
        input.lines().count() as i32,
    )
}
fn parse_pairs(input: &'static str) -> Vec<((i32, i32), (i32, i32))> {
    input
        .lines()
        .enumerate()
        .fold(FnvHashMap::default(), |m, (i, l)| {
            l.bytes()
                .enumerate()
                .filter(|&(_, b)| b != b'.')
                .fold(m, |mut m, (j, b)| {
                    let values: &mut Vec<(i32, i32)> = m.entry(b).or_default();
                    values.push((i as i32, j as i32));
                    m
                })
        })
        .values()
        .flat_map(|v| v.iter().cartesian_product(v))
        .map(|(a, b)| (*a, *b))
        .filter(|&(i1, i2)| i1 != i2)
        .collect::<Vec<_>>()
}
