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

            // Input for purge/waste material weight
            ui.horizontal(|ui| {
                ui.label("Purge/Waste Filament (grams):");
                ui.add(egui::DragValue::new(&mut self.logic.purge_waste_weight).speed(1.0));
            });

            // Multi-material section
            ui.collapsing("Filament Details", |ui| {
                for (i, filament) in self.logic.filaments.iter_mut().enumerate() {
                    ui.group(|ui| {
                        ui.label(format!("Filament #{}", i + 1));

                        // Brand selection
                        let brands: Vec<_> = self.logic.filament_prices.keys().cloned().collect();
                        egui::ComboBox::new(format!("brand_selector_{}", i), "Select Brand")
                            .selected_text(filament.brand.clone())
                            .show_ui(ui, |ui| {
                                for brand in &brands {
                                    if ui
                                        .selectable_label(&filament.brand == brand, *brand)
                                        .clicked()
                                    {
                                        filament.brand = brand.to_string();
                                    }
                                }
                            });

                        // Material selection
                        if let Some(materials) = self.logic.filament_prices.get(&filament.brand.as_str()) {
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
                                            if let Some(price) = materials.get(&material) {
                                                filament.price_per_kg = *price;
                                            }
                                        }
                                    }
                                });
                        }                        

                        // Carbon-based checkbox
                        ui.checkbox(&mut filament.is_carbon_based, "Carbon-Based");

                        // Filament weight input
                        ui.horizontal(|ui| {
                            ui.label("Weight (grams):");
                            ui.add(egui::DragValue::new(&mut filament.weight).speed(1.0));
                        });

                        // Custom price input
                        ui.horizontal(|ui| {
                            ui.label("Price (€/kg):");
                            ui.add(egui::DragValue::new(&mut filament.price_per_kg).speed(0.1));
                        });
                    });
                }
            });

            ui.separator();

            // Other inputs
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
                        ui.label("• **Defaults are for reference only:**");
                        ui.indent("defaults_info", |ui| {
                            ui.label("   - The default filament price is in Euros.");
                            ui.label("   - Hourly charge and electricity rates are common estimates.");
                            ui.label("   - Default values may not reflect your actual costs.");
                        });
                        ui.label("• **For accurate costs and estimates:**");
                        ui.indent("input_info", |ui| {
                            ui.label("   - Input the actual cost of your filament.");
                            ui.label("   - Adjust the electricity rate and printer wattage.");
                            ui.label("   - Specify the filament weight and print time.");
                            ui.label("   - Add your shipping cost and desired markup percentage.");
                            ui.label("   - Customize the hourly charge if needed.");
                        });
                        ui.label("• **About Currency Selection:**");
                        ui.indent("currency_info", |ui| {
                            ui.label("   - The currency button lets you switch the displayed currency symbol.");
                            ui.label("   - This is purely for visual preferences and does not perform automatic conversions.");
                            ui.label("   - Adjust your filament price manually to match your preferred currency.");
                        });
                        ui.label("• Once all fields are adjusted, click 'Calculate' to:");
                        ui.indent("calculate_info", |ui| {
                            ui.label("   - See the total cost for your print.");
                            ui.label("   - Get a suggested selling price based on your inputs.");
                        });
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
