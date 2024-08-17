use redis::{Client, Commands};
use tower_cookies::Cookies;

pub async fn get_user_id(redis_client: &Client, cookies: Cookies) -> Option<String> {
    let session_id = match cookies.get("session_id") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return None;
        }
    };

    let mut con = match redis_client.get_connection() {
        Ok(client) => client,
        Err(err) => {
            println!("{:#?}", err);
            return None;
        }
    };

    let user_id: String = match con.get(session_id) {
        Ok(client) => client,
        Err(err) => {
            println!("{:#?}", err);
            return None;
        }
    };

    Some(user_id)
}
