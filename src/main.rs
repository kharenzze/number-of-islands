struct Solution;

struct Point {
    x: i32,
    y: i32,
}

struct Game {
    grid: Vec<Vec<char>>,
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {

    }
}

fn main() {
    let grid:Vec<Vec<char>> = vec![vec!['1','1','1','1','0'],vec!['1','1','0','1','0'],vec!['1','1','0','0','0'],vec!['0','0','0','0','0']];
    println!("{}", Solution::num_islands(grid));
}
