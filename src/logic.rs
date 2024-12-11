use std::collections::HashMap;

pub enum Currency {
    USD,
    EUR,
    GBP,
}

pub struct CalculatorLogic {
    pub selected_brand: Option<String>, // Selected filament brand
    pub selected_material: Option<String>, // Selected filament material
    pub is_carbon_based: bool, // Whether the filament is carbon-based
    pub currency: Currency, // Selected currency
    pub filament_price: f32, // Current filament price
    pub filament_prices: HashMap<&'static str, HashMap<&'static str, f32>>, // Filament prices

    // Calculator fields
    pub filament_weight: f32, // Filament weight in grams
    pub electricity_rate: f32, // Electricity rate in EUR/kWh
    pub printer_wattage: f32, // Printer wattage in watts
    pub print_time: f32, // Print time in hours
    pub shipping_cost: f32, // Shipping cost in EUR
    pub markup_percentage: f32, // Markup percentage for profit
    pub wear_and_tear_cost: f32, // Wear and tear cost
    pub total_cost: f32, // Total cost (calculated)
    pub suggested_price: f32, // Suggested price (calculated)
}

impl Default for CalculatorLogic {
    fn default() -> Self {
        Self {
            selected_brand: None,
            selected_material: None,
            is_carbon_based: false,
            currency: Currency::EUR,
            filament_price: 0.0,
            filament_prices: crate::filament_prices::get_filament_prices(),

            filament_weight: 0.0,
            electricity_rate: 0.12,
            printer_wattage: 250.0,
            print_time: 0.0,
            shipping_cost: 0.0,
            markup_percentage: 20.0,
            wear_and_tear_cost: 0.0,
            total_cost: 0.0,
            suggested_price: 0.0,
        }
    }
}

impl CalculatorLogic {
    pub fn update_filament_price(&mut self) {
        if let (Some(brand), Some(material)) = (&self.selected_brand, &self.selected_material) {
            if let Some(price) = self
                .filament_prices
                .get(brand.as_str())
                .and_then(|materials| materials.get(material.as_str()))
            {
                self.filament_price = *price;

                // Adjust for carbon-based variants
                if self.is_carbon_based {
                    self.filament_price += 10.0; // Example fixed increase
                }
            }
        }
    }

    pub fn calculate_costs(&mut self) {
        let filament_cost = (self.filament_price / 1000.0) * self.filament_weight;
        let electricity_cost =
            (self.printer_wattage / 1000.0) * self.print_time * self.electricity_rate;

        // Adjust wear and tear for carbon-based materials
        self.wear_and_tear_cost = self.print_time * 0.05; // Simplified wear and tear (EUR/hour)
        if self.is_carbon_based {
            self.wear_and_tear_cost += 0.02 * self.print_time; // Extra for carbon-based
        }

        self.total_cost = filament_cost + electricity_cost + self.wear_and_tear_cost + self.shipping_cost;
        self.suggested_price = self.total_cost * (1.0 + self.markup_percentage / 100.0);
    }

    pub fn switch_currency(&mut self) {
        self.currency = match self.currency {
            Currency::USD => Currency::EUR,
            Currency::EUR => Currency::GBP,
            Currency::GBP => Currency::USD,
        };
    }

    pub fn currency_symbol(&self) -> &str {
        match self.currency {
            Currency::USD => "$",
            Currency::EUR => "€",
            Currency::GBP => "£",
        }
    }
}
