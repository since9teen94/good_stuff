use crate::routes::redirect_to;
use actix_identity::Identity;
use actix_web::{web, Either, HttpResponse};
use actix_web_lab::web::Redirect;
use diesel::prelude::*;
use good_stuff::{
    establish_connection, render,
    utils::consts::{DETAILS_URL, LOGIN_URL},
};
use std::collections::HashMap;
use tera::Context;

type RedirectOrResponse = actix_web::Either<Redirect, HttpResponse>;

async fn details_get(user: Option<Identity>) -> RedirectOrResponse {
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
    Either::Right(render("details.html", context))
}

pub fn index(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource(DETAILS_URL)
            .route(web::get().to(details_get))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
