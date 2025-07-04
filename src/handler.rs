use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    model::{Student, StudentModelResponse,FilterOptions},
    AppState,
};


pub async fn student_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Param
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    // Query without macro
    let notes =
        sqlx::query_as::<_, Student>(r#"SELECT * FROM studentinfo ORDER by id LIMIT ? OFFSET ?"#)
            .bind(limit as i32)
            .bind(offset as i32)
            .fetch_all(&data.db)
            .await
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "error",
                    "message": format!("Database error: { }", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

    // Response
    let note_responses = notes
        .iter()
        .map(|note| to_student_response(&note))
        .collect::<Vec<StudentModelResponse>>();

    let json_response = serde_json::json!({
        "status": "ok",
        "count": note_responses.len(),
        "notes": note_responses
    });

    Ok(Json(json_response))
}

pub async fn add_student_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<Student>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query(r#"INSERT INTO studentinfo(name,class,age) VALUES (?, ?,?)"#)
        .bind(&body.name)
        .bind(&body.class)
        .bind(&body.age)
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    let note_response = serde_json::json!({
            "status": "success",
            "data": "Student added successfully.."
    });
    Ok(Json(note_response))
}
pub async fn get_student_byid_handler(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    // get not using query macro
    let query_result = sqlx::query_as::<_, Student>(r#"select id,name,class,s.is_active ,s.created_at ,s.updated_at,s.age  from studentinfo s where id= ?"#)
        .bind(&id)
        .fetch_one(&data.db)
        .await;

    // check & response
    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "note": to_student_response(&note)
                })
            });

            return Ok(Json(note_response));
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Student with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    };
}

pub async fn edit_student_handler(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<Student>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let query_result = sqlx::query_as::<_, Student>(r#"select id,name,class,s.is_active ,s.created_at ,s.updated_at  from studentinfo s where id= ?"#)
        .bind(&id)
        .fetch_one(&data.db)
        .await;

    let note = match query_result {
        Ok(note) => note,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Student with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            ));
        }
    };

    //let is_active = body.is_active.unwrap_or(note.is_active != 0);
    //let i8_is_active = is_active as i8;

    let update_result =
        sqlx::query(r#"update studentinfo set name=?,class=?,is_active=? where  id = ?"#)
            .bind(&body.name)
            .bind(&body.class)
            .bind(&body.is_active)
            .bind(&id)
            .execute(&data.db)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("{:?}", e)
                    })),
                )
            })?;

            if update_result.rows_affected() == 0 {
                let error_response = serde_json::json!({
                    "status": "error",
                    "message": format!("Note with ID: {} not found", id)
                });
                return Err((StatusCode::NOT_FOUND, Json(error_response)));
            }

            let note_response = serde_json::json!({
                "status": "success",
                "data": "Student details updated successfully."
            });
    Ok(Json(note_response))
}
pub async fn delete_student_handler(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    // delete not using query macro
    let query_result = sqlx::query(r#"DELETE FROM studentinfo WHERE id = ?"#)
        .bind(&id)
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;

    // response
    if query_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }
    let note_response = serde_json::json!({
                "status": "success",
                "data": "Student details removed successfully."
            });

    Ok(Json(note_response))
}

// Convert DB Model to Response
fn to_student_response(note: &Student) -> StudentModelResponse {
    StudentModelResponse {
        id: note.id.to_owned(),
        name: note.name.to_owned(),
        class: note.class.to_owned(),
        is_active: note.is_active != 0,
        created_at: note.created_at.unwrap(),
        updated_at: note.updated_at.unwrap(),
        age:note.age.to_owned()
    }
}