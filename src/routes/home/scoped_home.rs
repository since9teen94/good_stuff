use crate::routes::redirect_to;
use actix_identity::Identity;
use actix_web::{web, Either, HttpResponse};
use actix_web_lab::web::Redirect;
use chrono::Datelike;
use diesel::prelude::*;
use good_stuff::{
    establish_connection, render,
    utils::consts::{DETAILS_URL, GAME_URL, LOGIN_URL},
};
use std::collections::HashMap;
use tera::Context;

type IdCheck = actix_web::Either<Redirect, HttpResponse>;

async fn details_get(user: Option<Identity>) -> IdCheck {
    if user.is_none() {
        return Either::Left(redirect_to(DETAILS_URL, LOGIN_URL));
    }
    let user_email = user.unwrap().id().unwrap();
    use good_stuff::schema::users::dsl::*;
    let conn = &mut establish_connection();
    let params = (email, first_name, last_name);
    let user_details = users
        .select(params)
        .filter(email.eq(user_email))
        .first::<(String, String, String)>(conn)
        .unwrap();
    let mut context = Context::new();
    context.insert("title", "Details");
    let details = HashMap::from([
        ("Email", &user_details.0),
        ("First Name", &user_details.1),
        ("Last Name", &user_details.2),
    ]);
    context.insert("details", &details);
    context.insert("year", &chrono::Utc::now().year());
    Either::Right(render("home.html", context))
}

async fn game_get(user: Option<Identity>) -> IdCheck {
    if user.is_none() {
        return Either::Left(redirect_to(DETAILS_URL, LOGIN_URL));
    }
    let mut context = Context::new();
    context.insert("title", "Tic-Tac-Toe");
    context.insert("year", &chrono::Utc::now().year());
    Either::Right(render("home.html", context))
}

pub fn index(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource(DETAILS_URL)
            .route(web::get().to(details_get))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    )
    .service(
        web::resource(GAME_URL)
            .route(web::get().to(game_get))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
