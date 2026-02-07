//! # Saffron CLI
//!
//! Command-line interface for the Saffron programming language.
//!
//! Usage:
//!   saffron compile <file>     Compile a .saffron file
//!   saffron run <file>         Compile and execute a recipe
//!   saffron check <file>       Type-check without compiling
//!   saffron simulate <file>    Run simulation with output
//!   saffron validate <file>    Validate against SLS
//!   saffron fmt <file>         Format source code
//!   saffron new <name>         Scaffold new recipe
//!   saffron ingredient <name>  Query SID
//!   saffron nutrition <file>   Compute nutrition facts

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "saffron")]
#[command(version = "0.1.0")]
#[command(about = "The Saffron Culinary Programming Language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a .saffron file to bytecode
    Compile {
        /// Path to the .saffron source file
        file: String,
        /// Output path for bytecode
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Compile and execute a recipe
    Run {
        /// Path to the .saffron source file
        file: String,
        /// Show verbose simulation output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Type-check without compiling
    Check {
        /// Path to the .saffron source file
        file: String,
    },
    /// Run and show step-by-step simulation
    Simulate {
        /// Path to the .saffron source file
        file: String,
        /// Time step in milliseconds
        #[arg(long, default_value = "100")]
        dt: u32,
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Format source code
    Fmt {
        /// Path to the .saffron source file
        file: String,
        /// Check formatting without modifying
        #[arg(long)]
        check: bool,
    },
    /// Scaffold a new recipe
    New {
        /// Recipe name (PascalCase)
        name: String,
    },
    /// Query the Saffron Ingredient Database
    Ingredient {
        /// Ingredient name or ID to look up
        name: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Compute nutrition facts for a recipe
    Nutrition {
        /// Path to the .saffron source file
        file: String,
    },
    /// Export recipe to another format
    Export {
        /// Path to the .saffron source file
        file: String,
        /// Output format: md, json, sfmi
        #[arg(short, long, default_value = "md")]
        format: String,
    },
}

fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { file, output } => {
            println!("Compiling {}...", file);
            println!("TODO: Implement compilation pipeline (Phase 1)");
        }
        Commands::Run { file, verbose } => {
            println!("Running {}...", file);
            println!("TODO: Implement run pipeline (Phase 2)");
        }
        Commands::Check { file } => {
            println!("Checking {}...", file);
            println!("TODO: Implement type checking (Phase 1)");
        }
        Commands::Simulate { file, dt, verbose } => {
            println!("Simulating {} (dt={}ms)...", file, dt);
            println!("TODO: Implement simulation (Phase 2)");
        }
        Commands::Fmt { file, check } => {
            println!("Formatting {}...", file);
            println!("TODO: Implement formatter (Phase 4)");
        }
        Commands::New { name } => {
            println!("Creating new recipe: {}", name);
            println!("TODO: Implement scaffolding (Phase 4)");
        }
        Commands::Ingredient { name, json } => {
            println!("Looking up ingredient: {}", name);
            println!("TODO: Implement SID query (Phase 3)");
        }
        Commands::Nutrition { file } => {
            println!("Computing nutrition for {}...", file);
            println!("TODO: Implement nutrition calculator (Phase 3)");
        }
        Commands::Export { file, format } => {
            println!("Exporting {} to {}...", file, format);
            println!("TODO: Implement export (Phase 4)");
        }
    }
}
