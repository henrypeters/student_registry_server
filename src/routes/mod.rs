use crate::handlers::{root::root, registry_handler::{init_registry, add_student, get_students, get_student_by_id, change_grade, remove_student}};
use crate::schema::{registry::Registry, role::Role, entity::Entity, grade::Grade, sex::Sex};

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
    pub fn new() -> Self{
        Self { container: Arc::new(Mutex::new(None)) }
    }
}

pub async fn axum_router() -> Router {
    let state = AppState::new();

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/intialize", post(init_registry))
        .route("/add-student", post(add_student))
        .route("/all-students", get(get_students))
        .route("/get-student", get(get_student_by_id))
        .route("/change-grade", put(change_grade))
        .route("/remove-student", delete(remove_student))
        .with_state(state);
        // `POST /users` goes to `create_user`
        // .route("/users", post(create_user));
    app
}