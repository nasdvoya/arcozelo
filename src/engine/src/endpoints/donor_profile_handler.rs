use axum::{extract::State, Json};
use chrono::{NaiveDateTime, NaiveTime};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{types::uuid, PgPool};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Doador {
    pub id: Option<Uuid>,                 // UUID for unique identifier
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

pub async fn new_temp_profile_started() {
    todo!()
}
pub async fn new_temp_profile_cancelled() {
    todo!()
}

pub async fn new_donor(State(db_pool): State<PgPool>, Json(new_donor): Json<Doador>) -> Result<(StatusCode, String), (StatusCode, String)> {
    println!("new_donor:");
    let uuid = new_donor.id.unwrap_or_else(Uuid::new_v4);
    let result = sqlx::query!(
        "INSERT INTO doadores (
            id, nome, email, horario, doador, morada, freguesia, concelho,
            codigo_postal, tel_residencial, tel_trabalho, telemovel,
            criado_em, observacoes
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14
        )",
        uuid,
        new_donor.nome,
        new_donor.email,
        new_donor.horario,
        new_donor.doador,
        new_donor.morada,
        new_donor.freguesia,
        new_donor.concelho,
        new_donor.codigo_postal,
        new_donor.tel_residencial,
        new_donor.tel_trabalho,
        new_donor.telemovel,
        new_donor.criado_em,
        new_donor.observacoes
    )
    .execute(&db_pool)
    .await;

    match result {
        Ok(_) => {
            println!("Perfil criado com sucesso");
            Ok((StatusCode::CREATED, "Perfil criado com sucesso".into()))
        }
        Err(err) => {
            eprintln!("Falha ao criar perfil: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Falha ao criar perfil".into()))
        }
    }
}

pub async fn get_all_donors(State(db_pool): State<PgPool>) -> Result<(StatusCode, Json<Doador>), (StatusCode, String)> {
    let result = sqlx::query_as!(Doador, "SELECT * FROM doadores LIMIT 1").fetch_one(&db_pool).await;

    match result {
        Ok(row) => {
            println!("Doadores encontrados com sucesso");
            Ok((StatusCode::OK, Json(row)))
        }
        Err(err) => {
            eprintln!("Falha ao buscar doadores: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch profile".into()))
        }
    }
}
