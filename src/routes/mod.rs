use crate::handlers::{root::root, registry_handler::{init_registry, add_student, add_staff, get_students, get_student_by_id, get_staffs, get_staff_by_id, change_grade, remove_student, remove_staff, get_all_entities}};
use crate::schema::{registry::Registry, role::Role, entity::Entity, grade::Grade, sex::Sex};
use crate::utils::util::load_storage;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub container: Arc<Mutex<Option<Registry>>>
}

impl AppState {
    pub fn new() -> Result<Self, Self>{
        let storage = load_storage();

        if !storage.is_empty() {
            let registry = Registry {
                entities: storage
            };
            Err(Self { container: Arc::new(Mutex::new(Some(registry))) })
        }else {
            Ok(Self { container: Arc::new(Mutex::new(None)) })
        }

    }
}

pub async fn axum_router() -> Router {
    let state = match AppState::new() {
        Ok(data) => data,
        Err(e) => e
    };

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/initialize", post(init_registry))
        .route("/add-student", post(add_student))
        .route("/all-students", get(get_students))
        .route("/all-entities", get(get_all_entities))
        .route("/get-student/{id}", get(get_student_by_id))
        .route("/change-grade", put(change_grade))
        .route("/remove-student/{id}", delete(remove_student))
        .route("/add-staff", post(add_staff))
        .route("/all-staffs", get(get_staffs))
        .route("/get-staff/{id}", get(get_staff_by_id))
        .route("/remove-staff/{id}", delete(remove_staff))
        .with_state(state);
        // `POST /users` goes to `create_user`
        // .route("/users", post(create_user));
    app
}