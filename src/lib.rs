use std::fmt;
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "C:/Users/AlviHysa/Desktop/dev/wasm-game-of-life/js/lib.js")]
extern "C" {
    fn clear_screen();
    fn draw_rect(x: usize, y: usize);
    fn set_fill(color: &str);
    fn draw_screen_outline(color: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

#[wasm_bindgen]
#[repr(C)]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    paused: bool,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: usize, height: usize) -> Universe {
        console_error_panic_hook::set_once();
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
            paused: false,
        }
    }

    pub fn render(&self) {
        clear_screen();

        set_fill("#000000");
        for (i, cell) in self.cells.iter().enumerate() {
            let x = i % self.width;
            let y = i / self.width;
            if *cell == Cell::Alive {
                draw_rect(x, y);
            }
        }
        if self.paused {
            draw_screen_outline("#FF0000");
        }
    }

    pub fn get_index(&self, col: usize, row: usize) -> usize {
        (row * self.width + col) as usize
    }

    pub fn live_neighbor_count(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_col, neighbor_row);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick_once(&mut self) {
        self.update()
    }

    pub fn tick(&mut self) {
        if self.paused {
            return;
        }
        self.update()
    }

    pub fn update(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(col, row);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        if !self.paused {
            return;
        }

        if (x >= self.width || x < 0) {
            return;
        }
        if (y >= self.height || y < 0) {
            return;
        }

        let idx = self.get_index(x, y);
        match self.cells[idx] {
            Cell::Dead => self.cells[idx] = Cell::Alive,
            Cell::Alive => self.cells[idx] = Cell::Dead,
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { ' ' } else { '@' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
