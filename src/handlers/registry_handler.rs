use crate::schema::registry::{AddStudent, AuthUser, ChangeStudentGrade, CreateNewRegistry, GetAndDeleteStudentById, Registry};
use crate::schema::entity::Entity;
use crate::schema::grade::Grade;
use crate::schema::role::Role::{self, Student};
use crate::schema::sex::Sex;
use crate::routes::AppState;

use crate::utils::util::{load_storage, save_data};

use axum::Json;
use tracing_subscriber::registry;
use uuid::Uuid;

use axum::extract::State;
use axum::{http::StatusCode};
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
            let (registry, response) = match Registry::init(payload.name.clone(), payload.age, payload.sex){
                Ok(data, ) => data,
                Err(e) => return Err(e) 
            };
            *storage = Some(registry);
            Ok((StatusCode::CREATED, format!("==== Registry iniialized === \nAdmin: {} \nTOKEN: {:?}", payload.name.clone(), response)))
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
        None => return Err("No entities found".to_string())
    }
}


/////////////// STUDENTS

pub async fn add_student(
    AuthUser{claims}: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<AddStudent>

) -> Result<(StatusCode, Json<Entity>), String> {
    if claims.role != Role::Administrator {
        return Err("Unauthorized".to_string());
    }

    let student = {
        let mut storage = state.container.lock().unwrap();
        
        match storage.as_mut() {
            Some(registry) => {
                let student = Entity::new(payload.name, payload.age, 
                                                    Sex::map_int_to_enum(payload.sex), 
                                                    Grade::map_int_to_grade(payload.grade), 
                                                    Role::Student);
                match registry.add_student(student.clone()) {
                    //This is returning a unit type, student JWT, and the student entity
                    // Wanted to return the token(JWT) and student entity, but the `add_student` 
                    // function returns this: `()` also because save_data is a function in util.rs that returns a result of unit type 
                    Ok(()) => (),  
                    Err(e) => return Err(e)
                }

                student
            },
            None => return Err("Registry Not Initialized".to_string())
        }
    };

    
    // In here, we're returniing statuscode, student entity and token together as a string.
    // Please Note: Every token generated after adding new students is not connected/related to each student yet. They(JWT) are just randomly created 
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
    AuthUser { claims }: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<ChangeStudentGrade>
) -> Result<(StatusCode, Json<Entity>), String> {
    if claims.role != Role::Administrator {
        return Err("Unauthoriized".to_string());
    }

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
    AuthUser { claims }: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<GetAndDeleteStudentById>
) -> Result<(StatusCode, Json<Entity>), String> {
    if claims.role != Role::Administrator {
        return Err("Unauthorized".to_string());
    }

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




/////////// STAFFS
pub async fn add_staff(
    AuthUser{claims}: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<AddStudent>

) -> Result<(StatusCode, String), String> {
    if claims.role != Role::Administrator {
        return Err("Unauthorized".to_string());
    }

    let new_staff_data = {
        let mut storage = state.container.lock().unwrap();
        
        match storage.as_mut() {
            Some(registry) => {
                let staff = Entity::new(payload.name, payload.age, 
                                                    Sex::map_int_to_enum(payload.sex), 
                                                    Grade::None, 
                                                    Role::Staff);
                match registry.add_staff(staff.clone()) {
                    //This is returning a unit type, student JWT, and the staff entity
                    // Wanted to return the token(JWT) and student entity, but the `add_staff` 
                    // function returns this: `()` also because save_data is a function in util.rs that returns a result of unit type 
                    Ok(((), token)) => ((), token, staff),  
                    Err(e) => return Err(e)
                }

            },
            None => return Err("Registry Not Initialized".to_string())
        }
    };

    // Destructure
    let (_, token, staff) = new_staff_data;
    
    // In here, we're returniing statuscode, student entity and token together as a string.
    // Please Note: Every token generated after adding new students is not connected/related to each student yet. They(JWT) are just randomly created 
    Ok((StatusCode::FOUND, format!("NEW STAFF ADDED \nStudent: {:?} \nToken: {}", staff, token)))
}