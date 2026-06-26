use clap::ValueEnum;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum Solver {
    Old,
    New,
}

impl Solver {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "old" => Some(Self::Old),
            "new" => Some(Self::New),
            _ => None,
        }
    }
}

impl fmt::Display for Solver {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Solver::Old => formatter.write_str("old"),
            Solver::New => formatter.write_str("new"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnumDef {
    pub name: String,
    pub items: Vec<String>,
}
