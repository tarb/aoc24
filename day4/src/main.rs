const INPUT: &str = include_str!("./input.txt");

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let board = INPUT.lines().collect::<Vec<_>>();
    let (bounds_r, bounds_c) = (board.len(), board[0].len());

    // iterate over the board and find all the positions of 'X'
    let positions = board.iter().enumerate().flat_map(|(c, s)| {
        s.bytes()
            .enumerate()
            .filter(|(_, b)| *b == b'X')
            .map(move |(r, _)| (c, r))
    });

    // with the positions of x, lookup the 4 bytes that branch of in each direction
    let words = positions
        .flat_map(|(r, c)| {
            let up = (r > 2).then(|| [(r, c), (r - 1, c), (r - 2, c), (r - 3, c)]);
            let down = (r + 3 < bounds_r).then(|| [(r, c), (r + 1, c), (r + 2, c), (r + 3, c)]);
            let left = (c > 2).then(|| [(r, c), (r, c - 1), (r, c - 2), (r, c - 3)]);
            let right = (c + 3 < bounds_c).then(|| [(r, c), (r, c + 1), (r, c + 2), (r, c + 3)]);
            let down_left = (r + 3 < bounds_r && c > 2)
                .then(|| [(r, c), (r + 1, c - 1), (r + 2, c - 2), (r + 3, c - 3)]);
            let down_right = (r + 3 < bounds_r && c + 3 < bounds_c)
                .then(|| [(r, c), (r + 1, c + 1), (r + 2, c + 2), (r + 3, c + 3)]);
            let up_left =
                (r > 2 && c > 2).then(|| [(r, c), (r - 1, c - 1), (r - 2, c - 2), (r - 3, c - 3)]);
            let up_right = (r > 2 && c + 3 < bounds_c)
                .then(|| [(r, c), (r - 1, c + 1), (r - 2, c + 2), (r - 3, c + 3)]);
            [
                up, down, left, right, down_left, down_right, up_left, up_right,
            ]
        })
        .flatten()
        .map(|[a, b, c, d]| {
            let lookup = |pos: (usize, usize)| board[pos.0].as_bytes()[pos.1];
            [lookup(a), lookup(b), lookup(c), lookup(d)]
        });

    // count the number of 'XMAS' words
    words.filter(|bs| bs == b"XMAS").count()
}

fn part2() -> usize {
    let board = INPUT.lines().collect::<Vec<_>>();
    let (bounds_r, bounds_c) = (board.len(), board[0].len());

    // iterate over the board and find all the positions of 'A'
    let positions = board.iter().enumerate().flat_map(|(c, s)| {
        s.bytes()
            .enumerate()
            .filter(|(_, b)| *b == b'A')
            .map(move |(r, _)| (c, r))
    });

    // with the positions of A, lookup the 5 bytes that branch of in each direction
    let words = positions
        .filter(|(r, c)| *r > 0 && *r < bounds_r - 1 && *c > 0 && *c < bounds_c - 1)
        .map(|(r, c)| {
            [
                (r - 1, c - 1),
                (r - 1, c + 1),
                (r, c),
                (r + 1, c - 1),
                (r + 1, c + 1),
            ]
        })
        .map(|[a, b, c, d, e]| {
            let lookup = |pos: (usize, usize)| board[pos.0].as_bytes()[pos.1];
            [lookup(a), lookup(b), lookup(c), lookup(d), lookup(e)]
        });

    // count the number of valid words ('MSAMS', 'SMASM', 'MMASS', 'SSAMM')
    words
        .filter(|bs1| {
            [b"MSAMS", b"SMASM", b"MMASS", b"SSAMM"]
                .into_iter()
                .any(|bs2| bs1 == bs2)
        })
        .count()
}
