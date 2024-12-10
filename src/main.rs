use eframe::egui;

struct CostCalculator {
    filament_cost_per_kilo: f32, // Filament cost in EUR/kilogram
    electricity_rate: f32,       // Electricity rate in EUR/kWh
    printer_wattage: f32,        // Printer wattage in watts
    filament_weight: f32,        // Filament weight used for the print in grams
    print_time: f32,             // Print time in hours
    markup_percentage: f32,      // Markup percentage for profit
    total_cost: f32,             // Total cost (calculated)
    suggested_price: f32,        // Suggested price (calculated)
}

impl Default for CostCalculator {
    fn default() -> Self {
        Self {
            filament_cost_per_kilo: 25.0, // Default filament cost in EUR/kilo
            electricity_rate: 0.12,      // Default electricity rate in EUR/kWh
            printer_wattage: 250.0,      // Default printer wattage in watts
            filament_weight: 0.0,        // Default filament weight in grams
            print_time: 0.0,             // Default print time in hours
            markup_percentage: 20.0,     // Default markup percentage
            total_cost: 0.0,             // Default total cost
            suggested_price: 0.0,        // Default suggested price
        }
    }
}

impl CostCalculator {
    fn calculate(&mut self) {
        let filament_cost_per_gram = self.filament_cost_per_kilo / 1000.0; // Convert kilo cost to per gram
        let filament_cost = filament_cost_per_gram * self.filament_weight; // Cost of the filament used
        let electricity_cost = (self.printer_wattage / 1000.0) * self.print_time * self.electricity_rate; // Electricity cost
        let wear_and_tear = self.print_time * 0.05; // Simplified wear and tear cost (EUR/hour)

        self.total_cost = filament_cost + electricity_cost + wear_and_tear;
        self.suggested_price = self.total_cost * (1.0 + self.markup_percentage / 100.0);
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "FDM Cost Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(CostCalculator::default()))), // Corrected to return Result
    )
}

impl eframe::App for CostCalculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FDM Cost Calculator");

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

            // Calculate button
            if ui.button("Calculate").clicked() {
                self.calculate();
            }

            // Display results
            ui.separator();
            ui.label(format!("Total cost: {:.2} EUR", self.total_cost));
            ui.label(format!("Suggested price (with markup): {:.2} EUR", self.suggested_price));
        });
    }
}
