#[derive(Default)]
pub struct GameState {
    pub rings: u64,
}

impl GameState {
    pub fn new() -> Self {
        Self { rings: 0 }
    }

    pub fn tick(&mut self, dt: f64) {
        // simulate passive ring collection (to be expanded)
        self.rings += (dt * 0.1) as u64;
    }
}
