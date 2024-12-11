use eframe::egui;
use eframe::epaint::TextureHandle;
use egui::Grid;
use crate::logic::{CalculatorLogic, Currency};

pub struct CalculatorUI {
    pub logic: CalculatorLogic,
    pub logo: Option<TextureHandle>, // Texture handle for the logo
    pub show_help: bool,             // Whether to show the help dialog
    pub is_multi_color: bool,        // Track toggle state
}

impl Default for CalculatorUI {
    fn default() -> Self {
        Self {
            logic: CalculatorLogic::default(),
            logo: None,
            show_help: false,
            is_multi_color: false, // Default to single-color mode
        }
    }
}

impl eframe::App for CalculatorUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // App heading and currency switcher
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

            // Help button and logo
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(logo) = &self.logo {
                    ui.image((logo.id(), egui::vec2(64.0, 64.0)));
                }

                if ui.selectable_label(self.show_help, egui::RichText::new("?").size(20.0))
                    .clicked()
                {
                    self.show_help = true;
                }
            });
            });

            ui.separator();

            // Print Type Toggle
            ui.horizontal(|ui| {
                ui.label("Print Type:");
                if ui.selectable_label(!self.is_multi_color, "Single Color").clicked() {
                    self.is_multi_color = false;
                    self.logic.update_filament_count(1); // Reset to one filament
                }
                if ui.selectable_label(self.is_multi_color, "Multi-Color/Material").clicked() {
                    self.is_multi_color = true;
                    if self.logic.filaments.len() < 2 {
                        self.logic.update_filament_count(2); // Start with two filaments
                    }
                }
            });

            ui.separator();

            // Filament Details Section
            ui.add_space(10.0); // Add spacing before the grid
            Grid::new("filament_grid")
                .num_columns(3) // Three columns
                .spacing([20.0, 10.0]) // Horizontal and vertical spacing
                .show(ui, |ui| {
                    let mut remove_index = None; // Track the index of the filament to remove
                    for (i, filament) in self.logic.filaments.iter_mut().enumerate() {
                        ui.vertical(|ui| {
                            ui.group(|ui| {
                                ui.label(format!("Filament #{}", i + 1));

                                // Brand selection
                                let brands: Vec<_> = self.logic.filament_prices.keys().cloned().collect();
                                egui::ComboBox::new(format!("brand_selector_{}", i), "Select Brand")
                                    .selected_text(filament.brand.clone())
                                    .show_ui(ui, |ui| {
                                        for brand in &brands {
                                            if ui.selectable_label(&filament.brand == brand, *brand).clicked() {
                                                filament.brand = brand.to_string();
                                            }
                                        }
                                    });

                                if let Some(materials) = self.logic.filament_prices.get(filament.brand.as_str()) {
                                    let material_keys: Vec<_> = materials.keys().cloned().collect();
                                    egui::ComboBox::new(format!("material_selector_{}", i), "Select Material")
                                        .selected_text(filament.material.clone())
                                        .show_ui(ui, |ui| {
                                            for material in &material_keys {
                                                if ui
                                                    .selectable_label(&filament.material == material, *material)
                                                    .clicked()
                                                {
                                                    filament.material = material.to_string();
                                                    if let Some(price) = materials.get(material) {
                                                        filament.price_per_kg = *price;
                                                    }
                                                }
                                            }
                                        });
                                }

                                ui.checkbox(&mut filament.is_carbon_based, "Carbon-Based");

                                ui.horizontal(|ui| {
                                    ui.label("Weight (grams):");
                                    ui.add(egui::DragValue::new(&mut filament.weight).speed(1.0));
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Price (€/kg):");
                                    ui.add(egui::DragValue::new(&mut filament.price_per_kg).speed(0.1));
                                });

                                // Trash can icon for removing filaments
                                if self.is_multi_color && i > 0 {
                                    if ui.button(egui::RichText::new("🗑️").size(18.0)).clicked() {
                                        remove_index = Some(i);
                                    }
                                }
                            });
                        });

                        if (i + 1) % 3 == 0 {
                            ui.end_row(); // Move to the next row after three columns
                        }
                    }

                    // Remove the marked filament after the loop
                    if let Some(index) = remove_index {
                        self.logic.remove_filament(index);
                    }
                });

            // Add button for adding filaments
            if self.is_multi_color && self.logic.filaments.len() < 16 {
                ui.horizontal(|ui| {
                    if ui.button("+ Add Filament").clicked() {
                        self.logic.add_filament();
                    }
                });
            }

            ui.separator();

            // Other inputs
            ui.horizontal(|ui| {
                ui.label("Purge/Waste Filament (grams):");
                ui.add(egui::DragValue::new(&mut self.logic.purge_waste_weight).speed(1.0));
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
                ui.label("Hourly Charge (€/hour):");
                ui.add(egui::DragValue::new(&mut self.logic.hourly_charge).speed(0.1));
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
                        ui.label("• **Switching Print Modes:** Use the 'Print Type' toggle to switch between Single Color and Multi-Color/Material modes.");
                        ui.label("• **Adding Filaments:** In Multi-Color mode, click '+ Add Filament' to include additional filaments (up to 16).");
                        ui.label("• **Removing Filaments:** Click the 🗑️ icon next to a filament to remove it from the list.");
                        ui.label("• **Specifying Filament Details:** For each filament:");
                        ui.indent("filament_info", |ui| {
                            ui.label("   - Select the brand and material.");
                            ui.label("   - Specify weight in grams.");
                            ui.label("   - Input cost per kilogram.");
                            ui.label("   - Check 'Carbon-Based' if applicable.");
                        });
                        ui.label("• **Inputting Additional Costs:** Provide:");
                        ui.indent("additional_costs", |ui| {
                            ui.label("   - Purge/Waste Filament weight.");
                            ui.label("   - Electricity rate (EUR/kWh).");
                            ui.label("   - Printer wattage (W).");
                            ui.label("   - Print time in hours.");
                            ui.label("   - Hourly charge for printing.");
                            ui.label("   - Shipping cost.");
                            ui.label("   - Markup percentage for profit.");
                        });
                        ui.label("• **Results:** After clicking 'Calculate', the app will display:");
                        ui.indent("results_info", |ui| {
                            ui.label("   - Total cost of the print.");
                            ui.label("   - Suggested selling price with markup.");
                            ui.label("   - Wear and tear cost.");
                        });
                        ui.label("• **Switching Currency:** Use the currency button to switch between €, £, and $ for display purposes only.");
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