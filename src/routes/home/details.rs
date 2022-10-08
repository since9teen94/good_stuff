//TODO details displays user info
use crate::routes::redirect_to;
use actix_identity::Identity;
use actix_web::{web, Either, HttpResponse};
use actix_web_lab::web::Redirect;
use good_stuff::forms::{DETAILS_URL, LOGIN_URL};

type RedirectOrResponse = actix_web::Either<Redirect, HttpResponse>;

async fn details_get(user: Option<Identity>) -> RedirectOrResponse {
    if user.is_none() {
        return Either::Left(redirect_to(DETAILS_URL, LOGIN_URL));
    }
    Either::Right(HttpResponse::Ok().body("Hello from bod"))
}

pub fn index(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource(DETAILS_URL)
            .route(web::get().to(details_get))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
