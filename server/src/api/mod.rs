mod lobbies;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        lobbies::all,
        lobbies::get,
        lobbies::create,
        lobbies::join,
        lobbies::leave,
        lobbies::start
    ]
}
