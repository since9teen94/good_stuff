use actix_identity::Identity;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web_lab::web::{self as web_lab, Redirect};
pub mod home;
use good_stuff::{
    forms::LogRegForm,
    json_res, login,
    models::{Login, RegisterData},
    register, render,
    utils::consts::{
        INDEX_URL, LOGIN_TITLE, LOGIN_URL, LOGOUT_URL, LOG_REG_TEMPLATE, REGISTER, REGISTER_URL,
    },
};
use serde_json::json;
use tera::Context;
use validator::Validate;

fn redirect_to(old: &'static str, new: &'static str) -> Redirect {
    web_lab::Redirect::new(old, new)
}

async fn register_get() -> impl Responder {
    let login_form = LogRegForm::new(REGISTER);
    let context = Context::from_serialize(login_form).unwrap();
    render(LOG_REG_TEMPLATE, context)
}

async fn register_post(req: HttpRequest, register_data: web::Json<RegisterData>) -> impl Responder {
    let user = register_data.into_inner();
    let id = user.email.clone();
    if let Err(e) = user.validate() {
        return json_res(400, e);
    }
    if let Err(e) = register(user).await {
        return json_res(400, e);
    }
    let body = json!({ "message" : "User registered successfully", "status" : 201 });
    Identity::login(&req.extensions(), id).unwrap();
    json_res(201, body)
}

async fn login_get() -> impl Responder {
    let login_form = LogRegForm::new(LOGIN_TITLE);
    let context = Context::from_serialize(login_form).unwrap();
    render(LOG_REG_TEMPLATE, context)
}

async fn login_post(req: HttpRequest, login_data: web::Json<Login>) -> impl Responder {
    let user = login_data.into_inner();
    let id = user.email.clone();
    if let Err(e) = user.validate() {
        return json_res(400, e);
    }
    if let Err(e) = login(user).await {
        return json_res(400, e);
    }
    let body = json!({ "message" : "User logged in successfully", "status" : 200 });
    Identity::login(&req.extensions(), id).unwrap();
    json_res(200, body)
}

async fn logout_get(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        user.logout();
    }
    let body = json!({ "message" : "User logged out successfully", "status" : 200 });
    json_res(200, body)
}

pub fn index(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource(INDEX_URL)
            .route(web::get().to(|| async { redirect_to(INDEX_URL, LOGIN_URL) }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    )
    .service(
        web::resource(REGISTER_URL)
            .route(web::get().to(register_get))
            .route(web::post().to(register_post))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    )
    .service(
        web::resource(LOGIN_URL)
            .route(web::get().to(login_get))
            .route(web::post().to(login_post))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    )
    .service(
        web::resource(LOGOUT_URL)
            .route(web::get().to(logout_get))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
