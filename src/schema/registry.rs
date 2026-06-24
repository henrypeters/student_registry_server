

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::entity::Entity;
use crate::schema::role::Role;
use crate::schema::sex::Sex;
use crate::schema::grade::Grade;

use crate::utils::util::{load_storage, save_data};


#[derive(Deserialize)]
pub struct CreateNewRegistry {
    pub name: String,
    pub age: u8,
    pub sex: u8
}

#[derive(Deserialize)]
pub struct AddStudent {
    pub name: String,
    pub age: u8,
    pub sex: u8,
    pub grade: u8
}

#[derive(Deserialize)]
pub struct GetAndDeleteStudentById {
    pub id: Uuid
}

#[derive(Deserialize)]
pub struct ChangeStudentGrade {
    pub id: Uuid,
    pub grade: u8
}

#[derive(Serialize)]
pub struct Registry {
    pub entities: Vec<Entity>,
}

impl Registry {
    pub fn init(name: String, age: u8, sex: u8) -> Self {
        let admin = Entity::new(name, age, Sex::map_int_to_enum(sex), Grade::None, Role::Administrator);

        let entities = vec![admin];

        Self { 
            entities: entities
        }
    }

    pub fn all_entities(&self) -> Vec<Entity> {
       let entities = &self.entities;

       entities.clone() 
    }

    //////// STUDENTS

    pub fn add_student(&mut self, student: Entity) -> Result<(), String> {
        // let student = Entity::new(name.to_string(), age, sex, grade, Role::Student);
        self.entities.push(student);
        match save_data(&self.entities) {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("{}", e))
        }
    }

    pub fn list_all_students(&self) -> Result<Vec<Entity>, String> {
        let file_storage = load_storage();
        
        let ref_student_vec: Vec<&Entity> = file_storage.iter().filter(|x| x.role == Role::Student).collect();
        // let y = checkout.collect();
        if ref_student_vec.is_empty() {
            return Err("No students in registry".to_string());
        }

        let owned_student_vec:Vec<Entity> = ref_student_vec.into_iter().cloned().collect();

        Ok(owned_student_vec)
        
    }

    pub fn get_student_by_id(&self, id: Uuid) -> Option<Entity> {
        let file_storage = load_storage();

        let student_option = file_storage.into_iter()
                                            .find(|student| student.id == id && student.role == Role::Student);
                                    
        match student_option {
            Some(student) => {
                Some(student.clone())
            },
            None => None
        }   

    }

    pub fn change_student_grade(&mut self, id: Uuid, grade: u8) -> std::io::Result<Entity> {
        let file_storage = load_storage();

        let new_grade = Grade::map_int_to_grade(grade);
        let mut student = match file_storage.iter().find(|student| student.id == id && student.grade != new_grade && student.role == Role::Student) {
            Some(student) => student.clone(),
            None => return Err(std::io::Error::other("Coundn't find student"))
        };

        student.grade = new_grade;
        
        save_data(&file_storage)?;

        Ok(student)
    }

    pub fn remove_student(&mut self, id: Uuid) -> std::io::Result<Entity>{
        let mut file_storage = load_storage();

        let index = match file_storage.iter().position(|student| student.id == id && student.role == Role::Student) {
            Some(index) => index,
            None => return Err(std::io::Error::other("Couldn't find student index"))
        };

        let removed_element = file_storage.remove(index);

        save_data(&file_storage)?;

        Ok(removed_element)
    }



    //////////// STAFFS
    
}