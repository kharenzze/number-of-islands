use std::usize;

struct Solution;

struct Point {
    x: usize,
    y: usize,
}

const ZERO:usize = '0' as usize;

struct Game {
    grid: Vec<Vec<usize>>,
    dimensions: Point,
}

impl Game {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let dim = Point {
            x: grid.len(),
            y: grid[0].len(),
        };
        let g:Vec<Vec<usize>> = grid.into_iter().map(|line| {
            line.into_iter().map(|c| (c as usize) - ZERO).collect()
        }).collect();
        Self {
            grid: g,
            dimensions: dim,
        }
    }
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {

    }
}

fn main() {
    let grid:Vec<Vec<char>> = vec![vec!['1','1','1','1','0'],vec!['1','1','0','1','0'],vec!['1','1','0','0','0'],vec!['0','0','0','0','0']];
    println!("{}", Solution::num_islands(grid));
}
