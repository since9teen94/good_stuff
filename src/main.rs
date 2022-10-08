use actix_files as fs;
use actix_web::{cookie::Key, web, App, HttpServer};
mod routes;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use dotenvy::dotenv;
use good_stuff::not_found;
use routes::{home, home::scoped_home};
use std::{env, io};

#[actix_web::main()]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| String::from("3000"))
        .parse()
        .expect("Error parsing PORT variable: ");
    HttpServer::new(|| {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build(),
            )
            .configure(routes::index)
            .configure(home::index)
            .service(web::scope("/home").configure(scoped_home::index))
            .service(fs::Files::new("/static", "./static"))
            .default_service(web::to(not_found))
    })
    //TODO
    //
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
