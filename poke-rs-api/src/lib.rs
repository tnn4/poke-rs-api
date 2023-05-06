#[macro_use]
extern crate num_derive;
pub mod pkmn;
pub mod util;



use serde_derive::Deserialize;


#[derive(Deserialize)]
pub struct Data {
    pub auth: Auth,
}

/// user_agent:
/// client_id:
/// client_secret:
/// username: your reddit username
/// password: your reddit password
#[derive(Deserialize)]
pub struct Auth {
    pub user_agent: String,
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
}

