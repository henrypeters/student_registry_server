use crate::schema::registry::{Registry, AddStudent, CreateNewRegistry, GetAndDeleteStudentById, ChangeStudentGrade};
use crate::schema::entity::Entity;
use crate::schema::grade::Grade;
use crate::schema::role::Role::{self, Student};
use crate::schema::sex::Sex;
use crate::routes::AppState;

use crate::utils::util::{load_storage, save_data};

use tracing_subscriber::registry;
use uuid::Uuid;

use axum::extract::State;
use axum::{Json, http::StatusCode};
pub async fn init_registry(
    State(state): State<AppState>,
    Json(payload): Json<CreateNewRegistry>
) -> Result<(StatusCode, String), String> {
    let mut storage = state.container.lock().unwrap();
    
    // IN this line, ,I'm ensuring that when the registry has been initialized before, 
    // no other person can reinitialize the registry.
    // When it's `Some`, it means it is already initilialeized, 
    // When it's `None`, it means it is not yet initialized
    match *storage {
        Some(_) => {
            Err("Regisry is already initialized".to_string())
        },
        None =>  {
            *storage = Some(Registry::init(payload.name.clone(), payload.age, payload.sex));
            Ok((StatusCode::CREATED, format!("==== Registry iniialized === \nAdmin: {}", payload.name.clone())))
        }
    }   
}

//////////////// Entities
pub async fn get_all_entities(
    State(state): State<AppState>
) -> Result<(StatusCode, Json<Vec<Entity>>), String> {
    let storage = state.container.lock().unwrap();

    match storage.as_ref() {
        Some(registry) => {
            let students = registry.all_entities();

            Ok((StatusCode::FOUND, Json(students)))
        },
        None => return Err("Can't get all entities".to_string())
    }
}


/////////////// STUDENTS

pub async fn add_student(
    State(state): State<AppState>,
    Json(payload): Json<AddStudent>
) -> Result<(StatusCode, Json<Entity>), String> {
    let student = {
        let mut storage = state.container.lock().unwrap();
        
        match storage.as_mut() {
            Some(registry) => {
                let student = Entity::new(payload.name, payload.age, 
                                                    Sex::map_int_to_enum(payload.sex), 
                                                    Grade::map_int_to_grade(payload.grade), 
                                                    Role::Student);
                match registry.add_student(student.clone()) {
                    Ok(()) => (),
                    Err(e) => return Err(e)
                }

                student
            },
            None => return Err("Registry Not Initialized".to_string())
        }
    };
    
    Ok((StatusCode::FOUND, Json(student)))
}


pub async fn get_students(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Entity>>), String> {
    let students = {
        let storage = state.container.lock().unwrap();
        
        match storage.as_ref() {
            Some(registry) => {
           
            match registry.list_all_students() {
                    Ok(entities) => entities,
                    Err(e) => return Err(e)
                } 
            },
            None => return Err("Registry Not Initialized".to_string())
        }
    };
        
    Ok((StatusCode::FOUND, Json(students)))
}

pub async fn get_student_by_id(    
    State(state): State<AppState>,
    Json(payload): Json<GetAndDeleteStudentById>
) -> Result<(StatusCode, Json<Entity>), String> {
    
    let student = {
        let store = state.container.lock().unwrap();
        match store.as_ref() {
            Some(registry) => {
               match registry.get_student_by_id(payload.id){
                    Some(student) => student.clone(),
                    None => return Err("Can't find student".to_string())
                }
            },
            None => return Err("Registry not initialized".to_string())
        }
    };

    
    Ok((StatusCode::FOUND, Json(student)))
}

pub async fn change_grade(
    State(state): State<AppState>,
    Json(payload): Json<ChangeStudentGrade>
) -> Result<(StatusCode, Json<Entity>), String> {
    
    let student = {
        let mut store = state.container.lock().unwrap();
        
        match store.as_mut() {
            Some(registry) => {
                match registry.change_student_grade(payload.id, payload.grade) {
                    Ok(student) => student,
                    Err(_) => return Err("Student not found, maybe an incorrect id or grade".to_string())
                }

            },
            None => return Err("Not initialized".to_string())

        }
    };
    
    Ok((StatusCode::FOUND, Json(student)))

}


pub async fn remove_student(
    State(state): State<AppState>,
    Json(payload): Json<GetAndDeleteStudentById>
) -> Result<(StatusCode, Json<Entity>), String> {

    let student = {
        let mut store = state.container.lock().unwrap();


        match store.as_mut() {
            Some(registry) => {
                match registry.remove_student(payload.id) {
                    Ok(student) => student,
                    Err(_) => return Err("Can't find student to remove".to_string())
                }
            }
            None => return Err("Registry not initialiazed".to_string())
        }
    };

    Ok((StatusCode::FOUND, Json(student)))
    
}

