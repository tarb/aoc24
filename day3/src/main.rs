use regex::Regex;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(INPUT)
        .filter_map(|cap| {
            let a = cap[1].parse::<usize>();
            let b = cap[2].parse::<usize>();
            a.and_then(|a| b.map(|b| (a, b))).ok()
        })
        .map(|(a, b)| a * b)
        .sum::<usize>()
}

fn part2() -> usize {
    enum Instruction {
        Do,
        Dont,
        Mul(usize, usize),
    }

    Regex::new(r"(don't\(\))|(do\(\))|(mul\((\d+),(\d+)\))")
        .unwrap()
        .captures_iter(INPUT)
        .filter_map(|cap| match cap.get(0).map(|c| c.as_str()) {
            Some("do()") => Some(Instruction::Do),
            Some("don't()") => Some(Instruction::Dont),
            Some(_) => {
                let a = cap[4].parse::<usize>();
                let b = cap[5].parse::<usize>();
                a.and_then(|a| b.map(|b| (a, b)))
                    .ok()
                    .map(|(n1, n2)| Instruction::Mul(n1, n2))
            }
            None => None,
        })
        .fold(
            (Instruction::Do, 0),
            |(current, sum), instruction| match instruction {
                Instruction::Do => (Instruction::Do, sum),
                Instruction::Dont => (Instruction::Dont, sum),
                Instruction::Mul(a, b) => match current {
                    Instruction::Do => (Instruction::Do, sum + a * b),
                    Instruction::Dont => (Instruction::Dont, sum),
                    _ => unreachable!(),
                },
            },
        )
        .1
}
