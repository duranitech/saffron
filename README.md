# Saffron

**The Culinary Programming Language**

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Status: Pre-Alpha](https://img.shields.io/badge/status-pre--alpha-red.svg)]()

Saffron is a strongly-typed, object-oriented, domain-specific programming language designed to formalize culinary processes as computable, verifiable, and machine-executable programs.

## What is Saffron?

Saffron models ingredients, utensils, heat sources, and transformations as first-class typed objects. It enables:

- **Standardized recipes**: One canonical way to express every culinary operation
- **Physical simulation**: Real thermodynamic and chemical models (heat transfer, Maillard reaction, protein denaturation)
- **Type safety**: Impossible to mix Celsius with Fahrenheit, or fry water
- **AI-first design**: Optimized for AI agents to generate, validate, and iterate on recipes
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
  }

  expected_result: FriedEgg {
    whites: TextureState.Set,
    yolk: TextureState.Runny,
    browning: BrowningLevel.Light
  }
}
```

## Project Structure

```
saffron/
├── crates/
│   ├── saffron-ast/        # Abstract Syntax Tree definitions
│   ├── saffron-lexer/      # Tokenizer
│   ├── saffron-parser/     # Recursive descent parser
│   ├── saffron-typeck/     # Type checker
│   ├── saffron-semantic/   # Domain-specific semantic analysis
│   ├── saffron-codegen/    # Code generator (bytecode, markdown, machine)
│   ├── saffron-vm/         # Virtual machine and runtime
│   ├── saffron-physics/    # Physical/chemical simulation engine
│   ├── saffron-sid/        # Saffron Ingredient Database client
│   └── saffron-cli/        # Command-line interface
├── sid/                    # Ingredient database (JSON)
├── tests/                  # Fixture-based test suite
├── docs/                   # Documentation (Docusaurus)
├── playground/             # Web-based editor (React + WASM)
├── recipes/                # Community recipe library
├── rfcs/                   # Language change proposals
└── spec/                   # Saffron Language Specification
```

## Building

```bash
# Prerequisites: Rust 1.75+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build all crates
cargo build

# Run tests
cargo test

# Run the CLI
cargo run --bin saffron -- --help
```

## Design Principles

1. **Correctness by Construction**: If it compiles, it's physically plausible
2. **One Canonical Form**: Exactly one way to express each operation
3. **Declarative Intent**: Describe what, not how
4. **Physical Fidelity**: Real science, not approximation
5. **AI-First Ergonomics**: Optimized for machine code generation

## Documentation

- [Saffron Language Specification (SLS)](spec/) - The authoritative language reference
- [Saffron Ingredient Database Schema](sid/schema/) - Data format for ingredients
- [Contributing Guide](CONTRIBUTING.md) - How to contribute

## Current Status

**Pre-alpha** — We are building the foundational compiler and runtime. See the [Master Development Plan](spec/) for the full roadmap.

### Roadmap

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 0: Foundation | 🔨 In Progress | Grammar, infrastructure, schemas |
| Phase 1: Compiler | 📋 Planned | Lexer, parser, type checker |
| Phase 2: Runtime | 📋 Planned | VM, physics engine |
| Phase 3: Ingredient DB | 📋 Planned | 500 validated ingredients |
| Phase 4: Tooling | 📋 Planned | CLI, LSP, playground, docs |
| Phase 5: AI Integration | 📋 Planned | JSON schema, prompts, benchmarks |
| Phase 6: Launch | 📋 Planned | Community, paper, v1.0 |

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

Areas where help is especially needed:
- **Food scientists**: Validate physical/chemical models and ingredient data
- **Compiler engineers**: Help build the lexer, parser, and type checker
- **Rust developers**: Runtime and tooling implementation
- **Chefs**: Test recipes and suggest domain improvements
- **AI/ML engineers**: Prompt engineering and benchmark design

## License

Apache License 2.0 — see [LICENSE](LICENSE) for details.

---

*Saffron: Where code meets cuisine.*
