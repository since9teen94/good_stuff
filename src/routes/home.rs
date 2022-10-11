pub mod scoped_home;

use crate::routes::redirect_to;
use actix_identity::Identity;
use actix_web::{web, Either, HttpMessage, HttpRequest, HttpResponse};
use actix_web_lab::web::Redirect;
use good_stuff::render;
use good_stuff::utils::consts::{HOME_TEMPLATE, HOME_TITLE, HOME_URL, HOUSE_URL, LOGIN_URL, TITLE};
use tera::Context;

type RedirectOrResponse = actix_web::Either<Redirect, HttpResponse>;

async fn home_get(user: Option<Identity>) -> RedirectOrResponse {
    if user.is_none() {
        return Either::Left(redirect_to(HOME_URL, LOGIN_URL));
    }
    let mut context = Context::new();
    context.insert(TITLE, HOME_TITLE);
    Either::Right(render(HOME_TEMPLATE, context))
}

///This function exists as a convenience
async fn house_get(user: Option<Identity>, req: HttpRequest) -> Redirect {
    if user.is_some() {
        return redirect_to(HOUSE_URL, HOME_URL);
    }
    let mut context = Context::new();
    context.insert(TITLE, HOME_TITLE);
    let id = String::from("frodo@theshire.com");
    Identity::login(&req.extensions(), id).unwrap();
    redirect_to(HOUSE_URL, HOME_URL)
}

pub fn index(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource(HOME_URL)
            .route(web::get().to(home_get))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    )
    .service(
        web::resource(HOUSE_URL)
            .route(web::get().to(house_get))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
