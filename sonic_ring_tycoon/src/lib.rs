use std::time::Instant;
use std::time::Duration;

pub struct GameState {
    pub rings: u64,
    pub multiplier: u64,
    pub multiplier_upgrade_cost: u64,
    pub knuckles_num_collectors: u64,
    pub knuckles_collection_rate: u64,
    pub knuckles_upgrade_cost: u64,
    pub last_collect: Instant,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            rings: 0,
            multiplier: 1,
            multiplier_upgrade_cost: 10,
            knuckles_num_collectors: 0,
            knuckles_collection_rate: 1,
            knuckles_upgrade_cost: 10,
            last_collect: Instant::now(),
        }
    }
}

impl GameState {
    pub fn collect_ring(&mut self) {
        self.rings += self.multiplier;
    }

    pub fn update_passive_collection(&mut self, now: Instant) {
        let secs_elapsed = now.duration_since(self.last_collect).as_secs();
        if secs_elapsed > 0 {
            let collected = self.get_passive_rings_per_second() * secs_elapsed;
            self.rings += collected;
            self.last_collect += Duration::from_secs(secs_elapsed);
        }
    }

    pub fn increase_multiplier(&mut self) {
        if self.rings >= self.multiplier_upgrade_cost {
            self.rings -= self.multiplier_upgrade_cost;
            self.multiplier += 1;
            self.multiplier_upgrade_cost =
                (self.multiplier_upgrade_cost as f64 * 1.15).round() as u64;
            println!("Multiplier increased to {}", self.multiplier);
        } else {
            println!("Not enough rings to increase multiplier");
        }
    }

    pub fn increase_knuckles_collectors(&mut self) {
        if self.rings >= self.knuckles_upgrade_cost {
            self.rings -= self.knuckles_upgrade_cost;
            self.knuckles_num_collectors += 1;
            self.knuckles_upgrade_cost = (self.knuckles_upgrade_cost as f64 * 1.15).round() as u64;
            println!(
                "Knuckles collectors increased to {}",
                self.knuckles_num_collectors
            );
        } else {
            println!("Not enough rings to increase knuckles collectors");
        }
    }

    pub fn get_passive_rings_per_second(&self) -> u64 {
        self.get_knuckles_rings_per_second()
    }

    pub fn get_knuckles_rings_per_second(&self) -> u64 {
        self.knuckles_num_collectors * self.knuckles_collection_rate
    }
}
