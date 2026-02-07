//! # Saffron Ingredient Database (SID)
//!
//! Client library for querying the ingredient database.
//! The SID contains physical and chemical properties for all known ingredients.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Multilingual name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedName {
    pub en: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja: Option<String>,
}

/// Chemical composition per 100g
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Composition {
    pub water: f64,
    pub protein: f64,
    pub total_fat: f64,
    pub saturated_fat: f64,
    pub carbohydrates: f64,
    pub fiber: f64,
    pub sugar: f64,
    pub ph: Option<f64>,
    #[serde(default)]
    pub minerals: HashMap<String, f64>,
    #[serde(default)]
    pub vitamins: HashMap<String, f64>,
}

/// Physical properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalProperties {
    pub density_g_per_ml: Option<f64>,
    pub boiling_point_celsius: Option<f64>,
    pub freezing_point_celsius: Option<f64>,
    pub smoke_point_celsius: Option<f64>,
    pub specific_heat_j_per_g_k: Option<f64>,
    pub flash_point_celsius: Option<f64>,
}

/// A complete ingredient entry in the SID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientEntry {
    pub id: String,
    pub name: LocalizedName,
    pub category: String,
    pub subcategory: Option<String>,
    pub composition: Composition,
    pub physical: PhysicalProperties,
    #[serde(default)]
    pub allergens: Vec<String>,
    #[serde(default)]
    pub substitutes: Vec<String>,
    #[serde(default)]
    pub sources: Vec<String>,
}

/// The SID client for querying ingredients
pub struct SidClient {
    ingredients: HashMap<String, IngredientEntry>,
}

impl SidClient {
    /// Create a new SID client from a JSON data directory
    pub fn new() -> Self {
        Self {
            ingredients: HashMap::new(),
        }
    }

    /// Load ingredients from a JSON string
    pub fn load_json(&mut self, json: &str) -> Result<(), serde_json::Error> {
        let entries: Vec<IngredientEntry> = serde_json::from_str(json)?;
        for entry in entries {
            self.ingredients.insert(entry.id.clone(), entry);
        }
        Ok(())
    }

    /// Look up an ingredient by ID
    pub fn get(&self, id: &str) -> Option<&IngredientEntry> {
        self.ingredients.get(id)
    }

    /// Search ingredients by name (English)
    pub fn search(&self, query: &str) -> Vec<&IngredientEntry> {
        let q = query.to_lowercase();
        self.ingredients
            .values()
            .filter(|e| e.name.en.to_lowercase().contains(&q) || e.id.contains(&q))
            .collect()
    }

    /// Get all ingredients in a category
    pub fn by_category(&self, category: &str) -> Vec<&IngredientEntry> {
        self.ingredients
            .values()
            .filter(|e| e.category == category)
            .collect()
    }

    /// Total number of ingredients loaded
    pub fn count(&self) -> usize {
        self.ingredients.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_json() -> &'static str {
        r#"[{
            "id": "chicken_egg",
            "name": { "en": "Chicken Egg", "es": "Huevo de gallina" },
            "category": "protein",
            "subcategory": "egg",
            "composition": {
                "water": 76.15, "protein": 12.56, "total_fat": 9.51,
                "saturated_fat": 3.13, "carbohydrates": 0.72,
                "fiber": 0.0, "sugar": 0.37, "ph": 7.6
            },
            "physical": {
                "density_g_per_ml": 1.031,
                "boiling_point_celsius": 100.0,
                "specific_heat_j_per_g_k": 3.18
            },
            "allergens": ["eggs"],
            "substitutes": ["duck_egg", "quail_egg"],
            "sources": ["USDA FoodData Central #171287"]
        }]"#
    }

    #[test]
    fn test_load_and_query() {
        let mut client = SidClient::new();
        client.load_json(sample_json()).unwrap();
        assert_eq!(client.count(), 1);

        let egg = client.get("chicken_egg").unwrap();
        assert_eq!(egg.name.en, "Chicken Egg");
        assert_eq!(egg.composition.protein, 12.56);
    }

    #[test]
    fn test_search() {
        let mut client = SidClient::new();
        client.load_json(sample_json()).unwrap();
        let results = client.search("egg");
        assert_eq!(results.len(), 1);
    }
}
