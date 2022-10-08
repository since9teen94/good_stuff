use crate::schema::users;
use chrono;
use diesel::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Queryable, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    //#[serde(rename = "firstName")]
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Validate, Deserialize, Debug)]
pub struct RegisterData {
    #[validate(length(min = 1, message = "Required"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "Required"))]
    pub last_name: String,
    #[validate(
        length(min = 1, message = "Required"),
        email(message = "Please enter a valid email")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Required"),
        length(min = 8, message = "Password must be at least 8 characters"),
        must_match(other = "confirm_password", message = "Passwords must match")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = "Required"),
        length(min = 8, message = "Password must be at least 8 characters"),
        must_match(other = "password", message = "Passwords must match")
    )]
    pub confirm_password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct RegisterUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct Login {
    #[validate(
        email(message = "Please enter a valid email"),
        length(min = 1, message = "Required")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Required"),
        length(min = 8, message = "Password must be at least 8 characters")
    )]
    pub password: String,
}
