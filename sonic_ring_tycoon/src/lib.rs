pub struct GameState {
    pub rings: u64,
    pub multiplier: u64,
    pub multiplier_cost: u64,
    pub knuckles_collector_amount: u64,
    pub knuckles_collection_rate: u64,
    pub knuckles_cost: u64,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            rings: 0,
            multiplier: 1,
            multiplier_cost: 50,
            knuckles_collector_amount: 0,
            knuckles_collection_rate: 1,
            knuckles_cost: 30,
        }
    }
}

impl GameState {
    pub fn tick(&mut self, dt: f64) {
        // simulate passive ring collection (to be expanded)
        self.rings += (dt * 0.1) as u64;
    }
}
