use std::collections::HashMap;

pub enum Currency {
    USD,
    EUR,
    GBP,
}

pub struct FilamentUsage {
    pub brand: String,
    pub material: String,
    pub weight: f32,         // Weight of the filament used (grams)
    pub price_per_roll: f32, // Price of the filament roll
    pub roll_weight: f32,    // Weight of the filament roll (grams)
    pub is_carbon_based: bool, // Whether the filament is carbon-based
}

pub struct CalculatorLogic {
    pub currency: Currency, // Selected currency
    pub filament_prices: HashMap<&'static str, HashMap<&'static str, f32>>, // Filament prices
    pub filaments: Vec<FilamentUsage>, // Multi-material usage data
    pub purge_waste_weight: f32,       // Total purge/waste weight in grams

    // Calculator fields
    pub electricity_rate: f32, // Electricity rate in EUR/kWh
    pub printer_wattage: f32, // Printer wattage in watts
    pub print_time: f32, // Print time in hours
    pub hourly_charge: f32, // User-defined charge per print hour
    pub shipping_cost: f32, // Shipping cost in EUR
    pub markup_percentage: f32, // Markup percentage for profit
    pub wear_and_tear_cost: f32, // Wear and tear cost
    pub total_cost: f32, // Total cost (calculated)
    pub suggested_price: f32, // Suggested price (calculated)
    pub suggested_price_with_post_processing: f32, // Suggested price with post-processing

    // Post-processing fields
    pub post_processing_hours: f32,
    pub post_processing_rate: f32,
}

impl Default for CalculatorLogic {
    fn default() -> Self {
        Self {
            currency: Currency::EUR,
            filament_prices: crate::filament_prices::get_filament_prices(),
            filaments: vec![FilamentUsage {
                brand: "Custom".to_string(),
                material: "Custom".to_string(),
                weight: 0.0,
                price_per_roll: 0.0,
                roll_weight: 1000.0, // Default to 1kg
                is_carbon_based: false,
            }],
            purge_waste_weight: 0.0,

            electricity_rate: 0.26,
            printer_wattage: 250.0,
            print_time: 0.0,
            hourly_charge: 2.50,
            shipping_cost: 0.0,
            markup_percentage: 20.0,
            wear_and_tear_cost: 0.0,
            total_cost: 0.0,
            suggested_price: 0.0,
            suggested_price_with_post_processing: 0.0,

            post_processing_hours: 0.0,
            post_processing_rate: 15.0, // Default post-processing hourly rate
        }
    }
}

impl CalculatorLogic {
    /// Updates the number of filaments in use.
    pub fn update_filament_count(&mut self, count: usize) {
        if count > self.filaments.len() {
            self.filaments.extend((self.filaments.len()..count).map(|_| FilamentUsage {
                brand: "Custom".to_string(),
                material: "Custom".to_string(),
                weight: 0.0,
                price_per_roll: 0.0,
                roll_weight: 1000.0,
                is_carbon_based: false,
            }));
        } else {
            self.filaments.truncate(count);
        }
    }

    /// Adds a new filament to the list (for multi-color/material mode).
    pub fn add_filament(&mut self) {
        if self.filaments.len() < 16 {
            self.filaments.push(FilamentUsage {
                brand: "Custom".to_string(),
                material: "Custom".to_string(),
                weight: 0.0,
                price_per_roll: 0.0,
                roll_weight: 1000.0,
                is_carbon_based: false,
            });
        }
    }

    /// Removes a filament from the list by index.
    pub fn remove_filament(&mut self, index: usize) {
        if index < self.filaments.len() {
            self.filaments.remove(index);
        }
    }

    /// Calculates the costs of the print job.
    pub fn calculate_costs(&mut self) {
        let mut total_filament_cost = 0.0;

        // Calculate costs for each filament
        for filament in &self.filaments {
            let cost_per_gram = filament.price_per_roll / filament.roll_weight;
            total_filament_cost += cost_per_gram * filament.weight;

            if filament.is_carbon_based {
                total_filament_cost += 10.0 * (filament.weight / 1000.0); // Extra cost for CF/GF
            }
        }

        // Add purge/waste material cost
        total_filament_cost += (self.purge_waste_weight / 1000.0) * 10.0; // Example purge material cost

        // Calculate electricity costs
        let electricity_cost =
            (self.printer_wattage / 1000.0) * self.print_time * self.electricity_rate;

        // Calculate wear and tear costs
        self.wear_and_tear_cost = self.print_time * 0.05; // Simplified wear and tear (EUR/hour)
        self.wear_and_tear_cost += self.purge_waste_weight * 0.01; // Extra wear for purge waste

        // Calculate post-processing costs
        let post_processing_cost = self.post_processing_hours * self.post_processing_rate;

        // Calculate total costs
        self.total_cost = total_filament_cost + electricity_cost + self.wear_and_tear_cost
            + self.hourly_charge * self.print_time + self.shipping_cost + post_processing_cost;

        // Calculate suggested prices
        self.suggested_price = self.total_cost * (1.0 + self.markup_percentage / 100.0);
        self.suggested_price_with_post_processing = self.suggested_price + post_processing_cost;
    }

    /// Switches the currency symbol.
    pub fn switch_currency(&mut self) {
        self.currency = match self.currency {
            Currency::USD => Currency::EUR,
            Currency::EUR => Currency::GBP,
            Currency::GBP => Currency::USD,
        };
    }

    /// Returns the currency symbol as a string.
    pub fn currency_symbol(&self) -> &str {
        match self.currency {
            Currency::USD => "$",
            Currency::EUR => "€",
            Currency::GBP => "£",
        }
    }
}
