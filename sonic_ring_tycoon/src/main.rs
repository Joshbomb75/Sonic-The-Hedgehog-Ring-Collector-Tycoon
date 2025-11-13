use sonic_ring_tycoon::GameState;
use std::time::Instant;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Sonic Ring Tycoon",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    game: GameState,
    last_collect: Instant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            game: GameState::default(),
            last_collect: Instant::now(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle auto-collection timing
        let now = Instant::now();
        if now.duration_since(self.last_collect).as_secs_f32() >= 1.0 {
            let collected =
                self.game.knuckles_collector_amount * self.game.knuckles_collection_rate;
            self.game.rings += collected;
            self.last_collect = now;
        }

        // Draw the UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("💍 Sonic Ring Tycoon 💍");
            ui.label(format!("Rings: {}", self.game.rings));
            ui.label(format!("Multiplier: {}", self.game.multiplier));
            let passive_rings_per_second =
                self.game.knuckles_collector_amount * self.game.knuckles_collection_rate;
            if passive_rings_per_second > 0 {
                ui.label(format!(
                    "Total Passive Rings per Second: {}",
                    passive_rings_per_second
                ));
            }
            if self.game.knuckles_collector_amount > 0 {
                ui.label(format!(
                    "Knuckles Rings per Second: {}",
                    self.game.knuckles_collector_amount * self.game.knuckles_collection_rate
                ));
            }

            // Collect Ring button
            if ui.button("Collect Ring!").clicked() {
                self.game.rings += self.game.multiplier;
            }
            // Multiplier button
            if ui
                .button(format!(
                    "Increase Multiplier! ({}/{} rings)",
                    self.game.rings, self.game.multiplier_cost
                ))
                .clicked()
                && self.game.rings >= self.game.multiplier_cost
            {
                self.game.rings -= self.game.multiplier_cost;
                self.game.multiplier += 1;
                self.game.multiplier_cost += 10;
            }
            // Knuckles button (auto-collector)
            let knuckles_button_text = if self.game.knuckles_collector_amount == 0 {
                format!(
                    "Enlist Knuckles' Help to Dig for Rings! ({}/{} rings)",
                    self.game.rings, self.game.knuckles_cost
                )
            } else {
                format!(
                    "Increase Knuckles' Collection Rate! ({}/{} rings)",
                    self.game.rings, self.game.knuckles_cost
                )
            };
            if ui.button(knuckles_button_text).clicked()
                && self.game.rings >= self.game.knuckles_cost
            {
                self.game.rings -= self.game.knuckles_cost;
                self.game.knuckles_collector_amount += 1;
                self.game.knuckles_cost += 10;
            }
        });

        ctx.request_repaint(); // refresh loop
    }
}
