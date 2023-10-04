//! # Shaco
//!
//! A LCU REST + WAMP api wrapper

pub mod ingame;
pub mod model;
pub mod rest;
mod utils;
pub mod ws;
pub mod api;

#[cfg(test)]
mod tests {
    use crate::api::api_endpoints::get_lol_lobby_v2_lobby;
    use crate::rest::RESTClient;

    #[tokio::test]
    async fn test() {
        let client = RESTClient::new().unwrap();
        let lobby = get_lol_lobby_v2_lobby(&client).await.unwrap();

    }
}