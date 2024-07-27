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

fn bfs(
    visited: &mut HashSet<Cell>,
    map: &Vec<Vec<u8>>,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
) {
    let mut queue: VecDeque<Cell> = VecDeque::new();
    queue.push_back(Cell {
        x: x as i32,
        y: y as i32,
    });

    let bound = Cell {
        x: width as i32,
        y: height as i32,
    };

    while (!queue.is_empty()) {
        let current_cell: Cell = queue.pop_front().unwrap();

        if visited.contains(&current_cell) {
            continue;
        }

        visited.insert(current_cell);

        let mut direction: Cell = Cell { x: 1, y: 0 };
        for i in 0..4 {
            let new_cell = Cell {
                x: current_cell.x + direction.x,
                y: current_cell.y + direction.y,
            };
            direction.rotate();

            if new_cell.out_of_bound(&bound) || new_cell.is_wall(map) {
                continue;
            }

            queue.push_back(new_cell);
        }
    }
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
    let mut map: Vec<Vec<u8>> = vec![vec![1; height]; width];

    for x in 0..width {
        str = String::new();
        io::stdin().read_line(&mut str).expect("Can't read map");
        for y in 0..height {
            if &str[y..y + 1] == "#" {
                map[x][y] = 1;
            } else {
                map[x][y] = 0;
            }
        }
    }

    let mut visited: HashSet<Cell> = HashSet::new();

    let mut count = 0;
    for x in 0..width {
        for y in 0..height {
            let cell = Cell {
                x: x as i32,
                y: y as i32,
            };
            if cell.is_wall(&map) || visited.contains(&cell) {
                continue;
            }
            bfs(&mut visited, &map, width, height, x, y);
            count += 1;
        }
    }

    print!("{}", count);
}
