mod person;
mod settings;
mod simulation;

use crate::settings::*;
use eframe::egui;
use simulation::Simulation;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([SIMULATION_AREA_SIZE * 3.0, SIMULATION_AREA_SIZE * 3.0]),
        ..Default::default()
    };
    let result = eframe::run_native(
        "Epidemic Simulation - SIR Model",
        options,
        Box::new(|_cc| Ok(Box::new(Simulation::new()))),
    );

    match result {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Error: {}", e),
    }
}
