//TODO details displays user info
use crate::routes::redirect_to;
use actix_identity::Identity;
use actix_web::{web, Either, HttpResponse};
use actix_web_lab::web::Redirect;
use diesel::prelude::*;
use good_stuff::establish_connection;
use good_stuff::forms::{DETAILS_URL, LOGIN_URL};

type RedirectOrResponse = actix_web::Either<Redirect, HttpResponse>;

async fn details_get(user: Option<Identity>) -> RedirectOrResponse {
    if user.is_none() {
        return Either::Left(redirect_to(DETAILS_URL, LOGIN_URL));
    }
    //TODO finish pulling details
    use good_stuff::schema::users::dsl::*;
    let conn = &mut establish_connection();
    let details = users
        .select(id)
        .filter(id.eq(user.id).first::<String>(&mut conn));
    //println!("{:?}", user);
    Either::Right(HttpResponse::Ok().body(details))
}

pub fn index(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource(DETAILS_URL)
            .route(web::get().to(details_get))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
