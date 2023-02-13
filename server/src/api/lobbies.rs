// use rocket::http::Status;
// use rocket::response::{content, status};

// #[get("/lobbies")]
// pub(crate) fn all() -> status::Custom<content::RawJson<&'static str>> {
//     status::Custom(
//         Status::Ok,
//         content::RawJson(
//             r#"
//                 [
//                     {
//                         "title": "My favourite lobby"
//                     }
//                 ]
//             "#,
//         ),
//     )
// }

#[get("/lobbies/<id>")]
pub(crate) fn get(id: u32) -> String {
    format!("Lobby {}", id)
}

#[post("/lobbies")]
pub(crate) fn create() -> String {
    "Lobby created".to_string()
}

#[post("/lobbies/<id>/join")]
pub(crate) fn join(id: u32) -> String {
    format!("Lobby join {}", id)
}

#[post("/lobbies/<id>/leave")]
pub(crate) fn leave(id: u32) -> String {
    format!("Lobby leave {}", id)
}

#[post("/lobbies/<id>/start")]
pub(crate) fn start(id: u32) -> String {
    format!("Lobby start {}", id)
}
