use rand::Rng;

pub fn insert_random(
    grid: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    probability: f64,
) {
    let mut rng = rand::rng();

    for dy in 0..height {
        for dx in 0..width {
            let gx = x + dx;
            let gy = y + dy;

            if gy < grid.len() {
                if let Some(cell) = grid[gy].get_mut(gx) {
                    *cell = rng.random::<f64>() < probability;
                }
            }
        }
    }
}

pub fn insert_pulsar(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    let pattern = [
        (2, 0), (3, 0), (4, 0),
        (0, 2), (0, 3), (0, 4),
        (5, 2), (5, 3), (5, 4),
        (2, 5), (3, 5), (4, 5),

        (2 + 6, 0), (3 + 6, 0), (4 + 6, 0),
        (0, 2 + 6), (0, 3 + 6), (0, 4 + 6),
        (5 + 6, 2), (5 + 6, 3), (5 + 6, 4),
        (2, 5 + 6), (3, 5 + 6), (4, 5 + 6),

        (2 + 6, 5 + 6), (3 + 6, 5 + 6), (4 + 6, 5 + 6),
        (5 + 6, 2 + 6), (5 + 6, 3 + 6), (5 + 6, 4 + 6),
        (0 + 6, 2 + 6), (0 + 6, 3 + 6), (0 + 6, 4 + 6),
        (2 + 6, 0 + 6), (3 + 6, 0 + 6), (4 + 6, 0 + 6),
    ];

    for (dx, dy) in pattern {
        if let Some(row) = grid.get_mut(y + dy) {
            if let Some(cell) = row.get_mut(x + dx) {
                *cell = true;
            }
        }
    }
}

pub fn insert_lwss(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    let pattern = [
        (1, 0), (2, 0), (3, 0), (4, 0),
        (0, 1), (4, 1),
        (4, 2),
        (0, 3), (3, 3),
    ];

    for (dx, dy) in pattern {
        if let Some(row) = grid.get_mut(y + dy) {
            if let Some(cell) = row.get_mut(x + dx) {
                *cell = true;
            }
        }
    }
}

pub fn insert_gosper_glider_gun(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    let pattern = [
        (1, 5), (1, 6), (2, 5), (2, 6),
        (11, 5), (11, 6), (11, 7),
        (12, 4), (12, 8),
        (13, 3), (13, 9),
        (14, 3), (14, 9),
        (15, 6),
        (16, 4), (16, 8),
        (17, 5), (17, 6), (17, 7),
        (18, 6),
        (21, 3), (21, 4), (21, 5),
        (22, 3), (22, 4), (22, 5),
        (23, 2), (23, 6),
        (25, 1), (25, 2), (25, 6), (25, 7),
        (35, 3), (35, 4),
        (36, 3), (36, 4),
    ];

    for (dx, dy) in pattern {
        if let Some(row) = grid.get_mut(y + dy) {
            if let Some(cell) = row.get_mut(x + dx) {
                *cell = true;
            }
        }
    }
}

