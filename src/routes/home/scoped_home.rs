use crate::routes::redirect_to;
use actix_identity::Identity;
use actix_web::{
    web::{self, Query},
    Either, HttpResponse,
};
use actix_web_lab::web::Redirect;
use chrono::Datelike;
use diesel::prelude::*;
use good_stuff::{
    establish_connection, render,
    utils::consts::{
        DETAILS_URL, ELEMENTS, GAME_URL, LINKS_ONE, LINKS_TWO, LINKS_URL, LOGIN_URL, PTABLE_URL,
        QUOTES_URL, SKILLS, SKILLS_URL,
    },
    Pagination,
};
use paginate::Pages;
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

async fn skills_get(user: Option<Identity>) -> IdCheck {
    if user.is_none() {
        return Either::Left(redirect_to(DETAILS_URL, LOGIN_URL));
    }
    let mut context = Context::new();
    context.insert("title", "Skills");
    context.insert("year", &chrono::Utc::now().year());
    let skills_len = SKILLS.len() as f32;
    let halfway = f32::ceil(skills_len / 2.0) as usize;
    let skills_one = &SKILLS[0..halfway];
    let skills_two = &SKILLS[halfway..];
    context.insert("skillsOne", skills_one);
    context.insert("skillsTwo", skills_two);
    Either::Right(render("home.html", context))
}

async fn links_get(user: Option<Identity>) -> IdCheck {
    if user.is_none() {
        return Either::Left(redirect_to(DETAILS_URL, LOGIN_URL));
    }
    let links_one = HashMap::from(LINKS_ONE);
    let links_two = HashMap::from(LINKS_TWO);
    let mut context = Context::new();
    context.insert("title", "Links");
    context.insert("year", &chrono::Utc::now().year());
    context.insert("links", &[links_one, links_two]);
    Either::Right(render("home.html", context))
}

async fn quotes_get(user: Option<Identity>) -> IdCheck {
    if user.is_none() {
        return Either::Left(redirect_to(DETAILS_URL, LOGIN_URL));
    }
    let mut context = Context::new();
    context.insert("title", "Office Quotes");
    context.insert("year", &chrono::Utc::now().year());
    Either::Right(render("home.html", context))
}

async fn ptable_get(user: Option<Identity>, info: Option<Query<Pagination>>) -> IdCheck {
    if user.is_none() {
        return Either::Left(redirect_to(DETAILS_URL, LOGIN_URL));
    }
    let total_items: usize = ELEMENTS.len();
    let items_per_page: usize = 10;
    let last_page = f32::ceil(total_items as f32 / items_per_page as f32) as i32;
    let pages = Pages::new(total_items, items_per_page);
    let mut cur_page = match info {
        Some(t) => t.cur_page,
        None => 1,
    };
    if cur_page < 1 {
        cur_page = 1
    } else if cur_page > last_page {
        cur_page = last_page
    }
    let page = pages.with_offset((cur_page - 1) as usize);
    let mut context = Context::new();
    context.insert("title", "Periodic Table");
    context.insert("year", &chrono::Utc::now().year());
    context.insert("curPage", &cur_page);
    context.insert("lastPage", &last_page);
    context.insert("elements", &ELEMENTS[page.start..=page.end]);
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
    )
    .service(
        web::resource(SKILLS_URL)
            .route(web::get().to(skills_get))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    )
    .service(
        web::resource(LINKS_URL)
            .route(web::get().to(links_get))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    )
    .service(
        web::resource(QUOTES_URL)
            .route(web::get().to(quotes_get))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    )
    .service(
        web::resource(PTABLE_URL)
            .route(web::get().to(ptable_get))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
