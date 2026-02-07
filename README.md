# Saffron

**The Culinary Programming Language**

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)
[![Status: Pre-Alpha](https://img.shields.io/badge/status-pre--alpha-red.svg)]()

Saffron is a strongly-typed, domain-specific programming language designed to formalize culinary processes as computable, verifiable, and machine-executable programs. It is optimized for AI code generation, physical simulation, and robotic execution.

## What is Saffron?

Saffron models ingredients, equipment, heat sources, and transformations as first-class typed objects. Every recipe is a program; every process call is a physically constrained operation.

- **Type-safe cooking**: Unit literals (`180.celsius`, `50.ml`) are first-class tokens — impossible to mix Celsius with Fahrenheit, or fry water
- **Closed process set**: 56 culinary process types (thermal, mechanical, chemical, temporal) — the language knows exactly what operations exist
- **Physical fidelity**: Real thermodynamic and chemical models (heat transfer, Maillard reaction, protein denaturation)
- **AI-first design**: Fixed block order, deterministic PEG grammar, named arguments — optimized for LLM code generation and validation
- **Machine execution**: Compile recipes to instructions for autonomous cooking robots

## Example

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
    browning: BrowningLevel.Light
  }

  nutrition: auto
}
```

## Language Highlights

| Feature | Description |
|---------|-------------|
| Unit literals | `180.celsius`, `2.5.cm`, `50.ml` — 25 unit types + 3 abbreviations |
| Identifier casing | `PascalCase` for types, `snake_case` for variables, `SCREAMING_CASE` for constants |
| Comments | `//` regular, `///` doc, `///ai:` AI hints for code generators |
| Process types | 56 closed-set culinary operations: `Fry`, `Boil`, `Crack`, `Marinate`, etc. |
| Destructuring | `Crack(egg) -> [yolk, white]` |
| Parallel steps | `parallel { a: Heat(pan), b: Season(steak) }` |
| Annotations | `@version("1.0.0")`, `@difficulty(Difficulty.Easy)`, `@servings(4)` |

## Project Structure

```
saffron/
├── crates/
│   ├── saffron-ast/        # Abstract Syntax Tree definitions
│   ├── saffron-lexer/      # Hand-written tokenizer (44 tests)
│   ├── saffron-parser/     # Recursive descent parser
│   ├── saffron-typeck/     # Type checker (dimensional analysis)
│   ├── saffron-semantic/   # Domain-specific semantic analysis
│   ├── saffron-codegen/    # Code generator (bytecode, markdown, machine)
│   ├── saffron-vm/         # Virtual machine and runtime
│   ├── saffron-physics/    # Physical/chemical simulation engine
│   ├── saffron-sid/        # Saffron Ingredient Database client
│   └── saffron-cli/        # Command-line interface
├── spec/                   # PEG grammar + language specification
│   ├── grammar.peg         # Formal PEG grammar (~840 lines)
│   └── LANGUAGE.md         # Human-readable language spec
├── tests/                  # Fixture-based test suite (.saffron files)
├── docs/                   # Documentation
├── recipes/                # Community recipe library
└── rfcs/                   # Language change proposals
```

## Building

```bash
# Prerequisites: Rust 1.85+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build all crates
cargo build

# Run tests
cargo test

# Run linter
cargo clippy --all-targets -- -D warnings

# Run the CLI (coming soon)
cargo run --bin saffron -- --help
```

## Design Principles

1. **Correctness by Construction** — If it compiles, it's physically plausible
2. **One Canonical Form** — Exactly one way to express each operation
3. **Declarative Intent** — Describe what, not how
4. **Physical Fidelity** — Real science, not approximation
5. **AI-First Ergonomics** — Deterministic grammar, fixed block order, named arguments

## Current Status

**Pre-alpha** — We are building the foundational compiler pipeline.

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 0: Foundation | ✅ Complete | PEG grammar, AST, project infrastructure |
| Phase 1: Compiler MVP | 🔨 In Progress | Lexer ✅, Parser, Type Checker, Semantic Analyzer |
| Phase 2: Runtime | 📋 Planned | VM, physics engine |
| Phase 3: Ingredient DB | 📋 Planned | 500 validated ingredients |
| Phase 4: Tooling | 📋 Planned | CLI, LSP, playground, docs |
| Phase 5: AI Integration | 📋 Planned | JSON schema, prompts, benchmarks |
| Phase 6: Launch | 📋 Planned | Community, paper, v1.0 |

### What's implemented

- **Formal grammar**: Complete PEG grammar (`spec/grammar.peg`) with AI generation guidelines
- **Language spec**: Full human-readable specification (`spec/LANGUAGE.md`)
- **AST**: Complete type definitions for all language constructs (25 unit types, 56 process types, 7 ingredient categories)
- **Lexer**: Hand-written zero-copy tokenizer with unit literal support, identifier casing enforcement, error recovery, and 44 passing tests including fixture snapshots

## Documentation

- [PEG Grammar](spec/grammar.peg) — Formal grammar definition
- [Language Specification](spec/LANGUAGE.md) — Human-readable language reference
- [Contributing Guide](CONTRIBUTING.md) — How to contribute

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

Areas where help is especially needed:

- **Food scientists**: Validate physical/chemical models and ingredient data
- **Compiler engineers**: Help build the parser, type checker, and semantic analyzer
- **Rust developers**: Runtime and tooling implementation
- **Chefs**: Test recipes and suggest domain improvements
- **AI/ML engineers**: Prompt engineering and benchmark design

## License

Apache License 2.0 — see [LICENSE](LICENSE) for details.

---

*Saffron: Where code meets cuisine.*
