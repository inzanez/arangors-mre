use rocket::routes;
use rocket_contrib::databases::{arangors, database};

mod config;


#[database("catalog")]
pub struct CatalogDb(pub arangors::Connection);

#[rocket::get("/health", rank = 2)]
fn health() -> rocket::http::Status {
    rocket::http::Status::Ok
}

#[rocket::launch]
fn launch() -> rocket::Rocket {
    dotenv::dotenv().ok();

    rocket::custom(config::Config::figment())
        .attach(CatalogDb::fairing())
        .mount("/", routes![health])
}
