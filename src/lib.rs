pub mod forms;
pub mod models;
pub mod schema;
pub mod utils;

use actix_http::StatusCode;
use actix_web::{HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::{self, insert_into, pg::PgConnection, prelude::*};
use dotenvy::dotenv;
use lazy_static::lazy_static;
use models::{Login, RegisterData, RegisterUser};
use serde::Serialize;
use std::borrow::Cow;
use std::env;
use tera::{Context, Tera};
use validator::{ValidationError, ValidationErrors};

lazy_static! {
    static ref TEMPLATES: Tera = Tera::new("templates/*").unwrap();
}

pub fn render(file: &str, context: Context) -> HttpResponse {
    let template = TEMPLATES.render(file, &context).unwrap();
    HttpResponse::Ok().body(template)
}

pub fn json_res(status_code: u16, errors: impl Serialize) -> HttpResponse {
    HttpResponse::build(StatusCode::from_u16(status_code).unwrap())
        .content_type("application/json; charset=utf-8")
        .json(errors)
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn invalid_credentials(log_reg_type: &'static str) -> ValidationErrors {
    let mut errors = ValidationErrors::new();
    let mut registration_error = ValidationError::new(log_reg_type);
    registration_error.message = Some(Cow::Borrowed("Invalid Credentials"));
    errors.add("email", registration_error.to_owned());
    errors.add("password", registration_error);
    errors
}

pub async fn register(user: RegisterData) -> Result<(), ValidationErrors> {
    let hashed_password = password_hasher(&user.password).await.unwrap();
    let new_user = RegisterUser {
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
        password: hashed_password,
    };
    use schema::users::dsl::*;
    let conn = &mut establish_connection();
    if email_count(&new_user.email, 0).is_err() {
        return Err(invalid_credentials("registration"));
    };
    if insert_into(users).values(new_user).execute(conn).is_err() {
        return Err(invalid_credentials("registration"));
    };
    Ok(())
}

pub async fn login(user: Login) -> Result<(), ValidationErrors> {
    if email_count(&user.email, 1).is_err() {
        return Err(invalid_credentials("login"));
    };
    use schema::users::dsl::*;
    let mut conn = establish_connection();
    let hashed_password = &users
        .select(password)
        .filter(email.eq(&user.email))
        .limit(1)
        .load::<String>(&mut conn)
        .unwrap()[0];
    if password_hash_checker(&user.password, hashed_password).is_err() {
        return Err(invalid_credentials("login"));
    }
    Ok(())
}

async fn password_hasher(password_str: &str) -> Result<String, argon2::password_hash::Error> {
    let password = password_str.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt)?.to_string();
    Ok(password_hash)
}

fn password_hash_checker(
    password: &str,
    password_hash: &str,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)?;
    Ok(())
}

fn email_count(value: &str, count: usize) -> Result<(), ValidationError> {
    use schema::users::dsl::*;
    let mut conn = establish_connection();
    let email_unique = users
        .select(email)
        .filter(email.eq(value))
        .limit(2)
        .load::<String>(&mut conn);
    if email_unique.is_err() || email_unique.unwrap().len() != count {
        return Err(ValidationError::new("email"));
    };
    Ok(())
}

pub async fn not_allowed() -> impl Responder {
    HttpResponse::build(StatusCode::METHOD_NOT_ALLOWED)
        .content_type("text/html; charset=utf-8")
        .body("<h1>405 Not Allowed</h1>")
}

pub async fn not_found() -> impl Responder {
    HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body("<h1>404</h1><p>Page Not Found</p>")
}
