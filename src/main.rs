use eframe::egui;
use eframe::epaint::TextureHandle;

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
    logo: Option<TextureHandle>, // Texture handle for the logo
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
            logo: None,
        }
    }
}

impl CostCalculator {
    fn calculate(&mut self) {
        let filament_cost_per_gram = self.filament_cost_per_kilo / 1000.0; // Convert kilo cost to per gram
        let filament_cost = filament_cost_per_gram * self.filament_weight; // Cost of the filament used
        let electricity_cost = (self.printer_wattage / 1000.0) * self.print_time * self.electricity_rate; // Electricity cost
        let wear_and_tear = self.print_time * 0.05; // Simplified wear and tear cost (EUR/hour)

        self.total_cost = filament_cost + electricity_cost + wear_and_tear + self.shipping_cost;
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

            // Input fields
            ui.horizontal(|ui| {
                ui.label("Filament cost per kilogram (EUR):");
                ui.add(egui::DragValue::new(&mut self.filament_cost_per_kilo).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Electricity rate (EUR/kWh):");
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
                ui.label("Shipping cost (EUR):");
                ui.add(egui::DragValue::new(&mut self.shipping_cost).speed(0.1));
            });

            // Calculate button
            if ui.button("Calculate").clicked() {
                self.calculate();
            }

            ui.separator();

            // Display results
            ui.label(format!("Total cost: {:.2} EUR", self.total_cost));
            ui.label(format!("Suggested price (with markup): {:.2} EUR", self.suggested_price));

            ui.separator();

            ui.label("Profit Estimates:");
            ui.label(format!("10% Profit: {:.2} EUR", self.profit_estimates[0]));
            ui.label(format!("20% Profit: {:.2} EUR", self.profit_estimates[1]));
            ui.label(format!("30% Profit: {:.2} EUR", self.profit_estimates[2]));
            ui.label(format!("50% Profit: {:.2} EUR", self.profit_estimates[3]));
            ui.label(format!("100% Profit: {:.2} EUR", self.profit_estimates[4]));
        });
    }
}

fn load_logo(cc: &eframe::CreationContext<'_>) -> Option<TextureHandle> {
    let bytes = include_bytes!("../assets/logo.png"); // Path to your logo file
    let image = image::load_from_memory(bytes).ok()?.to_rgba8();
    let size = [image.width() as _, image.height() as _];
    let pixels = image.into_raw();
    Some(cc.egui_ctx.load_texture(
        "logo",
        egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
        egui::TextureOptions::default(),
    ))
}
