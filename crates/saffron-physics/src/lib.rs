//! # Saffron Physics Engine
//!
//! Real physical and chemical models for culinary simulation:
//! - Heat transfer (Fourier's law, Newton's cooling)
//! - Protein denaturation (Arrhenius kinetics)
//! - Maillard reaction
//! - Caramelization
//! - Starch gelatinization
//! - Evaporation (Antoine equation)
//! - Emulsion stability

/// Heat transfer calculation using Newton's law of cooling
pub fn heat_transfer(
    source_temp_c: f64,
    object_temp_c: f64,
    thermal_conductivity: f64, // W/(m*K)
    contact_area: f64,         // m^2
    object_mass: f64,          // kg
    specific_heat: f64,        // J/(kg*K)
    dt: f64,                   // seconds
) -> f64 {
    let q = thermal_conductivity * contact_area * (source_temp_c - object_temp_c);
    let delta_t = (q * dt) / (object_mass * specific_heat);
    object_temp_c + delta_t
}

/// Protein denaturation rate using Arrhenius equation
pub fn protein_denaturation_rate(
    temperature_c: f64,
    activation_energy: f64,  // J/mol (e.g., 80000 for egg albumin)
    frequency_factor: f64,   // 1/s (e.g., 1e10 for egg proteins)
) -> f64 {
    let temp_k = temperature_c + 273.15;
    let r = 8.314; // Gas constant J/(mol*K)
    frequency_factor * (-activation_energy / (r * temp_k)).exp()
}

/// Maillard reaction progress (0.0 to 1.0)
pub fn maillard_progress(
    temperature_c: f64,
    duration_s: f64,
    surface_moisture_pct: f64,
    reducing_sugars_pct: f64,
    amino_acids_pct: f64,
) -> f64 {
    if temperature_c < 110.0 || surface_moisture_pct > 80.0 {
        return 0.0;
    }
    let rate = protein_denaturation_rate(temperature_c, 50000.0, 1e8);
    let substrate = reducing_sugars_pct.min(amino_acids_pct) / 100.0;
    (rate * substrate * duration_s).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heat_transfer_basic() {
        // Heating water from 20C with source at 100C
        let new_temp = heat_transfer(
            100.0,  // source temp (C)
            20.0,   // object temp (C)
            0.6,    // water thermal conductivity
            0.01,   // contact area (m^2)
            0.25,   // 250g water
            4186.0, // specific heat of water
            1.0,    // 1 second
        );
        assert!(new_temp > 20.0, "Water should heat up");
        assert!(new_temp < 100.0, "Water shouldn't exceed source temp in 1s");
    }

    #[test]
    fn test_protein_denaturation() {
        // At 62C, egg white should start denaturing
        let rate_62 = protein_denaturation_rate(62.0, 80000.0, 1e10);
        let rate_20 = protein_denaturation_rate(20.0, 80000.0, 1e10);
        assert!(rate_62 > rate_20, "Denaturation should be faster at higher temp");
    }

    #[test]
    fn test_maillard_below_threshold() {
        let progress = maillard_progress(100.0, 60.0, 50.0, 5.0, 10.0);
        assert_eq!(progress, 0.0, "No Maillard below 110C");
    }

    #[test]
    fn test_maillard_too_wet() {
        let progress = maillard_progress(180.0, 60.0, 90.0, 5.0, 10.0);
        assert_eq!(progress, 0.0, "No Maillard when too wet");
    }
}
