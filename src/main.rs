use eframe::egui;
use eframe::epaint::TextureHandle;

enum Currency {
    USD, // US Dollar
    EUR, // Euro
    GBP, // British Pound
}

struct CostCalculator {
    filament_cost_per_kilo: f32, // Filament cost in EUR/kilogram
    electricity_rate: f32,       // Electricity rate in EUR/kWh
    printer_wattage: f32,        // Printer wattage in watts
    filament_weight: f32,        // Filament weight used for the print in grams
    print_time: f32,             // Print time in hours
    markup_percentage: f32,      // Markup percentage for profit
    shipping_cost: f32,          // Additional shipping cost
    total_cost: f32,             // Total cost (calculated)
    suggested_price: f32,        // Suggested price (calculated)
    profit_estimates: Vec<f32>,  // Estimated prices for different profit margins
    wear_and_tear_cost: f32,     // Separate wear and tear cost (calculated)
    logo: Option<TextureHandle>, // Texture handle for the logo
    currency: Currency,          // Selected currency
}

impl Default for CostCalculator {
    fn default() -> Self {
        Self {
            filament_cost_per_kilo: 25.0,
            electricity_rate: 0.12,
            printer_wattage: 250.0,
            filament_weight: 0.0,
            print_time: 0.0,
            markup_percentage: 20.0,
            shipping_cost: 0.0,
            total_cost: 0.0,
            suggested_price: 0.0,
            profit_estimates: vec![0.0; 5],
            wear_and_tear_cost: 0.0,
            logo: None,
            currency: Currency::EUR, // Default to Euro
        }
    }
}

impl CostCalculator {
    fn calculate(&mut self) {
        let filament_cost_per_gram = self.filament_cost_per_kilo / 1000.0; // Convert kilo cost to per gram
        let filament_cost = filament_cost_per_gram * self.filament_weight; // Cost of the filament used
        let electricity_cost = (self.printer_wattage / 1000.0) * self.print_time * self.electricity_rate; // Electricity cost

        // Wear and tear calculation based on print time and average part costs
        let nozzle_cost_per_hour = 0.01; // Example: EUR 10 per nozzle, lasting ~1000 hours
        let belt_cost_per_hour = 0.005;  // Example: EUR 20 for a set of belts, lasting ~4000 hours
        let motor_cost_per_hour = 0.003; // Example: EUR 30 per motor, lasting ~10,000 hours
        let other_cost_per_hour = 0.002; // Miscellaneous parts wear

        self.wear_and_tear_cost = self.print_time
            * (nozzle_cost_per_hour + belt_cost_per_hour + motor_cost_per_hour + other_cost_per_hour);

        // Total cost excludes wear and tear for the main calculation
        self.total_cost = filament_cost + electricity_cost + self.shipping_cost;
        self.suggested_price = self.total_cost * (1.0 + self.markup_percentage / 100.0);

        // Calculate profit estimates
        self.profit_estimates = vec![
            self.total_cost * 1.10, // 10% profit
            self.total_cost * 1.20, // 20% profit
            self.total_cost * 1.30, // 30% profit
            self.total_cost * 1.50, // 50% profit
            self.total_cost * 2.00, // 100% profit
        ];
    }

    fn switch_currency(&mut self) {
        self.currency = match self.currency {
            Currency::USD => Currency::EUR,
            Currency::EUR => Currency::GBP,
            Currency::GBP => Currency::USD,
        };
    }

    fn currency_symbol(&self) -> &str {
        match self.currency {
            Currency::USD => "$",
            Currency::EUR => "€",
            Currency::GBP => "£",
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "FDM Cost Calculator",
        options,
        Box::new(|cc| {
            let mut app = CostCalculator::default();
            app.logo = load_logo(cc);

            Ok(Box::new(app))
        }),
    )
}

impl eframe::App for CostCalculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Main heading
                ui.heading("FDM Cost Calculator");

                // Place the logo in the top-right corner
                if let Some(logo) = &self.logo {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.image((logo.id(), egui::vec2(64.0, 64.0))); // Smaller logo for a modern look
                    });
                }
            });

            ui.separator();

            // Currency selector
            ui.horizontal(|ui| {
                ui.label("Currency:");
                if ui.button(self.currency_symbol()).clicked() {
                    self.switch_currency();
                }
            });

            ui.separator();

            // Input fields
            ui.horizontal(|ui| {
                ui.label("Filament cost per kilogram:");
                ui.add(egui::DragValue::new(&mut self.filament_cost_per_kilo).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Electricity rate:");
                ui.add(egui::DragValue::new(&mut self.electricity_rate).speed(0.01));
            });

            ui.horizontal(|ui| {
                ui.label("Printer wattage (W):");
                ui.add(egui::DragValue::new(&mut self.printer_wattage).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Filament weight (grams):");
                ui.add(egui::DragValue::new(&mut self.filament_weight).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Print time (hours):");
                ui.add(egui::DragValue::new(&mut self.print_time).speed(0.1));
            });

            ui.horizontal(|ui| {
                ui.label("Markup percentage (%):");
                ui.add(egui::DragValue::new(&mut self.markup_percentage).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Shipping cost:");
                ui.add(egui::DragValue::new(&mut self.shipping_cost).speed(0.1));
            });

            // Calculate button
            if ui.button("Calculate").clicked() {
                self.calculate();
            }

            ui.separator();

            // Display results
            ui.label(format!(
                "Total cost: {:.2} {}",
                self.total_cost,
                self.currency_symbol()
            ));
            ui.label(format!(
                "Suggested price (with markup): {:.2} {}",
                self.suggested_price,
                self.currency_symbol()
            ));

            ui.separator();

            ui.label("Profit Estimates:");
            for (i, estimate) in self.profit_estimates.iter().enumerate() {
                ui.label(format!(
                    "{}% Profit: {:.2} {}",
                    (i + 1) * 10,
                    estimate,
                    self.currency_symbol()
                ));
            }

            ui.separator();

            // Display wear and tear as a separate line
            ui.label(format!(
                "Wear and tear cost: {:.2} {}",
                self.wear_and_tear_cost,
                self.currency_symbol()
            ));
        });
    }
}

fn load_logo(cc: &eframe::CreationContext<'_>) -> Option<TextureHandle> {
    let bytes = include_bytes!("../assets/logo.png"); // Path to your in-app logo file
    let image = image::load_from_memory(bytes).ok()?.to_rgba8();
    let size = [image.width() as _, image.height() as _];
    let pixels = image.into_raw();
    Some(cc.egui_ctx.load_texture(
        "logo",
        egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
        egui::TextureOptions::default(),
    ))
}
