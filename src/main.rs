mod filament_prices;
mod logic;
mod ui;

use crate::ui::{CalculatorUI, load_logo};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "FDM Cost Calculator",
        options,
        Box::new(|cc| {
            let mut app = CalculatorUI::default();

            // Load the logo and assign it to the CalculatorUI instance
            app.logo = load_logo(cc);

            Ok(Box::new(app))
        }),
    )
}
