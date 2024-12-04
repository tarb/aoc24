const INPUT: &str = include_str!("./input.txt");

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let board = INPUT.lines().collect::<Vec<_>>();
    let bounds = (board.len(), board[0].len());

    let positions = board.iter().enumerate().flat_map(|(c, s)| {
        s.bytes()
            .enumerate()
            .filter(|(_, b)| *b == b'X')
            .map(move |(r, _)| (c, r))
            .flat_map(|(r, c)| {
                let up = (r > 2).then(|| [(r, c), (r - 1, c), (r - 2, c), (r - 3, c)]);
                let down = (r + 3 < bounds.0).then(|| [(r, c), (r + 1, c), (r + 2, c), (r + 3, c)]);
                let left = (c > 2).then(|| [(r, c), (r, c - 1), (r, c - 2), (r, c - 3)]);
                let right =
                    (c + 3 < bounds.1).then(|| [(r, c), (r, c + 1), (r, c + 2), (r, c + 3)]);
                let down_left = (r + 3 < bounds.0 && c > 2)
                    .then(|| [(r, c), (r + 1, c - 1), (r + 2, c - 2), (r + 3, c - 3)]);
                let down_right = (r + 3 < bounds.0 && c + 3 < bounds.1)
                    .then(|| [(r, c), (r + 1, c + 1), (r + 2, c + 2), (r + 3, c + 3)]);
                let up_left = (r > 2 && c > 2)
                    .then(|| [(r, c), (r - 1, c - 1), (r - 2, c - 2), (r - 3, c - 3)]);
                let up_right = (r > 2 && c + 3 < bounds.1)
                    .then(|| [(r, c), (r - 1, c + 1), (r - 2, c + 2), (r - 3, c + 3)]);

                [
                    up, down, left, right, down_left, down_right, up_left, up_right,
                ]
            })
            .flatten()
    });

    positions
        .into_iter()
        .map(|p| {
            p.into_iter()
                .map(|(r, c)| board[r].as_bytes()[c])
                .eq("XMAS".bytes())
        })
        .filter(|b| *b)
        .count()
}

fn part2() -> usize {
    let board = INPUT.lines().collect::<Vec<_>>();
    let bounds = (board.len(), board[0].len());

    let positions = board.iter().enumerate().flat_map(|(c, s)| {
        s.bytes()
            .enumerate()
            .filter(|(_, b)| *b == b'A')
            .map(move |(r, _)| (c, r))
            .filter(|(r, c)| *r > 0 && *r < bounds.0 - 1 && *c > 0 && *c < bounds.1 - 1)
            .map(|(r, c)| {
                [
                    (r - 1, c - 1),
                    (r - 1, c + 1),
                    (r, c), // A
                    (r + 1, c - 1),
                    (r + 1, c + 1),
                ]
            })
    });

    positions
        .into_iter()
        .map(|p| {
            let s = p
                .into_iter()
                .map(|(r, c)| board[r].as_bytes()[c])
                .collect::<Vec<_>>();

            ["MSAMS", "SMASM", "MMASS", "SSAMM"]
                .iter()
                .any(|&s2| s == s2.as_bytes())
        })
        .filter(|b| *b)
        .count()
}
