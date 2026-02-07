//! # Saffron AST
//!
//! Abstract Syntax Tree definitions for the Saffron programming language.
//! Every node in the AST carries a `Span` for source location tracking.

use serde::{Deserialize, Serialize};

/// Source location span
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub file: String,
    pub start_line: u32,
    pub start_col: u32,
    pub end_line: u32,
    pub end_col: u32,
    pub byte_offset: usize,
    pub byte_length: usize,
}

/// Physical unit types — the core of Saffron's type safety
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Unit {
    // Temperature
    Celsius,
    Fahrenheit,
    Kelvin,
    // Mass
    Grams,
    Kilograms,
    Ounces,
    Pounds,
    Milligrams,
    // Volume
    Milliliters,
    Liters,
    Cups,
    Tablespoons,
    Teaspoons,
    FluidOunces,
    // Time
    Seconds,
    Minutes,
    Hours,
    // Length
    Centimeters,
    Millimeters,
    Inches,
    // Energy
    Joules,
    Calories,
    Kilocalories,
    // Power
    Watts,
    // Percentage
    Percent,
}

/// Ingredient category enum (closed set)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IngredientCategory {
    Protein,
    Fat,
    Carbohydrate,
    Liquid,
    Seasoning,
    Produce,
    Dairy,
}

/// Process type enum — the closed set of all culinary transformations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProcessType {
    // Thermal
    Fry,
    DeepFry,
    Saute,
    Boil,
    Simmer,
    Steam,
    Blanch,
    Braise,
    Roast,
    Bake,
    Grill,
    Broil,
    Smoke,
    SousVide,
    Poach,
    Caramelize,
    Toast,
    Flambe,
    // Mechanical
    Cut,
    Dice,
    Mince,
    Julienne,
    Chiffonade,
    Crush,
    Grate,
    Blend,
    Knead,
    Fold,
    Whisk,
    Pound,
    Peel,
    Crack,
    // Chemical
    Marinate,
    Brine,
    Cure,
    Ferment,
    Pickle,
    Emulsify,
    Deglaze,
    Reduce,
    Dissolve,
    Leaven,
    // Container operations
    Add,
    Remove,
    Transfer,
    Drain,
    // Thermal control
    Heat,
    Cool,
    Preheat,
    // Temporal
    Wait,
    WaitUntil,
    Rest,
    // Serving
    Serve,
    Plate,
    Garnish,
    Season,
}

/// Comparison operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CmpOp {
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
}

/// Doneness levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Doneness {
    Raw,
    Rare,
    MediumRare,
    Medium,
    MediumWell,
    WellDone,
    Charred,
}

/// Phase of matter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Phase {
    Solid,
    Liquid,
    Gas,
    Gel,
    Foam,
    Emulsion,
}

/// Difficulty rating
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

/// Annotation on recipes and functions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Annotation {
    pub name: String,
    pub value: String,
    pub span: Span,
}

/// A typed parameter in declarations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub value: Expr,
    pub span: Span,
}

/// Type reference (e.g., Egg, FryingPan, Oil)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeRef {
    pub name: String,
    pub generics: Vec<TypeRef>,
    pub span: Span,
}

/// Expression nodes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    /// Numeric literal with unit: 180.celsius, 50.ml
    UnitLiteral {
        value: f64,
        unit: Unit,
        span: Span,
    },
    /// Plain numeric literal: 42, 3.14
    NumericLiteral {
        value: f64,
        span: Span,
    },
    /// Percentage literal: 76%
    PercentLiteral {
        value: f64,
        span: Span,
    },
    /// String literal: "hello"
    StringLiteral {
        value: String,
        span: Span,
    },
    /// Boolean literal: true, false
    BoolLiteral {
        value: bool,
        span: Span,
    },
    /// Identifier reference: egg, pan, oil
    Identifier {
        name: String,
        span: Span,
    },
    /// Enum variant: .Chicken, .StainlessSteel
    EnumVariant {
        variant: String,
        span: Span,
    },
    /// Field access: oil.state.temperature
    FieldAccess {
        object: Box<Expr>,
        field: String,
        span: Span,
    },
    /// Process call: Heat(pan, to: 180.celsius)
    ProcessCall {
        process: ProcessType,
        args: Vec<Param>,
        span: Span,
    },
    /// Comparison: oil.temp >= 180.celsius
    Comparison {
        left: Box<Expr>,
        op: CmpOp,
        right: Box<Expr>,
        span: Span,
    },
    /// Object construction: Egg(type: .Chicken, quantity: 1)
    Construction {
        type_ref: TypeRef,
        params: Vec<Param>,
        span: Span,
    },
    /// Array literal: [yolk, white]
    Array {
        elements: Vec<Expr>,
        span: Span,
    },
    /// Lambda / condition: () => oil.temp >= 180.celsius
    Lambda {
        body: Box<Expr>,
        span: Span,
    },
}

/// Destructuring pattern: -> [yolk, white]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Destructure {
    pub bindings: Vec<String>,
    pub span: Span,
}

/// A single recipe step
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Step {
    /// Sequential step: 1: Heat(pan, to: 180.celsius)
    Sequential {
        number: u32,
        action: Box<Expr>,
        output: Option<Destructure>,
        span: Span,
    },
    /// Parallel steps: parallel { a: ..., b: ... }
    Parallel {
        number: u32,
        sub_steps: Vec<SubStep>,
        span: Span,
    },
}

/// Sub-step within a parallel block
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubStep {
    pub label: String,
    pub action: Box<Expr>,
    pub output: Option<Destructure>,
    pub span: Span,
}

/// Ingredient declaration in the ingredients block
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IngredientDecl {
    pub name: String,
    pub type_ref: TypeRef,
    pub params: Vec<Param>,
    pub span: Span,
}

/// Equipment declaration in the equipment block
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipmentDecl {
    pub name: String,
    pub type_ref: TypeRef,
    pub params: Vec<Param>,
    pub span: Span,
}

/// Expected result definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpectedResult {
    pub type_ref: TypeRef,
    pub properties: Vec<Param>,
    pub span: Span,
}

/// A complete recipe — the top-level AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub annotations: Vec<Annotation>,
    pub ingredients: Vec<IngredientDecl>,
    pub equipment: Vec<EquipmentDecl>,
    pub steps: Vec<Step>,
    pub expected_result: ExpectedResult,
    pub nutrition: Option<String>, // "auto" or manual override
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_serialization() {
        let unit = Unit::Celsius;
        let json = serde_json::to_string(&unit).unwrap();
        assert_eq!(json, "\"Celsius\"");
    }

    #[test]
    fn test_process_type_completeness() {
        // Ensure all process types are serializable
        let processes = vec![
            ProcessType::Fry,
            ProcessType::Boil,
            ProcessType::Bake,
            ProcessType::Grill,
            ProcessType::Add,
            ProcessType::Heat,
            ProcessType::Wait,
        ];
        for p in processes {
            let json = serde_json::to_string(&p).unwrap();
            assert!(!json.is_empty());
        }
    }

    #[test]
    fn test_recipe_roundtrip_json() {
        let span = Span {
            file: "test.saffron".into(),
            start_line: 1, start_col: 1,
            end_line: 1, end_col: 10,
            byte_offset: 0, byte_length: 10,
        };
        
        let recipe = Recipe {
            name: "TestRecipe".into(),
            annotations: vec![],
            ingredients: vec![],
            equipment: vec![],
            steps: vec![],
            expected_result: ExpectedResult {
                type_ref: TypeRef {
                    name: "TestResult".into(),
                    generics: vec![],
                    span: span.clone(),
                },
                properties: vec![],
                span: span.clone(),
            },
            nutrition: Some("auto".into()),
            span,
        };

        let json = serde_json::to_string_pretty(&recipe).unwrap();
        let deserialized: Recipe = serde_json::from_str(&json).unwrap();
        assert_eq!(recipe, deserialized);
    }
}
