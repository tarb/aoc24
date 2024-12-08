use fnv::FnvHashMap;
use itertools::Itertools;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let dimensions = parse_dimensions(INPUT);
    let pairs = parse_pairs(INPUT);

    println!("Part 1: {}", part1(dimensions, &pairs));
    println!("Part 2: {}", part2(dimensions, &pairs));
}

fn part1((dim_x, dim_y): (i32, i32), vals: &[((i32, i32), (i32, i32))]) -> usize {
    vals.iter()
        .flat_map(|((ax, ay), (bx, by))| {
            let diff_x = ax - bx;
            let diff_y = ay - by;
            [(ax + diff_x, ay + diff_y), (bx - diff_x, by - diff_y)]
        })
        .filter(|&(x, y)| x >= 0 && y >= 0 && x < dim_x && y < dim_y)
        .unique()
        .count()
}

fn part2((dim_x, dim_y): (i32, i32), vals: &[((i32, i32), (i32, i32))]) -> usize {
    vals.iter()
        .flat_map(|((ax, ay), (bx, by))| {
            let diff_x = ax - bx;
            let diff_y = ay - by;

            let in_range = |&(x, y): &(i32, i32)| x >= 0 && y >= 0 && x < dim_x && y < dim_y;
            let a_iter = (0..)
                .map(move |i| (ax + (i * diff_x), ay + (i * diff_y)))
                .take_while(in_range);
            let b_iter = (0..)
                .map(move |i| (bx - (i * diff_x), by - (i * diff_y)))
                .take_while(in_range);

            a_iter.chain(b_iter)
        })
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
