// Suppress the terminal window on Windows release builds
#![cfg_attr(windows, windows_subsystem = "windows")]

mod filament_prices;
mod logic;
mod ui;

use crate::ui::{CalculatorUI, load_logo};

fn main() -> Result<(), eframe::Error> {
    // Configure native options for the application window
    let options = eframe::NativeOptions {
        vsync: true,               // Enable vertical sync for smoother rendering
        multisampling: 1,          // Default multisampling (anti-aliasing)
        depth_buffer: 0,           // Disable depth buffer (not needed for 2D applications)
        stencil_buffer: 0,         // Disable stencil buffer (not needed for 2D applications)
        ..Default::default()       // Use other default options
    };

    // Run the FDM Cost Calculator application
    eframe::run_native(
        "FDM Cost Calculator",     // Window title
        options,                   // Window and rendering options
        Box::new(|cc| {
            // Initialize the main application structure
            let mut app = CalculatorUI::default();

            // Load and assign the application logo
            app.logo = load_logo(cc);

            // Return the initialized application instance
            Ok(Box::new(app))
        }),
    )
}
