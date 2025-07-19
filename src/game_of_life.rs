use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::patterns::{ insert_pulsar, insert_lwss, insert_gosper_glider_gun, insert_random};

pub struct GameOfLife {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![false; height]; width];

        GameOfLife {
            width,
            height,
            cells,
        }
    }

    pub fn spawn_pulsar(&mut self, x: usize, y: usize){
        insert_pulsar(&mut self.cells, x, y);
    }

    pub fn spawn_lwss(&mut self, x: usize, y: usize) {
        insert_lwss(&mut self.cells, x, y);
    }
    
    pub fn spawn_gosper_glider_gun(&mut self, x: usize, y: usize) {
        insert_gosper_glider_gun(&mut self.cells, x, y);
    }

    pub fn spawn_random(&mut self, width: usize, height: usize, prob: f64) {
        insert_random(&mut self.cells, 0, 0, width, height, prob);
    }

    pub fn update(&mut self) {
        let mut next = vec![vec![false; self.height]; self.width];

        for x in 0..self.width {
            for y in 0..self.height {
                let live_neighbors = self.live_neighbor_count(x, y);
                next[x][y] = match (self.cells[x][y], live_neighbors) {
                    (true, 2) | (_, 3) => true,
                    _ => false,
                };
            }
        }

        self.cells = next;
    }

    fn live_neighbor_count(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for dx in [-1i32, 0, 1] {
            for dy in [-1i32, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    if self.cells[nx as usize][ny as usize] {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    pub fn draw(&self, fb: &mut Framebuffer, cell_size: u32) {
        fb.set_current_color(Color::WHITE);
        for x in 0..self.width {
            for y in 0..self.height {
                if self.cells[x][y] {
                    for dx in 0..cell_size {
                        for dy in 0..cell_size {
                            fb.set_pixel(
                                x as u32 * cell_size + dx,
                                y as u32 * cell_size + dy,
                            );
                        }
                    }
                }
            }
        }
    }
}
