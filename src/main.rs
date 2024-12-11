mod filament_prices;
mod logic;
mod ui;

use crate::ui::CalculatorUI;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("FDM Cost Calculator", options, Box::new(|_| Ok(Box::new(CalculatorUI::default()))))
}
