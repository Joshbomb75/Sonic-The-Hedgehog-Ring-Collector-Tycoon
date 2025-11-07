use sonic_ring_tycoon::GameState;

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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("💍 Sonic Ring Tycoon 💍");
            ui.label(format!("Rings: {}", self.game.rings));
            ui.label(format!("Multiplier: {}", self.game.multiplier));

            if ui.button("Collect Ring!").clicked() {
                self.game.rings += self.game.multiplier;
            }
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
        });

        ctx.request_repaint(); // refresh loop
    }
}
