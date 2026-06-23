use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Grade {
    First,
    Second,
    Third,
    None
}

impl Grade {
    pub fn as_str(&self) -> &str {
        match self {
            Grade::First => "Cohort 1",
            Grade::Second => "Cohort 2",
            Grade::Third => "Cohort 3",
            Grade::None => "Cohort 4",
        }
    }

    pub fn map_int_to_grade(int: u8) -> Self{
        match int {
            1 => Self::First,
            2 => Self::Second,
            3 => Self::Third,
            _ => Self::None,
        }
    }
}