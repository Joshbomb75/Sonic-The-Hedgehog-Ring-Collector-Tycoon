use sonic_ring_tycoon::{CHILI_DOG_BASE_COLLECTION_RATE, GameState, KNUCKLES_BASE_COLLECTION_RATE};
use std::time::Instant;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Sonic Ring Tycoon",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    game: GameState,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle auto-collection timing
        self.game.update_passive_collection(Instant::now());

        // Draw the UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("💍 Sonic Ring Tycoon 💍"); // heading
            ui.label(format!("Rings: {}", self.game.rings)); // rings
            // collect ring button multiplier
            if self.game.multiplier > 1 {
                ui.label(format!("Multiplier: {}", self.game.multiplier));
            }
            let passive_rings_per_second = self.game.get_passive_rings_per_second(); // passive rings per second
            if passive_rings_per_second > 0 {
                ui.label(format!(
                    "Total Passive Rings per Second: {}",
                    passive_rings_per_second
                ));
                // knuckles rings per second
                if self.game.knuckles_num_collectors > 0 {
                    let knuckles_rings_per_second = self.game.get_knuckles_rings_per_second();
                    ui.label(format!(
                        "Knuckles Rings per Second: {} ({:.1}% rps)",
                        knuckles_rings_per_second,
                        knuckles_rings_per_second as f64 / passive_rings_per_second as f64 * 100.0
                    ));
                }
                // chili dog rings per second
                if self.game.chili_dog_num_collectors > 0 {
                    let chili_dog_rings_per_second = self.game.get_chili_dog_rings_per_second();
                    ui.label(format!(
                        "Chili Dog Cart Rings per Second: {} ({:.1}% rps)",
                        chili_dog_rings_per_second,
                        chili_dog_rings_per_second as f64 / passive_rings_per_second as f64 * 100.0
                    ));
                }
                // tails rings per second
                if self.game.tails_num_collectors > 0 {
                    let tails_rings_per_second = self.game.get_tails_rings_per_second();
                    ui.label(format!(
                        "Tails' Ring Magnet Drone Rings per Second: {} ({:.1}% rps)",
                        tails_rings_per_second,
                        tails_rings_per_second as f64 / passive_rings_per_second as f64 * 100.0
                    ));
                }
            }

            // Collect Ring button and Multiplier button side by side
            ui.horizontal(|ui| {
                // Collect Ring button
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Collect Ring!").size(16.0),
                    ))
                    .clicked()
                {
                    self.game.collect_ring();
                }
                // Multiplier button
                if ui
                    .button(format!(
                        "Increase Multiplier! ({}/{} rings)",
                        self.game.rings, self.game.multiplier_increase_cost
                    ))
                    .clicked()
                {
                    self.game.increase_multiplier();
                }
            });
            // Knuckles button and upgrade button side by side
            ui.horizontal(|ui| {
                // Knuckles button (auto-collector)
                let knuckles_button_text = self.game.knuckles_button_label();
                if self.game.multiplier > 1 && ui.button(knuckles_button_text).clicked() {
                    self.game.increase_knuckles_collectors();
                }
                // Knuckles upgrade button
                if self.game.knuckles_num_collectors > 0
                    && ui
                        .button(self.game.knuckles_collection_rate_upgrade_button_label())
                        .clicked()
                {
                    self.game.increase_knuckles_collection_rate();
                }
            });
            // Chili Dog button and upgrade button side by side
            ui.horizontal(|ui| {
                // Chili Dog button
                let chili_dog_button_text = self.game.chili_dog_button_label();
                if self.game.knuckles_collection_rate > KNUCKLES_BASE_COLLECTION_RATE
                    && ui.button(chili_dog_button_text).clicked()
                {
                    self.game.increase_chili_dog_collectors();
                }
                // Chili Dog upgrade button
                if self.game.chili_dog_num_collectors > 0
                    && ui
                        .button(self.game.chili_dog_collection_rate_upgrade_button_label())
                        .clicked()
                {
                    self.game.increase_chili_dog_collection_rate();
                }
            });
            // Tails button and upgrade button side by side
            ui.horizontal(|ui| {
                // Tails button
                let tails_button_text = self.game.tails_button_label();
                if self.game.chili_dog_collection_rate > CHILI_DOG_BASE_COLLECTION_RATE
                    && ui.button(tails_button_text).clicked()
                {
                    self.game.increase_tails_collectors();
                }
                // Tails upgrade button
                if self.game.tails_num_collectors > 0
                    && ui
                        .button(self.game.tails_collection_rate_upgrade_button_label())
                        .clicked()
                {
                    self.game.increase_tails_collection_rate();
                }
            });
        });

        ctx.request_repaint(); // refresh loop
    }
}
