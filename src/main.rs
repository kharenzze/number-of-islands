use std::usize;
use std::ops::Add;

struct Solution;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

static STEPS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
];

enum Status {
    Water,
    Earth,
    Flagged(i32),
}

impl From<char> for Status {
    fn from(c: char) -> Self {
        match c {
            '0' => Self::Water,
            '1' => Self::Earth,
            _ => unreachable!()
        }
    }
}

struct Game {
    grid: Vec<Vec<Status>>,
    dimensions: Point,
}

impl Game {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let dim = Point {
            x: grid.len() as isize,
            y: grid[0].len() as isize,
        };
        let g:Vec<Vec<Status>> = grid.into_iter().map(|line| {
            line.into_iter().map(|c| Status::from(c)).collect()
        }).collect();
        Self {
            grid: g,
            dimensions: dim,
        }
    }

    fn is_point_inside_grid(&self, p: Point) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.dimensions.x && p.y < self.dimensions.y
    }

    fn get(&self, p: Point) -> &Status {
        if self.is_point_inside_grid(p) {
            return &self.grid[p.x as usize][p.y as usize];
        }
        &Status::Water
    }

    fn set_flag(&mut self, p: Point, flag: i32) {
        if self.is_point_inside_grid(p) {
            self.grid[p.x as usize][p.y as usize] = Status::Flagged(flag);
        }
    }

    fn play(&mut self) -> i32 {
        let mut count = 0;
        for i in 0..self.dimensions.x {
            for j in 0..self.dimensions.y {
                let pos = Point {x: i, y: j};
                match self.get(pos) {
                    &Status::Earth => {
                        count = count + 1;
                        self.explore(pos, count);
                    },
                    _ => ()
                }

            }
        }
        count
    }

    fn explore(&mut self, origin: Point, flag: i32) {
        self.set_flag(origin, flag);
        for p in STEPS.iter() {
            let next = origin + *p;
            match self.get(next) {
                &Status::Earth => self.explore(next, flag),
                _ => ()
            }
        }
    }
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut game = Game::new(grid);
        game.play()
    }
}

fn main() {
    let grid:Vec<Vec<char>> = vec![
        vec!['1','1','1','1','0'],
        vec!['1','1','0','1','0'],
        vec!['1','1','0','0','0'],
        vec!['0','0','0','0','1']
    ];
    println!("{}", Solution::num_islands(grid));
}
