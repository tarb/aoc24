use std::{
    io::BufRead,
    ops::{Index, IndexMut},
};

const INPUT: &[u8] = include_bytes!("./input.txt");

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let mut board = Board::new(INPUT);

    loop {
        let state = board.advance();
        match state {
            State::Running => continue,
            _ => break
        }
    }

    board.visited()
}

fn part2() -> usize {
    let board = Board::new(INPUT);

    board
        .tiles
        .iter()
        .enumerate()
        .filter(|&(_, t)| matches!(t, Tile::Empty))
        .filter_map(|(i, _)| {
            let mut board = board.clone();
            board.tiles[i] = Tile::Blocked;

            loop {
                let state = board.advance();
                match state {
                    State::Running => continue,
                    State::Ended => break None,
                    State::Loop => break Some(i),
                }
            }
        })
        .count()
}

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    Blocked,
    Visited(u8),
}

#[derive(Copy, Clone, PartialEq)]
enum State {
    Running,
    Ended,
    Loop,
}

#[derive(Clone)]
struct Board {
    dimensions: (usize, usize),
    tiles: Vec<Tile>,
    player_pos: (usize, usize),
    player_dir: u8,
}

impl Board {
    const UP: u8 = 1 << 1;
    const DOWN: u8 = 1 << 2;
    const LEFT: u8 = 1 << 3;
    const RIGHT: u8 = 1 << 4;

    fn new(bs: &[u8]) -> Self {
        let width = bs.iter().position(|&b| b == b'\n').unwrap_or_default();
        let height: usize = bs.lines().count();

        let tiles = bs
            .iter()
            .filter(|&&b| b != b'\n')
            .map(|b| match b {
                b'.' => Tile::Empty,
                b'#' => Tile::Blocked,
                b'v' => Tile::Visited(Self::DOWN),
                b'^' => Tile::Visited(Self::UP),
                b'<' => Tile::Visited(Self::LEFT),
                b'>' => Tile::Visited(Self::RIGHT),
                _ => unreachable!(),
            })
            .collect::<Vec<Tile>>();

        let (player_pos, player_dir) = bs
            .iter()
            .filter(|&&b| b != b'\n')
            .enumerate()
            .find_map(|(i, &b)| match b {
                b'v' => Some(((i % height, i / width), Self::DOWN)),
                b'^' => Some(((i % height, i / width), Self::UP)),
                b'<' => Some(((i % height, i / width), Self::LEFT)),
                b'>' => Some(((i % height, i / width), Self::RIGHT)),
                _ => None,
            })
            .expect("No player found");

        Self {
            dimensions: (width, height),
            tiles,
            player_pos,
            player_dir,
        }
    }

    fn advance(&mut self) -> State {
        let next_pos = match self.player_dir {
            Self::UP => (self.player_pos.1 > 0).then(|| (self.player_pos.0, self.player_pos.1 - 1)),
            Self::DOWN => (self.player_pos.1 < self.dimensions.1 - 1)
                .then(|| (self.player_pos.0, self.player_pos.1 + 1)),
            Self::LEFT => {
                (self.player_pos.0 > 0).then(|| (self.player_pos.0 - 1, self.player_pos.1))
            }
            Self::RIGHT => (self.player_pos.0 < self.dimensions.0 - 1)
                .then(|| (self.player_pos.0 + 1, self.player_pos.1)),
            _ => unreachable!(),
        };

        // if next_pos is None, then we are at the edge of the board
        let next_pos = match next_pos {
            None => return State::Ended,
            Some(np) => np,
        };

        let next_tile = self[next_pos];

        if next_tile == Tile::Blocked {
            let next_dir = match self.player_dir {
                Self::UP => Self::RIGHT,
                Self::RIGHT => Self::DOWN,
                Self::DOWN => Self::LEFT,
                Self::LEFT => Self::UP,
                _ => unreachable!(),
            };

            self.player_dir = next_dir;
        } else {
            // detect if we have been on the same spot, facing the same direction before
            // if so, we are in a loop
            if let Tile::Visited(v) = next_tile {
                if v & self.player_dir != 0 {
                    return State::Loop;
                }
            }

            self[next_pos] = match next_tile {
                Tile::Empty => Tile::Visited(self.player_dir),
                Tile::Visited(v) => Tile::Visited(v | self.player_dir),
                _ => unreachable!(),
            };

            self.player_pos = next_pos;
        }

        State::Running
    }

    fn visited(&self) -> usize {
        self.tiles
            .iter()
            .filter(|&&t| matches!(t, Tile::Visited(_)))
            .count()
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Tile;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.tiles[y * self.dimensions.0 + x]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.tiles[y * self.dimensions.0 + x]
    }
}
