use eframe::egui;
use eframe::epaint::TextureHandle;
use crate::logic::{CalculatorLogic, Currency};

pub struct CalculatorUI {
    pub logic: CalculatorLogic,
    pub logo: Option<TextureHandle>, // Texture handle for the logo
    pub show_help: bool,             // Whether to show the help dialog
}

impl Default for CalculatorUI {
    fn default() -> Self {
        Self {
            logic: CalculatorLogic::default(),
            logo: None,
            show_help: false,
        }
    }
}

impl eframe::App for CalculatorUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("FDM Cost Calculator");

                // Currency switcher button
                if ui
                    .button(format!(
                        "Switch to {}",
                        match self.logic.currency {
                            Currency::USD => "€", // EUR symbol
                            Currency::EUR => "£", // GBP symbol
                            Currency::GBP => "$", // USD symbol
                        }
                    ))
                    .clicked()
                {
                    self.logic.switch_currency();
                }

                // Place the help button and logo together
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Logo
                    if let Some(logo) = &self.logo {
                        ui.image((logo.id(), egui::vec2(64.0, 64.0)));
                    }

                    // Larger help button with styled text
                    let help_button = egui::Button::new(egui::RichText::new("?").size(18.0))
                        .min_size(egui::vec2(32.0, 32.0));
                    if ui.add(help_button).clicked() {
                        self.show_help = true;
                    }
                });
            });

            ui.separator();

            // Brand dropdown
            ui.horizontal(|ui| {
                ui.label("Brand:");
                let brands: Vec<_> = self.logic.filament_prices.keys().cloned().collect();
                egui::ComboBox::new("brand_selector", "Select a Brand")
                    .selected_text(self.logic.selected_brand.as_deref().unwrap_or("Select Brand").to_string())
                    .show_ui(ui, |ui| {
                        for brand in brands {
                            if ui
                                .selectable_label(self.logic.selected_brand.as_deref() == Some(brand), brand)
                                .clicked()
                            {
                                self.logic.selected_brand = Some(brand.to_string());
                                self.logic.update_filament_price();
                            }
                        }
                    });
            });

            // Material dropdown
            ui.horizontal(|ui| {
                ui.label("Material:");
                if let Some(brand) = &self.logic.selected_brand {
                    if let Some(materials) = self.logic.filament_prices.get(brand.as_str()) {
                        let material_keys: Vec<_> = materials.keys().cloned().collect();
                        egui::ComboBox::new("material_selector", "Select a Material")
                            .selected_text(self.logic.selected_material.as_deref().unwrap_or("Select Material").to_string())
                            .show_ui(ui, |ui| {
                                for material in material_keys {
                                    if ui
                                        .selectable_label(self.logic.selected_material.as_deref() == Some(material), material)
                                        .clicked()
                                    {
                                        self.logic.selected_material = Some(material.to_string());
                                        self.logic.update_filament_price();
                                    }
                                }
                            });
                    }
                }
            });

            // Carbon-based checkbox
            ui.horizontal(|ui| {
                ui.label("Carbon-Based:");
                if let Some(material) = &self.logic.selected_material {
                    if ["PLA", "PETG", "ABS", "ASA"].contains(&material.as_str()) {
                        ui.checkbox(&mut self.logic.is_carbon_based, "Add carbon-based cost");
                        self.logic.update_filament_price();
                    }
                }
            });

            ui.separator();

            // Manual filament price input
            ui.horizontal(|ui| {
                ui.label("Filament price (EUR/kg):");
                ui.add(egui::DragValue::new(&mut self.logic.filament_price).speed(0.1));
            });

            // Other inputs
            ui.horizontal(|ui| {
                ui.label("Filament weight (grams):");
                ui.add(egui::DragValue::new(&mut self.logic.filament_weight).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Electricity rate (EUR/kWh):");
                ui.add(egui::DragValue::new(&mut self.logic.electricity_rate).speed(0.01));
            });

            ui.horizontal(|ui| {
                ui.label("Printer wattage (W):");
                ui.add(egui::DragValue::new(&mut self.logic.printer_wattage).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Print time (hours):");
                ui.add(egui::DragValue::new(&mut self.logic.print_time).speed(0.1));
            });

            ui.horizontal(|ui| {
                ui.label("Shipping cost:");
                ui.add(egui::DragValue::new(&mut self.logic.shipping_cost).speed(0.1));
            });

            ui.horizontal(|ui| {
                ui.label("Markup percentage:");
                ui.add(egui::DragValue::new(&mut self.logic.markup_percentage).speed(1.0));
            });

            // Calculate button
            if ui.button("Calculate").clicked() {
                self.logic.calculate_costs();
            }

            ui.separator();

            // Results
            ui.label(format!(
                "Total cost: {:.2} {}",
                self.logic.total_cost,
                self.logic.currency_symbol()
            ));
            ui.label(format!(
                "Suggested price (with markup): {:.2} {}",
                self.logic.suggested_price,
                self.logic.currency_symbol()
            ));
            ui.label(format!(
                "Wear and tear cost: {:.2} {}",
                self.logic.wear_and_tear_cost,
                self.logic.currency_symbol()
            ));
        });

        // Help dialog
        if self.show_help {
            egui::Window::new("Help")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("How to Use the FDM Cost Calculator:");
                    ui.indent("help_instructions", |ui| {
                        ui.label("• The default filament price is in Euros.");
                        ui.label("• If using a different currency, adjust filament prices accordingly.");
                        ui.label("• Input filament weight, electricity rate, printer wattage, and print time.");
                        ui.label("• Add shipping cost and specify your desired markup percentage.");
                        ui.label("• Click 'Calculate' to see the total cost and suggested pricing.");
                    });
                    if ui.button("Close").clicked() {
                        self.show_help = false;
                    }
                });
        }
    }
}

pub fn load_logo(cc: &eframe::CreationContext<'_>) -> Option<TextureHandle> {
    let bytes = include_bytes!("../assets/logo.png");
    let image = image::load_from_memory(bytes).ok()?.to_rgba8();
    let size = [image.width() as _, image.height() as _];
    let pixels = image.into_raw();

    Some(cc.egui_ctx.load_texture(
        "logo",
        egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
        egui::TextureOptions::default(),
    ))
}
