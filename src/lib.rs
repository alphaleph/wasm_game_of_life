mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let output = format!("Hello, {}!", name);
    alert(&output);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

#[wasm_bindgen]
impl Universe {

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        //Default seeding pattern
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell:: Dead
                }
            }).collect();
        
        Universe {
            width,
            height,
            cells
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbors = self.live_neighbor_count(row, col);
                
                let next_cell = match (cell, live_neighbors) {
                    //Rule 1: Live cells with < 2 live neighbors die
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    //Rule 2: Live cells with 2-3 live neighbors live on
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    //Rule 3: Live cells with 4+ live neighbors die
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    //Rule 4: Dead cells with 3 live neighbors become alive
                    (Cell::Dead, 3) => Cell::Alive,
                    //Other cases remain in their current state
                    (otherwise, _) => otherwise
                };
                next[index] = next_cell;
            }
        }
        self.cells = next;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let index = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[index] as u8;
            }
        }
        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}