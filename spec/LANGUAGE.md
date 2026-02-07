# Saffron Language Specification (SLS) v0.1.0

## 1. Introduction

Saffron is a strongly-typed, domain-specific programming language that formalizes culinary processes as computable, verifiable, and machine-executable programs. Every recipe is a program; every ingredient, piece of equipment, and cooking process is a typed object with physical constraints.

This document is the authoritative reference for Saffron's syntax and semantics. The formal grammar is defined in `grammar.peg` (PEG notation); this document provides the human-readable specification with rationale and examples.

### 1.1 Design Principles

1. **Correctness by Construction** — If a Saffron program compiles, the recipe is physically plausible. You cannot fry water or mix Celsius with Fahrenheit.

2. **One Canonical Form** — There is exactly one syntactic representation for each culinary operation. No synonyms, no alternative orderings, no implicit defaults.

3. **Declarative Intent** — Recipes describe *what* happens, not *how*. The runtime decides physical simulation details.

4. **Physical Fidelity** — Saffron models real thermodynamics, chemistry, and physics. Heat transfer follows Newton's law of cooling. Protein denaturation uses the Arrhenius equation.

5. **AI-First Ergonomics** — The language is optimized for machine code generation. Deterministic grammar, explicit structure, named parameters, and structured metadata all reduce LLM generation errors.

### 1.2 AI-First Design

Saffron is designed to be generated, validated, and iterated on by AI agents. The following language properties directly support this:

**Deterministic Grammar** — Saffron uses a PEG (Parsing Expression Grammar), which guarantees exactly one parse tree for any valid input. An LLM generating Saffron code never faces structural ambiguity.

**Fixed Block Order** — Recipe blocks always appear in the same order: annotations → ingredients → equipment → steps → expected_result → nutrition. An AI model can follow this template without deciding block placement.

**Enforced Identifier Casing** — PascalCase for types/processes, snake_case for values/parameters, SCREAMING_CASE for constants. An LLM always knows the correct casing from the semantic role.

**Named Arguments** — Process calls use named parameters (`to:`, `using:`, `on:`, `duration:`, `target:`) that are self-documenting. The LLM doesn't need to memorize positional argument semantics.

**Closed Process Set** — The 56 process types (Fry, Boil, Heat, etc.) are a fixed enum. An LLM cannot invent invalid operations.

**Unit Literals** — `180.celsius` is syntactically typed. An LLM that writes `180` alone gets a compile error, forcing dimensional correctness.

**AI Hints** — The `///ai:` comment syntax provides a first-class channel for AI-specific metadata that doesn't affect compilation but guides generation and reasoning.

**JSON AST** — The AST serializes to/from JSON via serde, enabling AI toolchains to operate on structured representations rather than raw text.


## 2. Lexical Grammar

### 2.1 Source Encoding

Saffron source files use UTF-8 encoding. The file extension is `.saffron`.

### 2.2 Whitespace

Whitespace characters (space, tab, carriage return, newline) separate tokens but are otherwise insignificant. Saffron is not whitespace-sensitive — indentation is conventional, not syntactic.

### 2.3 Comments

Saffron supports three comment forms, all line-based:

```saffron
// Regular comment — ignored by the parser
/// Doc comment — attached to the following AST node
///ai: AI hint — machine-readable metadata for AI agents
```

**Regular comments** (`//`) are discarded during parsing.

**Doc comments** (`///`) are preserved in the AST and used for documentation generation. They attach to the immediately following declaration or step.

**AI hints** (`///ai:`) are a first-class feature for AI-LLM interoperability. They are preserved in the AST as structured metadata. AI hints use a key-value format:

```saffron
///ai: critical_for=food_safety reason="chicken must reach 74°C"
///ai: suggest_alternative=vegan
///ai: substitution egg->flax_egg when=vegan
///ai: difficulty_note="requires precise timing"
```

### 2.4 Identifiers

Saffron enforces identifier casing at the lexical level:

| Casing | Pattern | Usage | Examples |
|--------|---------|-------|----------|
| PascalCase | `[A-Z][a-zA-Z0-9]*` | Types, processes, enum types | `Egg`, `FryingPan`, `Heat`, `Doneness` |
| snake_case | `[a-z][a-z0-9_]*` | Variables, parameters, properties | `egg`, `my_pan`, `total_time` |
| SCREAMING_CASE | `[A-Z][A-Z0-9_]+` | Constants | `MAX_TEMP`, `DEFAULT_SERVINGS` |

This is not a convention — it's enforced by the lexer. Using the wrong casing produces a compile error.

**Rationale:** Enforced casing eliminates naming ambiguity for both humans and AI. When an LLM sees `PascalCase`, it knows it's a type or process. When it generates a variable name, it knows to use `snake_case`.

### 2.5 Keywords

The following identifiers are reserved and cannot be used as user-defined names:

**Phase 1 (current):** `recipe`, `ingredients`, `equipment`, `steps`, `expected_result`, `nutrition`, `parallel`, `auto`, `true`, `false`, `import`, `from`

**Phase 2+ (reserved):** `fn`, `let`, `const`, `mut`, `return`, `if`, `else`, `match`, `for`, `while`, `in`, `async`, `await`, `export`, `class`, `abstract`, `extends`, `implements`, `interface`, `trait`, `override`, `readonly`, `new`

Keywords are followed by a negative lookahead for identifier characters, preventing `recipe` from matching `recipeX` or `recipes`.

### 2.6 Literals

#### 2.6.1 Unit Literals

The most distinctive Saffron literal: a numeric value fused with a physical unit.

```
UnitLiteral = Number '.' UnitSuffix
```

Examples: `180.celsius`, `50.ml`, `2.5.cm`, `3.minutes`, `2000.watts`

The number can be integer or float. The dot between the number and unit suffix is syntactic (distinct from a decimal point). For `2.5.cm`, the parser recognizes `2.5` as a float and `.cm` as the unit attachment.

**Available unit suffixes:**

| Category | Suffixes |
|----------|----------|
| Temperature | `celsius`, `fahrenheit`, `kelvin` |
| Mass | `grams`, `kilograms`, `ounces`, `pounds`, `milligrams` |
| Volume | `ml` (milliliters), `liters`, `cups`, `tablespoons`, `teaspoons`, `fluid_ounces` |
| Time | `seconds`, `minutes`, `hours` |
| Length | `cm` (centimeters), `mm` (millimeters), `inches` |
| Energy | `joules`, `calories`, `kilocalories` |
| Power | `watts` |
| Percentage | `percent` |

Abbreviations (`ml`, `cm`, `mm`) are aliases for their full forms and produce the same AST node.

#### 2.6.2 Numeric Literals

Plain integers and floats without units.

```saffron
42          // integer
3.14        // float
1           // integer (used for quantity, etc.)
```

#### 2.6.3 String Literals

Double-quoted with standard escape sequences.

```saffron
"hello"
"version 1.0.0"
"contains \"quotes\""
```

Supported escapes: `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`, `\uXXXX`.

#### 2.6.4 Percent Literals

Two equivalent forms for percentage values:

```saffron
76%             // Shorthand form (preferred for readability)
76.percent      // Unit literal form (canonical)
```

Both produce the same AST representation (`PercentLiteral`). The canonical form for AI generation is `76%` (shorthand); `76.percent` is the unit literal equivalent and is accepted but not preferred.

#### 2.6.5 Boolean Literals

```saffron
true
false
```

#### 2.6.6 Array Literals

Square-bracketed, comma-separated expressions.

```saffron
[yolk, white]
[pasta_drained, cooking_water]
```

### 2.7 Operators and Punctuation

| Token | Symbol | Usage |
|-------|--------|-------|
| Comparison | `==`, `!=`, `<`, `<=`, `>`, `>=` | Value comparison |
| Arrow | `->` | Destructuring output |
| Fat arrow | `=>` | Lambda (Phase 2+) |
| Colon | `:` | Type annotation, named argument |
| Comma | `,` | Separator |
| Dot | `.` | Field access, unit suffix, enum path |
| At | `@` | Annotation prefix |
| Assignment | `=` | Binding (Phase 2+) |
| Parentheses | `(` `)` | Grouping, call arguments |
| Braces | `{` `}` | Blocks |
| Brackets | `[` `]` | Arrays, destructuring |


## 3. Syntactic Grammar

### 3.1 Program Structure

A Saffron program is a sequence of top-level declarations:

```
Program = TopLevelDecl*
TopLevelDecl = ImportDecl | RecipeDecl | FnDecl | TypeDecl
```

Phase 1 supports `RecipeDecl` and `ImportDecl`. `FnDecl` and `TypeDecl` are reserved for Phase 2+.

### 3.2 Recipe Declaration

The core construct. A recipe is a named, typed program that transforms ingredients through a sequence of process steps.

```saffron
recipe FriedEgg {
  @version("1.0.0")
  @difficulty(Difficulty.Easy)
  @servings(1)
  @total_time(5.minutes)

  ingredients {
    egg: Egg(type: .Chicken, quantity: 1)
    oil: SunflowerOil(volume: 50.ml)
    salt: Salt(mass: 2.grams)
  }

  equipment {
    pan: FryingPan(diameter: 24.cm, material: .StainlessSteel)
    stove: GasStove(power: 2000.watts)
  }

  steps {
    1: Heat(pan, to: 180.celsius, using: stove)
    2: Add(oil, to: pan)
    3: WaitUntil(oil.state.temperature >= 170.celsius)
    4: Crack(egg) -> [yolk, white]
    5: Add([yolk, white], to: pan)
    6: Season(salt, on: [yolk, white])
    7: Fry(duration: 3.minutes, target: Doneness.Medium)
    8: Remove([yolk, white], from: pan, using: spatula)
  }

  expected_result: FriedEgg {
    whites: TextureState.Set,
    yolk: TextureState.Runny,
    browning: BrowningLevel.Light,
    seasoning: SeasoningLevel.LightlySalted
  }

  nutrition: auto
}
```

**Block order is fixed.** Annotations come first, then `ingredients`, then `equipment`, then `steps`, then `expected_result`, then `nutrition`. This eliminates structural ambiguity for both the parser and AI generators.

### 3.3 Annotations

Annotations attach structured metadata to a recipe. They use the `@name(value)` syntax with `snake_case` names and always precede the blocks they annotate.

```saffron
@version("1.0.0")              // String value
@difficulty(Difficulty.Easy)    // Enum path value
@servings(1)                    // Numeric value
@total_time(5.minutes)          // Unit literal value
@cuisine(Cuisine.Italian)       // Enum path value
@author("saffron-community")   // String value
@concentration(76%)            // Percent literal value
```

Annotation names are always `snake_case` — this is enforced by the grammar. Annotation values are restricted to literals, percent literals, and enum paths. This ensures they are statically analyzable without evaluating expressions.

### 3.4 Ingredients Block

Each ingredient declaration binds a `snake_case` name to a typed construction with named properties:

```saffron
ingredients {
  egg: Egg(type: .Chicken, quantity: 1)
  steak: Beef(cut: .Ribeye, mass: 300.grams, thickness: 2.5.cm)
  water: Water(volume: 2.liters)
  salt: Salt(mass: 5.grams)
}
```

The type name (PascalCase) is resolved against the Saffron Ingredient Database (SID). All parameters use named syntax for clarity.

### 3.5 Equipment Block

Same syntax as ingredients, but for cooking equipment:

```saffron
equipment {
  pan: FryingPan(diameter: 24.cm, material: .StainlessSteel)
  stove: GasStove(power: 2000.watts)
  colander: Colander()    // Empty params allowed
}
```

### 3.6 Steps Block

Steps are the executable core of a recipe. Each step has an explicit integer index, a process call, and an optional destructuring output.

```saffron
steps {
  1: Heat(pan, to: 180.celsius, using: stove)
  2: Add(oil, to: pan)
  3: WaitUntil(oil.state.temperature >= 170.celsius)
  4: Crack(egg) -> [yolk, white]
  5: Add([yolk, white], to: pan)
}
```

#### 3.6.1 Sequential Steps

The standard step form. The integer index is explicit and 1-based:

```
StepNumber ':' Expression Destructure?
```

#### 3.6.2 Parallel Steps

Multiple sub-steps that can execute concurrently:

```saffron
3: parallel {
  a: Heat(oven, to: 200.celsius)
  b: Season(chicken, with: spice_mix)
}
```

Sub-steps use `snake_case` labels instead of numbers. All sub-steps in a parallel block are independent — they share no data dependencies.

#### 3.6.3 Destructuring

Steps that produce multiple outputs use `->` with array destructuring:

```saffron
4: Crack(egg) -> [yolk, white]
7: Drain(pot, using: colander) -> [pasta_drained, cooking_water]
```

The bound names become available for use in subsequent steps.

### 3.7 Expected Result

Declares the expected outcome as a typed object with property assertions:

```saffron
expected_result: FriedEgg {
  whites: TextureState.Set,
  yolk: TextureState.Runny,
  browning: BrowningLevel.Light,
  seasoning: SeasoningLevel.LightlySalted
}
```

Properties are comma-separated with an optional trailing comma. Each property maps a `snake_case` name to an expression (typically an enum path or unit literal).

### 3.8 Nutrition Declaration

Either computed automatically from the SID or specified manually:

```saffron
nutrition: auto                    // Computed from ingredient data
nutrition: { calories: 350.kilocalories, protein: 12.grams }  // Manual
```


## 4. Expressions

### 4.1 Precedence

Expression precedence from lowest to highest:

1. **Comparison** — `==`, `!=`, `<`, `<=`, `>`, `>=`
2. **Unary** — `-` (negation, e.g. `-18.celsius`)
3. **Chain** — field access (`.`), enum paths
4. **Primary** — literals, calls, identifiers, parenthesized

The precedence hierarchy is intentionally shallow. Complex expressions should be decomposed into named steps, not nested deeply. This is both a language design principle and an AI-friendliness feature — LLMs generate more correct code with flat expression trees.

### 4.2 Comparison Expressions

Binary comparison between two chain expressions:

```saffron
oil.state.temperature >= 170.celsius
water.state.phase == Phase.Liquid
```

Only one comparison per expression (no chaining `a < b < c`). This prevents ambiguity in both parsing and AI generation.

### 4.3 Chain Expressions (Field Access and Enum Paths)

Dot-separated chains serve two purposes:

**Field access** (snake_case chains):
```saffron
oil.state.temperature     // Access nested property
water.state.phase         // Access phase state
```

**Enum paths** (PascalCase chains):
```saffron
Doneness.Medium           // Qualified enum variant
TextureState.Set          // Qualified enum variant
Phase.Liquid              // Qualified enum variant
Cuisine.Italian           // Qualified enum variant
```

The parser produces a uniform chain expression. Semantic analysis distinguishes enum paths from field access based on the types involved.

### 4.4 Call Expressions

Unified syntax for process calls and type constructions:

```
PascalIdent '(' ArgList? ')'
```

**Process calls** (when the identifier matches a known ProcessType):
```saffron
Heat(pan, to: 180.celsius, using: stove)
Fry(duration: 3.minutes, target: Doneness.Medium)
WaitUntil(oil.state.temperature >= 170.celsius)
Crack(egg)
```

**Constructions** (when the identifier is a type name):
```saffron
Egg(type: .Chicken, quantity: 1)
FryingPan(diameter: 24.cm, material: .StainlessSteel)
Colander()
```

#### 4.4.1 Argument Rules

Arguments can be positional or named. **Positional arguments must come before named arguments.**

```saffron
// All named — preferred for constructions
Egg(type: .Chicken, quantity: 1)

// Mixed — common for process calls
Heat(pan, to: 180.celsius, using: stove)
//    ^^^  positional (the target)
//         ^^^^^^^^^^^^^^^^^^^^^^^^^^ named (modifiers)

// All positional — for single-argument processes
Crack(egg)

// Single condition — for temporal processes
WaitUntil(oil.state.temperature >= 170.celsius)
```

The first positional argument to a process call typically identifies the primary target or subject of the operation. Named arguments describe how the process is performed.

### 4.5 Enum Variants

Two forms:

**Shorthand** — type inferred from context:
```saffron
.Chicken          // Inferred: EggType.Chicken
.StainlessSteel   // Inferred: Material.StainlessSteel
.Silicone         // Inferred: Material.Silicone
```

**Qualified** — explicit enum type:
```saffron
Doneness.Medium
Phase.Liquid
TextureState.Set
Difficulty.Easy
```

Shorthand variants are preferred inside named arguments where the expected type is known from the parameter type.


## 5. Type System

### 5.1 Physical Units

Saffron's type system enforces dimensional analysis. Each unit literal carries a physical dimension (temperature, mass, volume, time, length, energy, power). The type checker prevents:

**Unit mismatch:**
```saffron
// ERROR E1001: Type mismatch — cannot compare Fahrenheit with Celsius
WaitUntil(oil.state.temperature >= 180.celsius)  // if pan was heated in Fahrenheit
```

**Dimensional incompatibility:**
```saffron
// ERROR: Cannot assign mass to a volume parameter
Water(volume: 200.grams)  // grams is mass, not volume
```

### 5.2 Ingredient Types

Ingredients are typed objects with properties from the SID. Each ingredient type has a category (Protein, Fat, Carbohydrate, Liquid, Seasoning, Produce, Dairy) that determines which processes can be applied to it.

### 5.3 State Types

Saffron tracks the physical state of ingredients through processes:

| Type | Values |
|------|--------|
| `Doneness` | `Raw`, `Rare`, `MediumRare`, `Medium`, `MediumWell`, `WellDone`, `Charred` |
| `Phase` | `Solid`, `Liquid`, `Gas`, `Gel`, `Foam`, `Emulsion` |
| `TextureState` | `Set`, `Runny`, `AlDente`, `Crispy`, `Tender`, etc. |
| `BrowningLevel` | `None`, `Light`, `Medium`, `Dark`, `Burnt` |
| `SeasoningLevel` | `Unseasoned`, `LightlySalted`, `Salted`, `WellSeasoned`, `Overseasoned` |
| `Difficulty` | `Easy`, `Medium`, `Hard`, `Expert` |


## 6. Semantic Rules

The semantic analyzer enforces physical plausibility rules that go beyond type checking:

### 6.1 Process-Ingredient Compatibility

Not all processes can be applied to all ingredients. The semantic analyzer validates against a compatibility matrix derived from food science:

```saffron
// ERROR E2001: Cannot apply Process.Fry to Ingredient.Water
// Water has no solid structure to undergo frying.
Fry(water, duration: 3.minutes, target: Doneness.Medium)
```

### 6.2 Temperature Safety

The analyzer checks temperature values against physically meaningful ranges:

- Water boils at 100°C at sea level — boiling checks above this are warnings
- Protein denaturation ranges are validated against known thresholds
- Equipment temperature limits are checked against specifications

### 6.3 Step Ordering

Steps that reference variables from destructuring must appear after the step that produces those variables:

```saffron
4: Crack(egg) -> [yolk, white]      // produces yolk, white
5: Add([yolk, white], to: pan)      // OK: uses yolk, white from step 4
```

### 6.4 Ingredient Availability

Every ingredient referenced in steps must be declared in the `ingredients` block. Every piece of equipment referenced must be in the `equipment` block.


## 7. Error Codes

| Code | Category | Description |
|------|----------|-------------|
| E1001 | Type | Unit mismatch (e.g., comparing Celsius with Fahrenheit) |
| E1002 | Type | Dimensional incompatibility (e.g., mass where volume expected) |
| E1003 | Type | Unknown type reference |
| E1004 | Type | Invalid enum variant for type |
| E2001 | Semantic | Invalid process-ingredient combination |
| E2002 | Semantic | Undefined ingredient reference in steps |
| E2003 | Semantic | Undefined equipment reference in steps |
| E2004 | Semantic | Variable used before destructuring |
| E2005 | Semantic | Duplicate step number |
| E2006 | Semantic | Duplicate ingredient name |
| E2007 | Semantic | Duplicate equipment name |
| E3001 | Physics | Temperature out of physical range |
| E3002 | Physics | Duration negative or zero |
| E3003 | Physics | Incompatible state transition |


## 8. Complete Example

```saffron
// Saffron Recipe: Grilled Steak
// Demonstrates protein cooking with precise temperature control.

///ai: complexity=moderate requires="thermometer for accuracy"

recipe GrilledSteak {
  @version("1.0.0")
  @author("saffron-community")
  @difficulty(Difficulty.Medium)
  @servings(1)
  @total_time(25.minutes)
  @cuisine(Cuisine.Universal)

  ingredients {
    steak: Beef(cut: .Ribeye, mass: 300.grams, thickness: 2.5.cm)
    salt: Salt(mass: 5.grams)
    pepper: BlackPepper(mass: 2.grams)
    oil: OliveOil(volume: 15.ml)
  }

  equipment {
    grill: Grill(type: .Charcoal)
    thermometer: Thermometer(type: .Instant)
    tongs: Tongs(material: .StainlessSteel)
  }

  steps {
    ///ai: critical_for=flavor reason="salt needs time to penetrate"
    1: Season(salt, on: steak)
    2: Season(pepper, on: steak)
    3: Rest(steak, duration: 20.minutes)

    ///ai: critical_for=sear reason="grill must be very hot for Maillard reaction"
    4: Heat(grill, to: 230.celsius, using: grill)
    5: Add(oil, to: steak)
    6: Add(steak, to: grill)
    7: Grill(duration: 4.minutes, target: Doneness.MediumRare)

    ///ai: critical_for=food_quality reason="resting redistributes juices"
    8: Remove(steak, from: grill, using: tongs)
    9: Rest(steak, duration: 5.minutes)
  }

  expected_result: GrilledSteak {
    doneness: Doneness.MediumRare,
    internal_temp: 57.celsius,
    crust: BrowningLevel.Dark,
    seasoning: SeasoningLevel.WellSeasoned
  }

  nutrition: auto
}
```


## 9. Grammar Reference

The formal PEG grammar is defined in [`grammar.peg`](grammar.peg). Key parsing rules:

| Construct | Grammar Rule | Example |
|-----------|-------------|---------|
| Recipe | `RecipeDecl` | `recipe FriedEgg { ... }` |
| Annotation | `Annotation` | `@version("1.0.0")` |
| Ingredient | `ItemDecl` | `egg: Egg(type: .Chicken)` |
| Step | `SequentialStep` | `1: Heat(pan, to: 180.celsius)` |
| Unit literal | `UnitLiteral` | `180.celsius`, `2.5.cm` |
| Process call | `CallExpr` | `Heat(pan, to: 180.celsius)` |
| Enum variant | `EnumVariant` | `.Chicken` |
| Enum path | `EnumPath` / `ChainExpr` | `Doneness.Medium` |
| Field access | `ChainExpr` | `oil.state.temperature` |
| Destructure | `Destructure` | `-> [yolk, white]` |
| Comparison | `Comparison` | `temp >= 170.celsius` |
| Array | `ArrayLiteral` | `[yolk, white]` |
| AI hint | `AiHint` | `///ai: suggest_alternative=vegan` |


## Appendix A: Comparison with Existing Formats

| Feature | Saffron | JSON Recipe | Markdown Recipe |
|---------|---------|-------------|-----------------|
| Type safety | Compile-time | None | None |
| Unit validation | Built-in | Manual | Manual |
| Physical constraints | Enforced | None | None |
| AI generation | Optimized | Good | Poor (ambiguous) |
| Machine execution | Native | Requires schema | Not possible |
| Human readability | High | Low | High |
| Formal verification | Possible | Limited | Not possible |


## Appendix B: Roadmap

| Phase | Focus | Key Deliverables |
|-------|-------|-----------------|
| 0 | Foundation | Grammar spec ✓, AST ✓, Lexer ✓, SID schema ✓ |
| 1 | Compiler MVP | Parser, type checker, semantic analyzer |
| 2 | Runtime | Code generator, VM, physics simulation |
| 3 | Ingredient DB | 500+ validated ingredients in SID |
| 4 | Tooling | CLI, LSP, playground, documentation site |
| 5 | AI Integration | JSON schema, generation prompts, benchmarks |
| 6 | Launch | Community, academic paper, v1.0 release |
