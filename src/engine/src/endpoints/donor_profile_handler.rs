use axum::{extract::State, Json};
use chrono::{NaiveDateTime, NaiveTime};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{types::uuid, PgPool};
use uuid::Uuid;

pub async fn new_temp_profile_started() {
    println!("Database query complete");
}
pub async fn new_temp_profile_cancelled() {
    todo!()
}

pub async fn create_new_temp_profile(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, Json<Doadores>), (StatusCode, String)> {
    println!("create_new_temp_profile");
    let result = sqlx::query_as!(Doadores, "SELECT * FROM doadores LIMIT 1")
        .fetch_one(&db_pool)
        .await;
    println!("Database query complete");

    match result {
        Ok(row) => Ok((StatusCode::OK, Json(row))), // Success: Return the row as JSON
        Err(err) => {
            eprintln!("Failed to fetch profile: {:?}", err); // Log the error for debugging
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch profile".into(),
            ))
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Doadores {
    pub id: Uuid,                         // UUID for unique identifier
    pub nome: Option<String>,             // Nullable name of the donor
    pub email: Option<String>,            // Nullable email
    pub horario: Option<NaiveTime>,       // Nullable time format (e.g., '09:00')
    pub doador: Option<bool>,             // Nullable donor status (true/false)
    pub morada: Option<String>,           // Nullable address
    pub freguesia: Option<String>,        // Nullable parish
    pub concelho: Option<String>,         // Nullable county
    pub codigo_postal: Option<String>,    // Nullable postal code
    pub tel_residencial: Option<String>,  // Nullable home phone
    pub tel_trabalho: Option<String>,     // Nullable work phone
    pub telemovel: Option<String>,        // Nullable mobile phone
    pub criado_em: Option<NaiveDateTime>, // Nullable timestamp for creation
    pub observacoes: Option<String>,      // Nullable notes or observations
}
