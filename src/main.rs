mod filament_prices;
mod logic;
mod ui;

use crate::ui::{CalculatorUI, load_logo};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        vsync: true,               // Enable vertical sync
        multisampling: 1,          // Use default multisampling (anti-aliasing)
        depth_buffer: 0,           // Disable depth buffer (2D app)
        stencil_buffer: 0,         // Disable stencil buffer (2D app)
        ..Default::default()       // Use other default options
    };

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
