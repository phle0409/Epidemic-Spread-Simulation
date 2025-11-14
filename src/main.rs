mod person;
mod settings;
mod simulation;

use simulation::Simulation;

fn main() {
    let options = eframe::NativeOptions::default();
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
