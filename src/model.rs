use serde::{Deserialize, Serialize};

// For sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct Student {
    #[serde(default)]
    pub id: i32,
    pub name: String,
    pub class: String,
    #[serde(default)]
    pub is_active: i8,
    #[serde(default)]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub age: i32
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct StudentModelResponse{
    pub id: i32,
    pub name: String,
    pub class: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,  
    pub age:i32
}

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}