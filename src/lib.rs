mod config;
mod generate;
mod model;
mod validate;

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub use config::{BuildOptions, BuildOverrides};
pub use model::{EnumDef, Solver};

pub fn build(config_path: &Path, overrides: BuildOverrides) -> Result<PathBuf> {
    let contents = fs::read_to_string(config_path)
        .with_context(|| format!("failed to read config {}", config_path.display()))?;
    let raw_config = config::parse_yaml(&contents)
        .with_context(|| format!("failed to parse config {}", config_path.display()))?;
    let options = BuildOptions::resolve(raw_config, overrides)?;

    validate::validate_options(&options)?;

    let output = generate::generate(&options);

    if let Some(parent) = options.output.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create output directory {}", parent.display()))?;
        }
    }

    fs::write(&options.output, output)
        .with_context(|| format!("failed to write output {}", options.output.display()))?;

    Ok(options.output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_defaults_and_overrides() {
        let raw = config::parse_yaml(
            r#"
output: generated.luau
enums:
  TransactionStatus:
    - Pending
"#,
        )
        .unwrap();

        let options = BuildOptions::resolve(
            raw,
            BuildOverrides {
                output: Some(PathBuf::from("override.luau")),
                solver: Some(Solver::New),
                use_const: Some(true),
            },
        )
        .unwrap();

        assert_eq!(options.output, PathBuf::from("override.luau"));
        assert_eq!(options.solver, Solver::New);
        assert!(options.use_const);
    }
}
