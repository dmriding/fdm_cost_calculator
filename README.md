# FDM Cost Calculator

The **FDM Cost Calculator** is a desktop application designed to help you calculate the cost of FDM (Fused Deposition Modeling) 3D prints. It provides a precise breakdown of costs based on filament usage, electricity, printer wear and tear, and shipping, while also supporting profit margin calculations. The app features an intuitive interface and allows switching between multiple currencies.

---

## Features

- **Cost Calculation**: Accurately calculates the total cost of a 3D print, factoring in:
  - Filament cost and weight.
  - Electricity rate and printer wattage.
  - Wear and tear and shipping costs.
- **Currency Switching**: Seamlessly switch between USD ($), EUR (€), and GBP (£).
- **Profit Estimates**: Automatically calculates suggested pricing with profit margins of 10%, 20%, 30%, 50%, and 100%.
- **Simple and Intuitive UI**: A user-friendly interface featuring sliders and buttons for easy adjustments.
- **Dynamic Multi-Color Support**: Add up to 16 different materials or colors for complex prints.

---

## Installation

### Prerequisites

Before using the **FDM Cost Calculator**, ensure you have the following installed:

- **Rust**: Download and install the latest stable version of Rust from the [official website](https://www.rust-lang.org/).
- **Cargo**: Cargo is included with Rust and acts as the package manager for this project.

### Steps to Install and Run

1. **Clone the Repository**:
    ```bash
    git clone https://github.com/dmriding/fdm_cost_calculator.git
    cd fdm_cost_calculator
    ```

2. **Build the Project**:
    Use Cargo to download dependencies and compile the project:
    ```bash
    cargo build
    ```

3. **Run the Application**:
    Launch the app with:
    ```bash
    cargo run
    ```

The **FDM Cost Calculator** GUI will open, ready for use.

---

## How to Use

### Input Fields

- **Filament Details**: Specify the cost per kilogram, brand, material, weight (grams), and carbon-based status (if applicable).
- **Electricity**: Enter your local electricity rate (e.g., EUR/kWh) and your printer’s wattage (W).
- **Print Time**: Provide the estimated time required for the print in hours.
- **Markup and Shipping**: Adjust the markup percentage for profit margins and include any shipping costs.

### Multi-Color/Material Support
- Toggle between **Single-Color Mode** and **Multi-Color Mode**.
- In Multi-Color Mode, add up to 16 materials with detailed individual cost inputs.

### Currency Switching
Click the currency button to cycle through USD ($), EUR (€), and GBP (£). All costs will update to reflect the selected currency.

### Calculation Results
Once all inputs are provided, click **Calculate** to compute:
- **Total Cost**: Combines filament, electricity, wear and tear, and shipping costs.
- **Suggested Price**: Includes your chosen profit margin.
- **Profit Estimates**: Shows potential earnings at 10%, 20%, 30%, 50%, and 100% margins.

---

## Screenshots

- **Single-Color Mode**: ![Single Color Screenshot](assets/screenshots/single_color_mode.png)  
- **Multi-Color Mode**: ![Multi-Color Screenshot](assets/screenshots/multi_color_mode.png)

---

## Troubleshooting

### Common Issues
- **Missing Dependencies**:  
  If you encounter errors related to missing dependencies, run:
  ```bash
  cargo build
  ```

- **Crashes or Bugs**:  
  Please report any issues in the repository’s [Issues section](https://github.com/dmriding/fdm_cost_calculator/issues).

---

## Roadmap

Future planned features:
1. Integration with slicers (e.g., Cura, PrusaSlicer, Bambu Studio) to import filament usage and print time data.
2. Advanced filament management (density tracking, spool management).
3. Export functionality (PDF/CSV cost breakdowns).
4. Platform fee and tax calculations (e.g., Etsy, Amazon).
5. Optional cloud syncing (if requested by users).
6. Open-source contributions to extend functionality.

---

## Contribution

Contributions are welcome! If you have ideas for improving the app:
- Fork the repository.
- Open a pull request with your changes.
- Submit feature requests or bug reports in the [Issues section](https://github.com/dmriding/fdm_cost_calculator/issues).

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
