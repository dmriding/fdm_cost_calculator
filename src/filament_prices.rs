use std::collections::HashMap;

pub fn get_filament_prices() -> HashMap<&'static str, HashMap<&'static str, f32>> {
    HashMap::from([
        ("Bambu Lab", HashMap::from([
            ("PLA", 27.08),
            ("PLA Metal", 29.16),
            ("PLA Silk", 29.16),
            ("PLA Matte", 23.96),
            ("PLA Luminous", 29.16),
            ("PC", 44.80),
            ("PA6", 46.88),
            ("PETG", 22.99),
            ("ABS", 27.08),
            ("ASA", 33.33),
            ("TPU 95A", 45.84),
            ("Support PLA/PETG", 38.54), // 0.5kg -> adjusted during runtime
            ("Support ABS", 17.71),      // 0.5kg -> adjusted during runtime
            ("PVA Support", 43.76),      // 0.5kg -> adjusted during runtime
        ])),
        ("eSun", HashMap::from([
            ("PETG", 22.99),
            ("ABS+", 22.99),
            ("ABS+ High Speed", 22.99),
            ("ASA", 25.99),
            ("TPU 95A", 37.99),
            ("PLA+", 23.99),
            ("PLA Matte", 19.99),
            ("PLA Silk Magic Multicolor", 25.99),
            ("PLA Silk", 19.99),
            ("PLA Silk Metal", 19.99),
            ("PLA Metal", 28.99),
            ("PLA Luminous", 26.99),
        ])),
        ("Elegoo", HashMap::from([
            ("PLA", 17.50),
        ])),
        ("Raise3D", HashMap::from([
            ("PLA Hyper Speed", 44.90),
            ("ABS Hyper Speed", 44.90),
            ("ASA", 49.90),
            ("PETG", 36.90),
            ("ABS", 36.90),
            ("PLA", 36.90),
        ])),
        ("Polymaker", HashMap::from([
            ("PolyMax Tough PETG", 45.90),
            ("Polylite ASA", 34.90),
            ("Polylite PLA", 29.90),
            ("PA6", 54.90),
            ("Polyflex TPU", 35.90),
            ("Polymax PLA", 45.90),
            ("PolyTerra PLA", 20.46),
            ("PolySonic PLA", 31.90),
        ])),
        ("Creality", HashMap::from([
            ("PETG", 19.90),
            ("PLA+", 17.90),
            ("ABS Hyper Speed", 28.90),
            ("PLA Hyper Speed", 24.80),
        ])),
    ])
}
