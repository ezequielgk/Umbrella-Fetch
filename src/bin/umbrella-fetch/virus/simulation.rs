//! Biological cellular automaton logic.

use super::strains::StrainProfile;

/// 2D grid containing the viral simulation state.
pub struct ViralGrid {
    pub grid:    Vec<bool>,
    pub age:     Vec<u8>,
    pub cols:    usize,
    pub rows:    usize,
    pub tick:    u64,
    pub strain:  &'static StrainProfile,
}

impl ViralGrid {
    /// Initializes a new seeded viral grid.
    pub fn new(cols: usize, rows: usize, strain: &'static StrainProfile) -> Self {
        let mut vg = Self {
            grid: vec![false; cols * rows],
            age: vec![0; cols * rows],
            cols,
            rows,
            tick: 0,
            strain,
        };
        vg.seed();
        vg
    }

    /// Clears the grid and plants an initial cluster in the center.
    pub fn seed(&mut self) {
        self.grid.fill(false);
        self.age.fill(0);
        self.tick = 0;
        let cx = self.cols / 2;
        let cy = self.rows / 2;
        let r = (self.strain.seed_radius * 4) as isize;
        
        for dy in -r..=r {
            for dx in -r..=r {
                if dx*dx + dy*dy <= r*r {
                    if rand::random::<f32>() < 0.5 {
                        let x = ((cx as isize + dx).rem_euclid(self.cols as isize)) as usize;
                        let y = ((cy as isize + dy).rem_euclid(self.rows as isize)) as usize;
                        let i = y * self.cols + x;
                        self.grid[i] = true;
                    }
                }
            }
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> u8 {
        let mut n = 0u8;
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = ((x as i32 + dx).rem_euclid(self.cols as i32)) as usize;
                let ny = ((y as i32 + dy).rem_euclid(self.rows as i32)) as usize;
                if self.grid[ny * self.cols + nx] {
                    n += 1;
                }
            }
        }
        n
    }

    /// Executes the simulation logic for the next generation.
    pub fn step(&mut self) {
        let s = self.strain;
        let mut next = vec![false; self.cols * self.rows];
        let mut changed = false;
        
        for y in 0..self.rows {
            for x in 0..self.cols {
                let i = y * self.cols + x;
                let n = self.neighbors(x, y);
                
                let mut alive = false;
                if self.grid[i] {
                    self.age[i] = self.age[i].saturating_add(1);
                    let should_die = self.age[i] > 12 && rand::random::<f32>() < s.decay_prob;
                    
                    if !should_die && n >= s.survive_min && n <= s.survive_max {
                        alive = true;
                    }
                } else {
                    if n >= s.birth_min && n <= s.birth_max && rand::random::<f32>() < s.spread_prob {
                        alive = true;
                        self.age[i] = 0;
                    }
                }
                
                next[i] = alive;
                if alive != self.grid[i] {
                    changed = true;
                }
            }
        }
        
        let alive_count = next.iter().filter(|&&c| c).count();
        if !changed || alive_count == 0 {
            self.seed();
            return;
        }
        
        let mutation_count = (self.cols * self.rows) / 250 + 1;
        for _ in 0..mutation_count {
            let rx = (rand::random::<u64>() as usize) % self.cols;
            let ry = (rand::random::<u64>() as usize) % self.rows;
            let ri = ry * self.cols + rx;
            next[ri] = true;
            self.age[ri] = 0;
        }
        
        self.grid = next;
        self.tick += 1;
        
        if self.tick % 300 == 0 {
            self.seed();
        }
    }

    pub fn alive_count(&self) -> usize {
        self.grid.iter().filter(|&&c| c).count()
    }

    pub fn spread_pct(&self) -> u8 {
        ((self.alive_count() * 100) / (self.cols * self.rows)) as u8
    }

    pub fn current_phase(&self) -> &'static str {
        let p = (self.spread_pct() / 20).min(4) as usize;
        self.strain.phases[p]
    }
}
