
use std::io;

use crate::schema::entity::Entity;
use crate::schema::role::Role;
use crate::schema::sex::Sex;
use crate::schema::grade::Grade;
use crate::utils::util::{load_storage, save_data};


use axum::http::Error;
// use jsonwebtoken::errors::ErrorKind::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use axum::{
    extract::{FromRequestParts, Json},
    http::{
        header::AUTHORIZATION,
        request::Parts,
        StatusCode
    },
    response::{IntoResponse, Response},
    routing::{get, post},
    Router
};
use jsonwebtoken::{
    decode,
    encode,
    DecodingKey,
    EncodingKey, 
    Validation,
    Header,
};

use chrono::{Duration, Utc};


// const JWT_secret: &str = "token_creation_secret##1";

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in the environment")
}



#[derive(Deserialize)]
pub struct CreateNewRegistryOrAddStaff {
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
pub struct ChangeStudentGrade {
    pub id: Uuid,
    pub grade: u8
}

//////////// JSON Web Token
#[derive(Deserialize, Serialize)]

#[derive(Debug)]
pub struct TokenResponse{
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub exp: usize
}


pub struct AuthUser{
    pub claims: Claims
}

impl<S> FromRequestParts<S> for AuthUser
where S: Send + Sync {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        _: &S
    ) -> Result<Self, Self::Rejection> {
        let auth_handller = parts
                            .headers
                            .get(AUTHORIZATION)
                            .and_then(|value| value.to_str().ok())
                            .ok_or_else(|| {
                                (StatusCode::UNAUTHORIZED, "Missing Authorization Header").into_response()
                            })?;

        let token = auth_handller
                            .strip_prefix("Bearer ")
                            .ok_or_else(|| {
                                (StatusCode::UNAUTHORIZED, "Invalid Authorization format").into_response()
                            })?;

        let secret = jwt_secret();
        let decoded = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
                                                    .map_err(|_| {
                                                        (StatusCode::UNAUTHORIZED, "Invalid token").into_response()
                                                    })?;

        Ok(AuthUser { 
            claims: decoded.claims 
        })                                            
    }
}

#[derive(Serialize)]
pub struct Registry {
    pub entities: Vec<Entity>,
}

impl Registry {
    pub fn init(name: String, age: u8, sex: u8) -> Result<(Self, String), String> {
        let expiration = Utc::now() + Duration::hours(24);

        let claims = Claims {
            sub: name.clone(),
            role: Role::Administrator,
            exp: expiration.timestamp() as usize
        };

        let secret = jwt_secret();
        let token = match  encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())) {
            Ok(token) => token,
            Err(_) => return Err("Internal Sever Error".to_string())
        };

        // let response = TokenResponse {
        //     token
        // };

        let admin = Entity::new(name, age, Sex::map_int_to_enum(sex), Grade::None, Role::Administrator);
        let entities = vec![admin];

        Ok((Self { entities: entities}, token))
    }

    pub fn all_entities(&self) -> Vec<Entity> {
       let entities = &self.entities;

       entities.clone() 
    }

    //////// STUDENTS

    pub fn add_student(&mut self, student: Entity) -> Result<(), String> {
        self.entities.push(student.clone());

        match save_data(&self.entities) {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("{}", e))
        }
    }

    pub fn list_all_students(&self) -> Result<Vec<Entity>, String> {
        let file_storage = load_storage();
        
        let ref_student_vec: Vec<&Entity> = file_storage.iter().filter(|x| x.role == Role::Student).collect();
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
        let mut file_storage = load_storage();

        let new_grade = Grade::map_int_to_grade(grade);
        let student = match file_storage.iter_mut().find(|student| student.id == id && student.grade != new_grade && student.role == Role::Student) {
            Some(student) => student,
            None => return Err(std::io::Error::other("Coundn't find student"))
        };

        student.grade = new_grade;
        let result = student.clone();
        
        save_data(&file_storage)?;

        Ok(result)
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
    pub fn add_staff(&mut self, staff: Entity) -> Result<((), String), String> {
        
        let expiration = Utc::now() + Duration::hours(24);
        
        let claims = Claims {
            sub: staff.clone().name,
            role: Role::Staff,
            exp: expiration.timestamp() as usize
        };
        
        let secret = jwt_secret();
        let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())){
            Ok(token) => token,
            Err(_) => return Err("Internal Server Error".to_string())
        };

        self.entities.push(staff.clone());

        match save_data(&self.entities) {
            Ok(()) => Ok(((), token)),
            Err(e) => Err(format!("{}", e))
        }
    }

     pub fn list_all_staffs(&self) -> Result<Vec<Entity>, String> {
        let file_storage = load_storage();
        
        let ref_staff_vec: Vec<&Entity> = file_storage.iter().filter(|x| x.role == Role::Staff).collect();
        if ref_staff_vec.is_empty() {
            return Err("No students in registry".to_string());
        }

        let owned_staff_vec:Vec<Entity> = ref_staff_vec.into_iter().cloned().collect();

        Ok(owned_staff_vec)
        
    }

    pub fn get_staff_by_id(&self, id: Uuid) -> Option<Entity> {
        let file_storage = load_storage();

        let staff_option = file_storage.into_iter()
                                            .find(|staff| staff.id == id && staff.role == Role::Student);
                                    
        match staff_option {
            Some(staff) => {
                Some(staff.clone())
            },
            None => None
        }   

    }

    pub fn remove_staff(&mut self, id: Uuid) -> io::Result<Entity> {
        let mut file_storage = load_storage();

        let index = match file_storage.iter().position(|staff| staff.id == id && staff.role == Role::Staff ) {
            Some(staff) => staff,
            None => return Err(std::io::Error::other("Couldn't find student"))
        };

        let removed_staff = file_storage.remove(index);

        save_data(&file_storage)?;

        Ok(removed_staff)        
    }

}