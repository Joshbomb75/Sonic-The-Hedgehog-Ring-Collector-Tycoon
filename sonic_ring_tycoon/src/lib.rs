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
pub const CHAOS_EMERALD_BASE_COST: u64 = 1000000;
pub const CHAOS_EMERALD_MULTIPLIER: u64 = 2;
pub const CHAOS_EMERALD_INCREASE_COST_GROWTH_FACTOR: f64 = 1.9;

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
    pub chaos_emerald_count: u32,
    pub chaos_emerald_cost: u64,
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
            chaos_emerald_count: 0,
            chaos_emerald_cost: CHAOS_EMERALD_BASE_COST,
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

    pub fn increase_chaos_emerald_count(&mut self) {
        if self.rings >= self.chaos_emerald_cost {
            self.rings -= self.chaos_emerald_cost;
            self.chaos_emerald_count += 1;
            self.chaos_emerald_cost = (self.chaos_emerald_cost as f64
                * CHAOS_EMERALD_INCREASE_COST_GROWTH_FACTOR)
                .round() as u64;
            println!(
                "Chaos emerald count increased to {}",
                self.chaos_emerald_count
            );
        } else {
            println!("Not enough rings to increase chaos emerald count");
        }
    }

    pub fn chaos_emerald_button_label(&self) -> String {
        format!(
            "Obtain a Chaos Emerald (Double All Collection Rates)! ({}/{})",
            self.rings, self.chaos_emerald_cost
        )
    }

    pub fn get_passive_rings_per_second(&self) -> u64 {
        self.get_knuckles_rings_per_second()
            + self.get_chili_dog_rings_per_second()
            + self.get_tails_rings_per_second()
    }

    pub fn get_knuckles_rings_per_second(&self) -> u64 {
        self.knuckles_num_collectors
            * self.knuckles_collection_rate
            * (CHAOS_EMERALD_MULTIPLIER.pow(self.chaos_emerald_count))
    }

    pub fn get_chili_dog_rings_per_second(&self) -> u64 {
        self.chili_dog_num_collectors
            * self.chili_dog_collection_rate
            * (CHAOS_EMERALD_MULTIPLIER.pow(self.chaos_emerald_count))
    }

    pub fn get_tails_rings_per_second(&self) -> u64 {
        self.tails_num_collectors
            * self.tails_collection_rate
            * (CHAOS_EMERALD_MULTIPLIER.pow(self.chaos_emerald_count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_default_game_state() {
        let state = GameState::default();
        assert_eq!(state.rings, 0);
        assert_eq!(state.multiplier, 1);
        assert_eq!(state.multiplier_increase_cost, MULTIPLIER_BASE_COST);
        assert_eq!(state.knuckles_num_collectors, 0);
        assert_eq!(
            state.knuckles_collection_rate,
            KNUCKLES_BASE_COLLECTION_RATE
        );
        assert_eq!(
            state.knuckles_add_collector_cost,
            KNUCKLES_BASE_ADD_COLLECTOR_COST
        );
        assert_eq!(
            state.knuckles_collection_rate_upgrade_cost,
            KNUCKLES_BASE_COLLECTION_RATE_UPGRADE_COST
        );
        assert_eq!(state.chili_dog_num_collectors, 0);
        assert_eq!(
            state.chili_dog_collection_rate,
            CHILI_DOG_BASE_COLLECTION_RATE
        );
        assert_eq!(
            state.chili_dog_add_collector_cost,
            CHILI_DOG_BASE_ADD_COLLECTOR_COST
        );
        assert_eq!(
            state.chili_dog_collection_rate_upgrade_cost,
            CHILI_DOG_BASE_COLLECTION_RATE_UPGRADE_COST
        );
        assert_eq!(state.tails_num_collectors, 0);
        assert_eq!(state.tails_collection_rate, TAILS_BASE_COLLECTION_RATE);
        assert_eq!(
            state.tails_add_collector_cost,
            TAILS_BASE_ADD_COLLECTOR_COST
        );
        assert_eq!(
            state.tails_collection_rate_upgrade_cost,
            TAILS_BASE_COLLECTION_RATE_UPGRADE_COST
        );
        assert_eq!(state.chaos_emerald_count, 0);
        assert_eq!(state.chaos_emerald_cost, CHAOS_EMERALD_BASE_COST);
    }

    #[test]
    fn test_collect_ring() {
        let mut state = GameState::default();
        state.collect_ring();
        assert_eq!(state.rings, 1);

        state.multiplier = 5;
        state.collect_ring();
        assert_eq!(state.rings, 6);
    }

    #[test]
    fn test_update_passive_collection() {
        let mut state = GameState::default();
        state.knuckles_num_collectors = 2;
        state.knuckles_collection_rate = 10;
        state.last_collect = Instant::now() - Duration::from_secs(5);

        let initial_rings = state.rings;
        let now = Instant::now();
        state.update_passive_collection(now);

        // Should collect: 2 collectors * 10 rate * 5 seconds = 100 rings
        assert_eq!(state.rings, initial_rings + 100);
    }

    #[test]
    fn test_update_passive_collection_no_time_elapsed() {
        let mut state = GameState::default();
        state.knuckles_num_collectors = 2;
        let initial_rings = state.rings;
        let now = Instant::now();
        state.last_collect = now;

        state.update_passive_collection(now);
        assert_eq!(state.rings, initial_rings);
    }

    #[test]
    fn test_increase_multiplier_sufficient_rings() {
        let mut state = GameState::default();
        state.rings = MULTIPLIER_BASE_COST;
        let initial_cost = state.multiplier_increase_cost;

        state.increase_multiplier();

        assert_eq!(state.multiplier, 2);
        assert_eq!(state.rings, 0);
        assert!(state.multiplier_increase_cost > initial_cost);
    }

    #[test]
    fn test_increase_multiplier_insufficient_rings() {
        let mut state = GameState::default();
        state.rings = MULTIPLIER_BASE_COST - 1;
        let initial_multiplier = state.multiplier;
        let initial_cost = state.multiplier_increase_cost;

        state.increase_multiplier();

        assert_eq!(state.multiplier, initial_multiplier);
        assert_eq!(state.rings, MULTIPLIER_BASE_COST - 1);
        assert_eq!(state.multiplier_increase_cost, initial_cost);
    }

    #[test]
    fn test_increase_knuckles_collectors() {
        let mut state = GameState::default();
        state.rings = KNUCKLES_BASE_ADD_COLLECTOR_COST;
        let initial_cost = state.knuckles_add_collector_cost;

        state.increase_knuckles_collectors();

        assert_eq!(state.knuckles_num_collectors, 1);
        assert_eq!(state.rings, 0);
        assert!(state.knuckles_add_collector_cost > initial_cost);
    }

    #[test]
    fn test_increase_knuckles_collectors_insufficient_rings() {
        let mut state = GameState::default();
        state.rings = KNUCKLES_BASE_ADD_COLLECTOR_COST - 1;
        let initial_count = state.knuckles_num_collectors;

        state.increase_knuckles_collectors();

        assert_eq!(state.knuckles_num_collectors, initial_count);
        assert_eq!(state.rings, KNUCKLES_BASE_ADD_COLLECTOR_COST - 1);
    }

    #[test]
    fn test_increase_knuckles_collection_rate() {
        let mut state = GameState::default();
        state.rings = KNUCKLES_BASE_COLLECTION_RATE_UPGRADE_COST;
        let initial_rate = state.knuckles_collection_rate;
        let initial_cost = state.knuckles_collection_rate_upgrade_cost;

        state.increase_knuckles_collection_rate();

        assert_eq!(
            state.knuckles_collection_rate,
            initial_rate + KNUCKLES_BASE_COLLECTION_RATE
        );
        assert_eq!(state.rings, 0);
        assert!(state.knuckles_collection_rate_upgrade_cost > initial_cost);
    }

    #[test]
    fn test_knuckles_button_label_no_collectors() {
        let state = GameState::default();
        let label = state.knuckles_button_label();
        assert!(label.contains("Enlist Knuckles' Help"));
        assert!(label.contains(&state.rings.to_string()));
        assert!(label.contains(&state.knuckles_add_collector_cost.to_string()));
    }

    #[test]
    fn test_knuckles_button_label_with_collectors() {
        let mut state = GameState::default();
        state.knuckles_num_collectors = 1;
        let label = state.knuckles_button_label();
        assert!(label.contains("Motivate Knuckles"));
        assert!(label.contains(&state.rings.to_string()));
        assert!(label.contains(&state.knuckles_add_collector_cost.to_string()));
    }

    #[test]
    fn test_knuckles_collection_rate_upgrade_button_label() {
        let state = GameState::default();
        let label = state.knuckles_collection_rate_upgrade_button_label();
        assert!(label.contains("Upgrade Knuckles' Gloves"));
        assert!(label.contains(&state.rings.to_string()));
        assert!(label.contains(&state.knuckles_collection_rate_upgrade_cost.to_string()));
    }

    #[test]
    fn test_increase_chili_dog_collectors() {
        let mut state = GameState::default();
        state.rings = CHILI_DOG_BASE_ADD_COLLECTOR_COST;
        let initial_cost = state.chili_dog_add_collector_cost;

        state.increase_chili_dog_collectors();

        assert_eq!(state.chili_dog_num_collectors, 1);
        assert_eq!(state.rings, 0);
        assert!(state.chili_dog_add_collector_cost > initial_cost);
    }

    #[test]
    fn test_increase_chili_dog_collection_rate() {
        let mut state = GameState::default();
        state.rings = CHILI_DOG_BASE_COLLECTION_RATE_UPGRADE_COST;
        let initial_rate = state.chili_dog_collection_rate;
        let initial_cost = state.chili_dog_collection_rate_upgrade_cost;

        state.increase_chili_dog_collection_rate();

        assert_eq!(
            state.chili_dog_collection_rate,
            initial_rate + CHILI_DOG_BASE_COLLECTION_RATE
        );
        assert_eq!(state.rings, 0);
        assert!(state.chili_dog_collection_rate_upgrade_cost > initial_cost);
    }

    #[test]
    fn test_chili_dog_button_label_no_collectors() {
        let state = GameState::default();
        let label = state.chili_dog_button_label();
        assert!(label.contains("Earn Rings with a Chili Dog Cart"));
        assert!(label.contains(&state.rings.to_string()));
        assert!(label.contains(&state.chili_dog_add_collector_cost.to_string()));
    }

    #[test]
    fn test_chili_dog_button_label_with_collectors() {
        let mut state = GameState::default();
        state.chili_dog_num_collectors = 1;
        let label = state.chili_dog_button_label();
        assert!(label.contains("Open Another Chili Dog Cart"));
        assert!(label.contains(&state.rings.to_string()));
        assert!(label.contains(&state.chili_dog_add_collector_cost.to_string()));
    }

    #[test]
    fn test_chili_dog_collection_rate_upgrade_button_label() {
        let state = GameState::default();
        let label = state.chili_dog_collection_rate_upgrade_button_label();
        assert!(label.contains("Add a New Topping"));
        assert!(label.contains(&state.rings.to_string()));
        assert!(label.contains(&state.chili_dog_collection_rate_upgrade_cost.to_string()));
    }

    #[test]
    fn test_increase_tails_collectors() {
        let mut state = GameState::default();
        state.rings = TAILS_BASE_ADD_COLLECTOR_COST;
        let initial_cost = state.tails_add_collector_cost;

        state.increase_tails_collectors();

        assert_eq!(state.tails_num_collectors, 1);
        assert_eq!(state.rings, 0);
        assert!(state.tails_add_collector_cost > initial_cost);
    }

    #[test]
    fn test_increase_tails_collection_rate() {
        let mut state = GameState::default();
        state.rings = TAILS_BASE_COLLECTION_RATE_UPGRADE_COST;
        let initial_rate = state.tails_collection_rate;
        let initial_cost = state.tails_collection_rate_upgrade_cost;

        state.increase_tails_collection_rate();

        assert_eq!(
            state.tails_collection_rate,
            initial_rate + TAILS_BASE_COLLECTION_RATE
        );
        assert_eq!(state.rings, 0);
        assert!(state.tails_collection_rate_upgrade_cost > initial_cost);
    }

    #[test]
    fn test_tails_button_label_no_collectors() {
        let state = GameState::default();
        let label = state.tails_button_label();
        assert!(label.contains("Ask Tails to Build a Flying Ring Magnet Drone"));
        assert!(label.contains(&state.rings.to_string()));
        assert!(label.contains(&state.tails_add_collector_cost.to_string()));
    }

    #[test]
    fn test_tails_button_label_with_collectors() {
        let mut state = GameState::default();
        state.tails_num_collectors = 1;
        let label = state.tails_button_label();
        assert!(label.contains("Ask Tails to Build Another Drone"));
        assert!(label.contains(&state.rings.to_string()));
        assert!(label.contains(&state.tails_add_collector_cost.to_string()));
    }

    #[test]
    fn test_tails_collection_rate_upgrade_button_label() {
        let state = GameState::default();
        let label = state.tails_collection_rate_upgrade_button_label();
        assert!(label.contains("Tails Upgrades the Drones"));
        assert!(label.contains(&state.rings.to_string()));
        assert!(label.contains(&state.tails_collection_rate_upgrade_cost.to_string()));
    }

    #[test]
    fn test_increase_chaos_emerald_count() {
        let mut state = GameState::default();
        state.rings = CHAOS_EMERALD_BASE_COST;
        let initial_cost = state.chaos_emerald_cost;

        state.increase_chaos_emerald_count();

        assert_eq!(state.chaos_emerald_count, 1);
        assert_eq!(state.rings, 0);
        assert!(state.chaos_emerald_cost > initial_cost);
    }

    #[test]
    fn test_increase_chaos_emerald_count_insufficient_rings() {
        let mut state = GameState::default();
        state.rings = CHAOS_EMERALD_BASE_COST - 1;
        let initial_count = state.chaos_emerald_count;

        state.increase_chaos_emerald_count();

        assert_eq!(state.chaos_emerald_count, initial_count);
        assert_eq!(state.rings, CHAOS_EMERALD_BASE_COST - 1);
    }

    #[test]
    fn test_chaos_emerald_button_label() {
        let state = GameState::default();
        let label = state.chaos_emerald_button_label();
        assert!(label.contains("Obtain a Chaos Emerald"));
        assert!(label.contains(&state.rings.to_string()));
        assert!(label.contains(&state.chaos_emerald_cost.to_string()));
    }

    #[test]
    fn test_get_knuckles_rings_per_second() {
        let mut state = GameState::default();
        state.knuckles_num_collectors = 3;
        state.knuckles_collection_rate = 5;
        state.chaos_emerald_count = 0;

        assert_eq!(state.get_knuckles_rings_per_second(), 3 * 5 * 1);

        state.chaos_emerald_count = 1;
        assert_eq!(state.get_knuckles_rings_per_second(), 3 * 5 * 2);

        state.chaos_emerald_count = 2;
        assert_eq!(state.get_knuckles_rings_per_second(), 3 * 5 * 4);
    }

    #[test]
    fn test_get_chili_dog_rings_per_second() {
        let mut state = GameState::default();
        state.chili_dog_num_collectors = 2;
        state.chili_dog_collection_rate = 20;
        state.chaos_emerald_count = 0;

        assert_eq!(state.get_chili_dog_rings_per_second(), 2 * 20 * 1);

        state.chaos_emerald_count = 1;
        assert_eq!(state.get_chili_dog_rings_per_second(), 2 * 20 * 2);
    }

    #[test]
    fn test_get_tails_rings_per_second() {
        let mut state = GameState::default();
        state.tails_num_collectors = 4;
        state.tails_collection_rate = 50;
        state.chaos_emerald_count = 0;

        assert_eq!(state.get_tails_rings_per_second(), 4 * 50 * 1);

        state.chaos_emerald_count = 1;
        assert_eq!(state.get_tails_rings_per_second(), 4 * 50 * 2);
    }

    #[test]
    fn test_get_passive_rings_per_second() {
        let mut state = GameState::default();
        state.knuckles_num_collectors = 1;
        state.knuckles_collection_rate = 10;
        state.chili_dog_num_collectors = 2;
        state.chili_dog_collection_rate = 20;
        state.tails_num_collectors = 3;
        state.tails_collection_rate = 30;
        state.chaos_emerald_count = 0;

        let expected = (1 * 10) + (2 * 20) + (3 * 30);
        assert_eq!(state.get_passive_rings_per_second(), expected);

        state.chaos_emerald_count = 1;
        let expected_with_emerald = expected * 2;
        assert_eq!(state.get_passive_rings_per_second(), expected_with_emerald);
    }

    #[test]
    fn test_multiple_upgrades_cost_growth() {
        let mut state = GameState::default();
        state.rings = 10000;

        let first_cost = state.multiplier_increase_cost;
        state.increase_multiplier();
        let second_cost = state.multiplier_increase_cost;

        // Cost should increase by growth factor (accounting for rounding)
        assert!(second_cost > first_cost);
        let expected_cost = (first_cost as f64 * CONST_GROWTH_FACTOR).round() as u64;
        assert_eq!(second_cost, expected_cost);
    }

    #[test]
    fn test_chaos_emerald_multiplier_effect() {
        let mut state = GameState::default();
        state.knuckles_num_collectors = 1;
        state.knuckles_collection_rate = 10;

        let base_rate = state.get_knuckles_rings_per_second();
        assert_eq!(base_rate, 10);

        state.rings = CHAOS_EMERALD_BASE_COST;
        state.increase_chaos_emerald_count();

        let doubled_rate = state.get_knuckles_rings_per_second();
        assert_eq!(doubled_rate, 20);

        // Get another emerald
        state.rings = state.chaos_emerald_cost;
        state.increase_chaos_emerald_count();

        let quadrupled_rate = state.get_knuckles_rings_per_second();
        assert_eq!(quadrupled_rate, 40);
    }
}
