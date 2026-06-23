use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
    None,
}

impl Sex {
    pub fn map_int_to_enum(int: u8) -> Self{
        match int {
            1 => Sex::Male,
            2 => Sex::Female,
            _ => Sex::None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Sex::Male => "Male",
            Sex::Female => "Female",
            Sex::None => "None",
        }
    }
}
