use std::{io};
use actix_web::{App, HttpServer};

mod users;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            // .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(users::list)
            .service(users::get)
            .service(users::create)
            .service(users::delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
