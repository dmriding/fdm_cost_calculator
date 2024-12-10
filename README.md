# FDM Cost Calculator

This application is designed to help you calculate the cost of 3D printing FDM (Fused Deposition Modeling) prints. It factors in various parameters such as filament cost, electricity usage, printer wattage, print time, and markup percentage to give you a precise cost breakdown. Additionally, it allows you to switch between different currencies (USD, EUR, GBP).

## Features

- **Cost Calculation**: Calculates the total cost of a print based on filament cost, electricity, wear and tear, and shipping cost.
- **Currency Switching**: Supports multiple currencies: US Dollar (USD), Euro (EUR), and British Pound (GBP).
- **Profit Estimates**: Provides profit estimates at 10%, 20%, 30%, 50%, and 100% profit margins.
- **Simple User Interface**: Easy-to-use interface with sliders and buttons for quick input adjustments.

---

## Installation Instructions

### Prerequisites

Before using the **FDM Cost Calculator**, ensure you have the following installed:

- **Rust**: Install the latest stable version of Rust from the [official website](https://www.rust-lang.org/).
- **Cargo**: Cargo is the Rust package manager, and it comes bundled with Rust.

### Step-by-Step Installation

1. **Clone the repository**:
    ```bash
    git clone https://github.com/dmriding/fdm_cost_calculator.git
    cd fdm_cost_calculator
    ```

2. **Install dependencies**:
    The project uses the `eframe` crate for the GUI, as well as `image` for loading the logo. Install these dependencies with Cargo:
    ```bash
    cargo build
    ```

3. **Run the Application**:
    After installing dependencies, you can run the application with:
    ```bash
    cargo run
    ```

This will launch the **FDM Cost Calculator** GUI.

---

## How to Use the Application

### 1. **Input Fields**

On the main interface, you will see several input fields:

- **Filament cost per kilogram (EUR)**: Enter the cost of the filament you are using for the print in your chosen currency.
- **Electricity rate (EUR/kWh)**: Enter the electricity cost per kilowatt-hour in your area.
- **Printer wattage (W)**: Enter the wattage of your 3D printer.
- **Filament weight (grams)**: Enter the total weight of filament used for the print.
- **Print time (hours)**: Enter the estimated print time in hours.
- **Markup percentage (%)**: Set the markup percentage to calculate your suggested price.
- **Shipping cost (EUR)**: Include any shipping costs if applicable.

You can adjust these values using the sliders or by typing in the fields directly.

### 2. **Currency Switching**

The current currency is shown as a button next to "Currency". Click on this button to cycle through the following currencies:
- **USD ($)**
- **EUR (€)**
- **GBP (£)**

The selected currency will be displayed next to all cost values.

### 3. **Calculate Button**

Once you've entered the values, click the **Calculate** button to compute:
- **Total cost**: The total cost for the print, including filament, electricity, wear and tear, and shipping.
- **Suggested price (with markup)**: The suggested price based on your selected markup percentage.
- **Profit Estimates**: Profit estimates at various profit margins (10%, 20%, 30%, 50%, and 100%).

The results will be displayed below the input fields.

---

## Troubleshooting

### 1. **Missing Dependencies**

If you receive an error about missing dependencies, make sure to run:
```bash
cargo build
```

### License
This project is licensed under the MIT License - see the LICENSE file for details.

### Contribution
Feel free to fork the repository and contribute if you have ideas for improving the app! If you encounter any issues, please open an issue, and I’ll try to address it as soon as possible.