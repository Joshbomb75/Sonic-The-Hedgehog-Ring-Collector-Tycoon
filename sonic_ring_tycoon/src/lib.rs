use std::time::Duration;
use std::time::Instant;

pub const CONST_GROWTH_FACTOR: f64 = 1.15;
pub const MULTIPLIER_BASE_COST: u64 = 10;
pub const KNUCKLES_BASE_COLLECTION_RATE: u64 = 1;
pub const KNUCKLES_BASE_ADD_COLLECTOR_COST: u64 = 15;
pub const KNUCKLES_BASE_COLLECTION_RATE_UPGRADE_COST: u64 = 75;
pub const CHILI_DOG_BASE_COLLECTION_RATE: u64 = 10;
pub const CHILI_DOG_BASE_ADD_COLLECTOR_COST: u64 = 100;
pub const CHILI_DOG_BASE_COLLECTION_RATE_UPGRADE_COST: u64 = 500;
pub const TAILS_BASE_COLLECTION_RATE: u64 = 100;
pub const TAILS_BASE_ADD_COLLECTOR_COST: u64 = 750;
pub const TAILS_BASE_COLLECTION_RATE_UPGRADE_COST: u64 = 7500;

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
    pub chili_dog_collection_rate_upgrade_cost: u64,
    pub tails_num_collectors: u64,
    pub tails_collection_rate: u64,
    pub tails_add_collector_cost: u64,
    pub tails_collection_rate_upgrade_cost: u64,
    pub last_collect: Instant,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            rings: 0,
            multiplier: 1,
            multiplier_increase_cost: MULTIPLIER_BASE_COST,
            knuckles_num_collectors: 0,
            knuckles_collection_rate: KNUCKLES_BASE_COLLECTION_RATE,
            knuckles_add_collector_cost: KNUCKLES_BASE_ADD_COLLECTOR_COST,
            knuckles_collection_rate_upgrade_cost: KNUCKLES_BASE_COLLECTION_RATE_UPGRADE_COST,
            chili_dog_num_collectors: 0,
            chili_dog_collection_rate: CHILI_DOG_BASE_COLLECTION_RATE,
            chili_dog_add_collector_cost: CHILI_DOG_BASE_ADD_COLLECTOR_COST,
            chili_dog_collection_rate_upgrade_cost: CHILI_DOG_BASE_COLLECTION_RATE_UPGRADE_COST,
            tails_num_collectors: 0,
            tails_collection_rate: TAILS_BASE_COLLECTION_RATE,
            tails_add_collector_cost: TAILS_BASE_ADD_COLLECTOR_COST,
            tails_collection_rate_upgrade_cost: TAILS_BASE_COLLECTION_RATE_UPGRADE_COST,
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
            self.knuckles_collection_rate += KNUCKLES_BASE_COLLECTION_RATE;
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

    pub fn increase_chili_dog_collection_rate(&mut self) {
        if self.rings >= self.chili_dog_collection_rate_upgrade_cost {
            self.rings -= self.chili_dog_collection_rate_upgrade_cost;
            self.chili_dog_collection_rate += CHILI_DOG_BASE_COLLECTION_RATE;
            self.chili_dog_collection_rate_upgrade_cost =
                (self.chili_dog_collection_rate_upgrade_cost as f64 * CONST_GROWTH_FACTOR).round()
                    as u64;
            println!(
                "Chili dog collection rate increased to {}",
                self.chili_dog_collection_rate
            );
        } else {
            println!("Not enough rings to increase chili dog collection rate");
        }
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

    pub fn chili_dog_collection_rate_upgrade_button_label(&self) -> String {
        format!(
            "Add a New Topping to Increase Collection Rate! ({}/{})",
            self.rings, self.chili_dog_collection_rate_upgrade_cost
        )
    }

    pub fn increase_tails_collectors(&mut self) {
        if self.rings >= self.tails_add_collector_cost {
            self.rings -= self.tails_add_collector_cost;
            self.tails_num_collectors += 1;
            self.tails_add_collector_cost =
                (self.tails_add_collector_cost as f64 * CONST_GROWTH_FACTOR).round() as u64;
            println!(
                "Tails collectors increased to {}",
                self.tails_num_collectors
            );
        } else {
            println!("Not enough rings to increase tails collectors");
        }
    }

    pub fn increase_tails_collection_rate(&mut self) {
        if self.rings >= self.tails_collection_rate_upgrade_cost {
            self.rings -= self.tails_collection_rate_upgrade_cost;
            self.tails_collection_rate += TAILS_BASE_COLLECTION_RATE;
            self.tails_collection_rate_upgrade_cost =
                (self.tails_collection_rate_upgrade_cost as f64 * CONST_GROWTH_FACTOR).round()
                    as u64;
            println!(
                "Tails collection rate increased to {}",
                self.tails_collection_rate
            );
        } else {
            println!("Not enough rings to increase tails collection rate");
        }
    }

    pub fn tails_button_label(&self) -> String {
        if self.tails_num_collectors == 0 {
            format!(
                "Ask Tails to Build a Flying Ring Magnet Drone! ({}/{})",
                self.rings, self.tails_add_collector_cost
            )
        } else {
            format!(
                "Ask Tails to Build Another Drone! ({}/{})",
                self.rings, self.tails_add_collector_cost
            )
        }
    }

    pub fn tails_collection_rate_upgrade_button_label(&self) -> String {
        format!(
            "Tails Upgrades the Drones to Increase Collection Rate! ({}/{})",
            self.rings, self.tails_collection_rate_upgrade_cost
        )
    }

    pub fn get_passive_rings_per_second(&self) -> u64 {
        self.get_knuckles_rings_per_second()
            + self.get_chili_dog_rings_per_second()
            + self.get_tails_rings_per_second()
    }

    pub fn get_knuckles_rings_per_second(&self) -> u64 {
        self.knuckles_num_collectors * self.knuckles_collection_rate
    }

    pub fn get_chili_dog_rings_per_second(&self) -> u64 {
        self.chili_dog_num_collectors * self.chili_dog_collection_rate
    }

    pub fn get_tails_rings_per_second(&self) -> u64 {
        self.tails_num_collectors * self.tails_collection_rate
    }
}
