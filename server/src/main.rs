#[macro_use]
extern crate rocket;

mod api;

#[get("/")]
pub(crate) fn health() -> &'static str {
    "OK"
}

#[launch]
fn rocker() -> _ {
    rocket::build()
        .mount("/", routes![health])
        .mount("/api", api::routes())
}
