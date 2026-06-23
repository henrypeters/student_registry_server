use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Role {
    Administrator,
    Staff,
    Student,
    None
}

impl Role {
    pub fn as_str(&self) -> &str {
        match self {
            Role::Administrator => "Administrator",
            Role::Staff => "Staff",
            Role::Student => "Student",
            Role::None => "Invalid option"
        }
    }

    pub fn map_int_to_role(int: u8) -> Self{
        match int {
            1 => Self::Administrator,
            2 => Self::Staff,
            3 => Self::Student,
            _ => Self::None,
        }
    }
}
