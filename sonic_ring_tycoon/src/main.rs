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

            if ui.button("Collect Ring!").clicked() {
                self.game.rings += 1;
            }
        });

        ctx.request_repaint(); // refresh loop
    }
}
