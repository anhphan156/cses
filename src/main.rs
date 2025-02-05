#![allow(unused)]
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
    io,
};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Cell {
    x: i32,
    y: i32,
}

impl Cell {
    fn out_of_bound(&self, bound: &Cell) -> bool {
        self.x < 0 || self.y < 0 || self.x >= bound.x || self.y >= bound.y
    }

    fn is_wall(&self, map: &Vec<Vec<u8>>) -> bool {
        map[self.x as usize][self.y as usize] == 1
    }

    fn rotate(&mut self) {
        self.x ^= self.y;
        self.y ^= self.x;
        self.x ^= self.y;
        self.x *= -1
    }
}

fn dfs(
    visited: &mut HashSet<Cell>,
    map: &Vec<Vec<u8>>,
    bound: &Cell,
    origin: &Cell,
    destination: &Cell,
) -> u32 {
    let mut stack: Vec<Cell> = Vec::new();
    stack.push(origin.clone());

    let mut min_step = u32::MAX;
    let mut step = 0;

    while (!stack.is_empty()) {
        let mut current_cell = stack.pop().unwrap();
        visited.insert(current_cell);
        step += 1;

        if current_cell == *destination {
            min_step = min(min_step, step);
        }

        let mut direction = Cell { x: 1, y: 0 };
        for i in 0..4 {
            let next_cell = Cell {
                x: current_cell.x + direction.x,
                y: current_cell.y + direction.y,
            };
            direction.rotate();

            if next_cell.out_of_bound(bound)
                || next_cell.is_wall(map)
                || visited.contains(&next_cell)
            {
                continue;
            }

            stack.push(next_cell);
        }
    }

    min_step - 1
}

fn main() {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");

    let first_line: Vec<usize> = str
        .split_whitespace()
        .map(|x| x.parse().unwrap_or(0))
        .collect();

    let width: usize = first_line[0];
    let height: usize = first_line[1];
    let bound = Cell {
        x: width as i32,
        y: height as i32,
    };

    let mut a_pos: Cell = Cell { x: 0, y: 0 };
    let mut b_pos: Cell = Cell { x: 0, y: 0 };
    let mut map: Vec<Vec<u8>> = vec![vec![1; height]; width];

    for x in 0..width {
        str = String::new();
        io::stdin().read_line(&mut str).expect("Can't read map");
        for y in 0..height {
            let cur = &str[y..y + 1];
            if cur == "#" {
                map[x][y] = 1;
            } else if cur == "A" {
                map[x][y] = 0;
                a_pos = Cell {
                    x: x as i32,
                    y: y as i32,
                };
            } else if cur == "B" {
                map[x][y] = 2;
                b_pos = Cell {
                    x: x as i32,
                    y: y as i32,
                };
            } else {
                map[x][y] = 0;
            }
        }
    }

    let mut visited: HashSet<Cell> = HashSet::new();
    let result = dfs(&mut visited, &map, &bound, &a_pos, &b_pos);
    print!("{}", result);
}
