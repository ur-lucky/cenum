use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use cenum::{build, Solver};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a starter cenum.yaml config file.
    Init {
        /// Path to write the YAML config file.
        #[arg(default_value = "cenum.yaml")]
        config: PathBuf,

        /// Overwrite the config file if it already exists.
        #[arg(long)]
        force: bool,
    },

    /// Compile YAML enum definitions into a Luau module.
    Build {
        /// Path to the YAML config file.
        #[arg(default_value = "cenum.yaml")]
        config: PathBuf,

        /// Override the generated Luau output path.
        #[arg(long)]
        output: Option<PathBuf>,

        /// Override the solver output style.
        #[arg(long, value_enum)]
        solver: Option<Solver>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { config, force } => {
            let config_path = cenum::init_config(&config, force)?;
            println!("Created {}", config_path.display());
        }
        Commands::Build {
            config,
            output,
            solver,
        } => {
            let overrides = cenum::BuildOverrides {
                output,
                solver,
            };

            let output_path = build(&config, overrides)?;
            println!("Generated {}", output_path.display());
        }
    }

    Ok(())
}
