use crate::config::BuildOptions;
use anyhow::{bail, Result};
use std::collections::HashSet;

const LUAU_KEYWORDS: &[&str] = &[
    "and",
    "break",
    "do",
    "else",
    "elseif",
    "end",
    "false",
    "for",
    "function",
    "if",
    "in",
    "local",
    "nil",
    "not",
    "or",
    "repeat",
    "return",
    "then",
    "true",
    "until",
    "while",
    "type",
    "export",
    "continue",
];

const RESERVED_ENUM_NAMES: &[&str] = &["EnumSet", "EnumItem"];
const RESERVED_ITEM_NAMES: &[&str] = &["FromName", "FromValue", "GetEnumItems"];

pub fn validate_options(options: &BuildOptions) -> Result<()> {
    if options.enums.is_empty() {
        bail!("at least one enum must be defined");
    }

    let mut enum_names = HashSet::new();
    for enum_def in &options.enums {
        validate_identifier(&enum_def.name, "enum")?;

        if enum_def.name.starts_with("__CEnum") || RESERVED_ENUM_NAMES.contains(&enum_def.name.as_str()) {
            bail!("enum name `{}` is reserved", enum_def.name);
        }

        if !enum_names.insert(enum_def.name.as_str()) {
            bail!("duplicate enum `{}`", enum_def.name);
        }

        if enum_def.items.is_empty() {
            bail!("enum `{}` must define at least one item", enum_def.name);
        }

        let mut item_names = HashSet::new();
        for item in &enum_def.items {
            validate_identifier(item, &format!("item in enum `{}`", enum_def.name))?;

            if item.starts_with("__CEnum") || RESERVED_ITEM_NAMES.contains(&item.as_str()) {
                bail!("enum item `{}.{}` is reserved", enum_def.name, item);
            }

            if !item_names.insert(item.as_str()) {
                bail!("duplicate enum item `{}.{}`", enum_def.name, item);
            }
        }
    }

    Ok(())
}

fn validate_identifier(value: &str, label: &str) -> Result<()> {
    if value.is_empty() {
        bail!("{label} name cannot be empty");
    }

    let mut chars = value.chars();
    let first = chars.next().expect("checked non-empty");
    if !(first == '_' || first.is_ascii_alphabetic()) {
        bail!("{label} name `{value}` must start with a letter or underscore");
    }

    if !chars.all(|character| character == '_' || character.is_ascii_alphanumeric()) {
        bail!("{label} name `{value}` must contain only letters, numbers, and underscores");
    }

    if LUAU_KEYWORDS.contains(&value) {
        bail!("{label} name `{value}` is a Luau keyword");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::BuildOptions;
    use crate::model::{EnumDef, Solver};
    use std::path::PathBuf;

    fn options(enums: Vec<EnumDef>) -> BuildOptions {
        BuildOptions {
            output: PathBuf::from("out.luau"),
            solver: Solver::Old,
            use_const: false,
            enums,
        }
    }

    #[test]
    fn rejects_duplicate_items() {
        let error = validate_options(&options(vec![EnumDef {
            name: "TransactionStatus".to_owned(),
            items: vec!["Pending".to_owned(), "Pending".to_owned()],
        }]))
        .unwrap_err();

        assert!(error.to_string().contains("duplicate enum item"));
    }

    #[test]
    fn rejects_reserved_names() {
        let error = validate_options(&options(vec![EnumDef {
            name: "__CEnumRuntime".to_owned(),
            items: vec!["Pending".to_owned()],
        }]))
        .unwrap_err();

        assert!(error.to_string().contains("reserved"));
    }
}
