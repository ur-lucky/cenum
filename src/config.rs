use crate::model::{EnumDef, Solver};
use anyhow::{bail, Context, Result};
use indexmap::IndexMap;
use serde_yaml::Value;
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RawConfig {
    pub output: Option<PathBuf>,
    pub solver: Option<Solver>,
    pub use_const: Option<bool>,
    pub enums: Vec<EnumDef>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BuildOverrides {
    pub output: Option<PathBuf>,
    pub solver: Option<Solver>,
    pub use_const: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuildOptions {
    pub output: PathBuf,
    pub solver: Solver,
    pub use_const: bool,
    pub enums: Vec<EnumDef>,
}

impl BuildOptions {
    pub fn resolve(raw: RawConfig, overrides: BuildOverrides) -> Result<Self> {
        let output = overrides
            .output
            .or(raw.output)
            .context("missing output path; set `output` in YAML or pass `--output`")?;

        Ok(Self {
            output,
            solver: overrides.solver.or(raw.solver).unwrap_or(Solver::Old),
            use_const: overrides.use_const.or(raw.use_const).unwrap_or(false),
            enums: raw.enums,
        })
    }

    pub fn declaration_keyword(&self) -> &'static str {
        if self.use_const {
            "const"
        } else {
            "local"
        }
    }
}

pub fn parse_yaml(input: &str) -> Result<RawConfig> {
    let value: Value = serde_yaml::from_str(input)?;
    let Value::Mapping(root) = value else {
        bail!("config root must be a YAML mapping");
    };

    let mut raw = RawConfig::default();
    let mut top_level_enums = IndexMap::<String, Vec<String>>::new();

    for (key, value) in root {
        let key = string_key(&key)?;

        match key.as_str() {
            "output" => raw.output = Some(PathBuf::from(string_value(&value, "output")?)),
            "solver" => {
                let solver = string_value(&value, "solver")?;
                raw.solver = Some(
                    Solver::parse(&solver)
                        .with_context(|| format!("invalid solver `{solver}`; expected `old` or `new`"))?,
                );
            }
            "use-const" | "use_const" => {
                raw.use_const = Some(bool_value(&value, &key)?);
            }
            "enums" => {
                let enums = enum_map(&value, "enums")?;
                for (name, items) in enums {
                    push_unique_enum(&mut raw.enums, name, items)?;
                }
            }
            _ => {
                let items = enum_items(&value, &key)?;
                top_level_enums.insert(key, items);
            }
        }
    }

    for (name, items) in top_level_enums {
        push_unique_enum(&mut raw.enums, name, items)?;
    }

    Ok(raw)
}

fn push_unique_enum(enums: &mut Vec<EnumDef>, name: String, items: Vec<String>) -> Result<()> {
    if enums.iter().any(|existing| existing.name == name) {
        bail!("duplicate enum `{name}`");
    }

    enums.push(EnumDef { name, items });
    Ok(())
}

fn enum_map(value: &Value, path: &str) -> Result<IndexMap<String, Vec<String>>> {
    let Value::Mapping(mapping) = value else {
        bail!("`{path}` must be a mapping of enum names to item arrays");
    };

    let mut enums = IndexMap::new();
    for (key, value) in mapping {
        let name = string_key(key)?;
        let items = enum_items(value, &format!("{path}.{name}"))?;
        if enums.insert(name.clone(), items).is_some() {
            bail!("duplicate enum `{name}`");
        }
    }

    Ok(enums)
}

fn enum_items(value: &Value, path: &str) -> Result<Vec<String>> {
    let Value::Sequence(sequence) = value else {
        bail!("`{path}` must be an array of enum item strings");
    };

    sequence
        .iter()
        .enumerate()
        .map(|(index, value)| string_value(value, &format!("{path}[{index}]")))
        .collect()
}

fn string_key(value: &Value) -> Result<String> {
    match value {
        Value::String(value) => Ok(value.clone()),
        _ => bail!("mapping keys must be strings"),
    }
}

fn string_value(value: &Value, path: &str) -> Result<String> {
    match value {
        Value::String(value) => Ok(value.clone()),
        _ => bail!("`{path}` must be a string"),
    }
}

fn bool_value(value: &Value, path: &str) -> Result<bool> {
    match value {
        Value::Bool(value) => Ok(*value),
        _ => bail!("`{path}` must be a boolean"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_canonical_yaml() {
        let raw = parse_yaml(
            r#"
output: src/shared/Enums.luau
solver: new
use-const: true
enums:
  TransactionType:
    - Robux
    - Tickets
"#,
        )
        .unwrap();

        assert_eq!(raw.output, Some(PathBuf::from("src/shared/Enums.luau")));
        assert_eq!(raw.solver, Some(Solver::New));
        assert_eq!(raw.use_const, Some(true));
        assert_eq!(raw.enums[0].name, "TransactionType");
        assert_eq!(raw.enums[0].items, vec!["Robux", "Tickets"]);
    }

    #[test]
    fn parses_enums_only_yaml() {
        let raw = parse_yaml(
            r#"
TransactionStatus:
  - Pending
  - Completed
"#,
        )
        .unwrap();

        assert_eq!(raw.enums[0].name, "TransactionStatus");
        assert!(raw.output.is_none());
    }

    #[test]
    fn missing_output_is_an_error() {
        let raw = parse_yaml(
            r#"
enums:
  TransactionStatus:
    - Pending
"#,
        )
        .unwrap();

        let error = BuildOptions::resolve(raw, BuildOverrides::default()).unwrap_err();
        assert!(error.to_string().contains("missing output path"));
    }
}
