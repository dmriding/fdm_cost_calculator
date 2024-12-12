# Changelog

All notable changes to this project will be documented in this file.

The format follows:
- `vMONTH.YEAR.VERSION[a|b]`:
  - `MONTH.YEAR`: Indicates the release date.
  - `VERSION`: Incremental updates within the same month/year.
  - `a`: Alpha release.
  - `b`: Beta release.
  - No suffix: Stable major release.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]
### Added
- Scrollable UI for filament details, supporting up to 16 materials.
- Dynamic three-column layout for better organization.
- New default window size to fit content without requiring manual resizing.

### Fixed
- UI issues with bottom section being cut off for small window sizes.

---

## [v12.24.01b] - 2024-12-10
### Added
- Initial beta release of the FDM Cost Calculator.
- Single-Color Mode for simple cost calculations.
- Multi-Color/Material Mode supporting up to 16 filaments.
- Cost calculations for filament, electricity, shipping, and markup.
- Currency switching (USD, EUR, GBP).
- Basic profit margin estimates (10%, 20%, 30%, 50%, 100%).

---

## [v12.24.02b] - 2024-12-11
### Added
- Enhanced Multi-Color/Material Mode with detailed cost breakdowns.
- Support for carbon-based filaments (e.g., CF or GF) for accurate pricing.
- Energy and labor cost estimation, including electricity rate, printer wattage, and print time.
- Customizable markup percentage and shipping cost inputs.

---

## [v12.24.03b] - 2024-12-11
### Added
- Dynamic scrolling and sizing adjustments for improved UI responsiveness.
- New default window size to accommodate all UI elements without manual resizing.

### Fixed
- Resolved issues with the bottom section being cut off on smaller window sizes.
