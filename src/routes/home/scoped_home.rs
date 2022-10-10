//TODO details displays user info
use crate::routes::redirect_to;
use actix_identity::Identity;
use actix_web::{web, Either, HttpResponse};
use actix_web_lab::web::Redirect;
use diesel::prelude::*;
use good_stuff::{
    establish_connection,
    forms::{DETAILS_URL, LOGIN_URL},
};

type RedirectOrResponse = actix_web::Either<Redirect, HttpResponse>;

async fn details_get(user: Option<Identity>) -> RedirectOrResponse {
    if user.is_none() {
        return Either::Left(redirect_to(DETAILS_URL, LOGIN_URL));
    }
    //at this point, only if logged in and verified, also...
    //try to get the tuple to more meaningful state
    let user_email = user.unwrap().id().unwrap();
    use good_stuff::schema::users::dsl::*;
    let conn = &mut establish_connection();
    let params = (email, first_name, last_name);
    let user_details = users
        .select(params)
        .filter(email.eq(user_email))
        .first::<(String, String, String)>(conn)
        .unwrap();
    let body = serde_json::json!({
        "email": user_details.0,
        "first_name": user_details.1,
        "last_name": user_details.2,
    });
    //let user_details_struct = UserDetails::new(user_details);
    Either::Right(HttpResponse::Ok().json(body))
}

pub fn index(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource(DETAILS_URL)
            .route(web::get().to(details_get))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
