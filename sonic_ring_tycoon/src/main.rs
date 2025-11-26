use sonic_ring_tycoon::GameState;
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
            ui.heading("💍 Sonic Ring Tycoon 💍");
            ui.label(format!("Rings: {}", self.game.rings));
            ui.label(format!("Multiplier: {}", self.game.multiplier));
            let passive_rings_per_second = self.game.get_passive_rings_per_second();
            if passive_rings_per_second > 0 {
                ui.label(format!(
                    "Total Passive Rings per Second: {}",
                    passive_rings_per_second
                ));
            }
            if self.game.knuckles_num_collectors > 0 {
                ui.label(format!(
                    "Knuckles Rings per Second: {}",
                    self.game.get_knuckles_rings_per_second()
                ));
            }

            // Collect Ring button
            if ui.button("Collect Ring!").clicked() {
                self.game.collect_ring();
            }
            // Multiplier button
            if ui
                .button(format!(
                    "Increase Multiplier! ({}/{} rings)",
                    self.game.rings, self.game.multiplier_upgrade_cost
                ))
                .clicked()
            {
                self.game.increase_multiplier();
            }
            // Knuckles button (auto-collector)
            let knuckles_button_text = if self.game.knuckles_num_collectors == 0 {
                format!(
                    "Enlist Knuckles' Help to Dig for Rings! ({}/{} rings)",
                    self.game.rings, self.game.knuckles_upgrade_cost
                )
            } else {
                format!(
                    "Motivate Knuckles to Dig for More Rings! ({}/{} rings)",
                    self.game.rings, self.game.knuckles_upgrade_cost
                )
            };
            if ui.button(knuckles_button_text).clicked() {
                self.game.increase_knuckles_collectors();
            }
        });

        ctx.request_repaint(); // refresh loop
    }
}
