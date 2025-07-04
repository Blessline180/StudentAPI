
use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};

use crate::{
    handler::{
        add_student_handler, delete_student_handler, edit_student_handler, get_student_byid_handler, student_list_handler
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/addstudent",post(add_student_handler))
        .route("/api/studentlist", get(student_list_handler))
        .route("/api/getbyid/:id", get(get_student_byid_handler))
        .route("/api/update/:id", patch(edit_student_handler))
        .route("/api/delete/:id", delete(delete_student_handler))
        .with_state(app_state)
}