use std::time::Duration;
use std::time::Instant;

pub struct GameState {
    pub rings: u64,
    pub multiplier: u64,
    pub multiplier_increase_cost: u64,
    pub knuckles_num_collectors: u64,
    pub knuckles_collection_rate: u64,
    pub knuckles_add_collector_cost: u64,
    pub knuckles_collection_rate_upgrade_cost: u64,
    pub chili_dog_num_collectors: u64,
    pub chili_dog_collection_rate: u64,
    pub chili_dog_add_collector_cost: u64,
    pub last_collect: Instant,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            rings: 0,
            multiplier: 1,
            multiplier_increase_cost: 10,
            knuckles_num_collectors: 0,
            knuckles_collection_rate: 1,
            knuckles_add_collector_cost: 10,
            knuckles_collection_rate_upgrade_cost: 100,
            chili_dog_num_collectors: 0,
            chili_dog_collection_rate: 10,
            chili_dog_add_collector_cost: 50,
            last_collect: Instant::now(),
        }
    }
}

const CONST_GROWTH_FACTOR: f64 = 1.15;

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
        if self.rings >= self.multiplier_increase_cost {
            self.rings -= self.multiplier_increase_cost;
            self.multiplier += 1;
            self.multiplier_increase_cost =
                (self.multiplier_increase_cost as f64 * CONST_GROWTH_FACTOR).round() as u64;
            println!("Multiplier increased to {}", self.multiplier);
        } else {
            println!("Not enough rings to increase multiplier");
        }
    }

    pub fn increase_knuckles_collectors(&mut self) {
        if self.rings >= self.knuckles_add_collector_cost {
            self.rings -= self.knuckles_add_collector_cost;
            self.knuckles_num_collectors += 1;
            self.knuckles_add_collector_cost =
                (self.knuckles_add_collector_cost as f64 * CONST_GROWTH_FACTOR).round() as u64;
            println!(
                "Knuckles collectors increased to {}",
                self.knuckles_num_collectors
            );
        } else {
            println!("Not enough rings to increase knuckles collectors");
        }
    }

    pub fn increase_knuckles_collection_rate(&mut self) {
        if self.rings >= self.knuckles_collection_rate_upgrade_cost {
            self.rings -= self.knuckles_collection_rate_upgrade_cost;
            self.knuckles_collection_rate += 1;
            self.knuckles_collection_rate_upgrade_cost =
                (self.knuckles_collection_rate_upgrade_cost as f64 * CONST_GROWTH_FACTOR).round()
                    as u64;
            println!(
                "Knuckles collection rate increased to {}",
                self.knuckles_collection_rate
            );
        } else {
            println!("Not enough rings to increase knuckles collection rate");
        }
    }

    pub fn increase_chili_dog_collectors(&mut self) {
        if self.rings >= self.chili_dog_add_collector_cost {
            self.rings -= self.chili_dog_add_collector_cost;
            self.chili_dog_num_collectors += 1;
            self.chili_dog_add_collector_cost =
                (self.chili_dog_add_collector_cost as f64 * CONST_GROWTH_FACTOR).round() as u64;
            println!(
                "Chili dog collectors increased to {}",
                self.chili_dog_num_collectors
            );
        } else {
            println!("Not enough rings to increase chili dog collectors");
        }
    }

    pub fn get_passive_rings_per_second(&self) -> u64 {
        self.get_knuckles_rings_per_second() + self.get_chili_dog_rings_per_second()
    }

    pub fn get_knuckles_rings_per_second(&self) -> u64 {
        self.knuckles_num_collectors * self.knuckles_collection_rate
    }

    pub fn get_chili_dog_rings_per_second(&self) -> u64 {
        self.chili_dog_num_collectors * self.chili_dog_collection_rate
    }

    pub fn knuckles_button_label(&self) -> String {
        if self.knuckles_num_collectors == 0 {
            format!(
                "Enlist Knuckles' Help to Dig for Rings! ({}/{})",
                self.rings, self.knuckles_add_collector_cost
            )
        } else {
            format!(
                "Motivate Knuckles to Dig for More Rings! ({}/{})",
                self.rings, self.knuckles_add_collector_cost
            )
        }
    }

    pub fn knuckles_collection_rate_upgrade_button_label(&self) -> String {
        format!(
            "Upgrade Knuckles' Gloves to Increase Collection Rate! ({}/{})",
            self.rings, self.knuckles_collection_rate_upgrade_cost
        )
    }

    pub fn chili_dog_button_label(&self) -> String {
        if self.chili_dog_num_collectors == 0 {
            format!(
                "Earn Rings with a Chili Dog Cart! ({}/{})",
                self.rings, self.chili_dog_add_collector_cost
            )
        } else {
            format!(
                "Open Another Chili Dog Cart! ({}/{})",
                self.rings, self.chili_dog_add_collector_cost
            )
        }
    }
}
