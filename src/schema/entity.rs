use crate::schema::sex::Sex;
use crate::schema::grade::Grade;
use crate::schema::role::Role;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: Uuid,
    pub name: String, 
    pub age: u8,
    pub sex: Sex,
    pub grade: Grade, 
    pub role: Role
}

impl Entity {
    pub fn new(name: String, age: u8, sex: Sex, grade: Grade, role: Role) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            age,
            sex,
            grade,
            role
        }
    }
}
