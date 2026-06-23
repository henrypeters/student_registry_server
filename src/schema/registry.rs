// use axum::http::status;
// use serde::{Deserialize, Serialize};
// use uuid::Uuid;

// #[derive(Deserialize)]
// pub struct CreateTodoPayload {
//     pub name: String
// }

// #[derive(Deserialize)]
// pub struct GetTodoPayload {
//     pub id: Uuid
// }

// #[derive(Deserialize)]
// pub struct UpdateTodoPayload {
//     pub id: Uuid,
//     pub status: u8
// }

// #[derive(Deserialize)]
// pub struct DeletTodoPayload {
//     pub id: Uuid
// }

// #[derive(Serialize)]
// pub struct TodoStorage {
//     pub todos: Vec<Todo>
// }

// #[derive(Clone, Serialize)]
// pub struct Todo {
//     pub id: Uuid,
//     pub name: String,
//     pub status: Status,
// }

// #[derive(Clone, Deserialize, Serialize, PartialEq)]
// pub enum Status {
//     Started,
//     Completed,
//     InvalidEntry

// }

// impl Status {
//     pub fn map_id_to_status(x: u8) -> Self {
//         match x {
//             1 => Status::Started,
//             2 => Status::Completed,
//             _ => Status::InvalidEntry
//         }
//     }
// }


// impl TodoStorage {
//     pub fn new() -> Self{
//         Self { 
//             todos: vec![]
//         }
//     }

//     pub fn add(&mut self, todo: Todo) {
//         self.todos.push(todo);
//     }

//     pub fn get_all(&self) -> &[Todo] {
//         &self.todos
//     }

//     pub fn get_todo_by_id(&self, id: Uuid) -> Option<&Todo> {
//         let index = self.todos.iter().position(|todo| todo.id == id);
         
//         match index {
//             Some(t) => {
//                 let todo = &self.todos[t];
//                 Some(todo)
//             },
//             None => None
//         }
//     }

//     pub fn change_status(&mut self, id: Uuid, status: u8) -> Option<Todo> {
//         let index = self.todos.iter().position(|todo| todo.id == id);
         
//         match index {
//             Some(t) => {
//                 let todo = &mut self.todos[t];

//                 if todo.status != Status::map_id_to_status(status) {
//                     todo.status = Status::map_id_to_status(status);
//                     Some(todo.clone())
//                 }else {
//                     None
//                 }
//             },
//             None => None
//         }
//     }

//     pub fn delete_todo(&mut self, id: Uuid) -> Option<Todo> {
//         let index = self.todos.iter().position(|todo| todo.id == id);

//         match index {
//             Some(t) => {
//                 let todo = &mut self.todos.remove(t);

//                 Some(todo.clone())
//             },
//             None => None
//         }
//     }

//     // pub fn change_status()
    
// }


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
    pub fn new(name: String, age: u8, sex: u8) -> Self {
        // let file_storage = load_storage();

        let admin = Entity::new(name, age, Sex::map_int_to_enum(sex), Grade::None, Role::Administrator);

        let entities = vec![admin];

        // save_data(&entities);

        Self { 
            entities: entities
        }
    }

    // pub fn add_student(&mut self, name: &str, age: u8, sex: Sex, grade: Grade) {
    //     let student = Entity::new(name.to_string(), age, sex, grade, Role::Student);
    //     self.entities.push(student);
    //     save_data(&self.entities);
    // }

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
}