use uuid::Uuid;

pub fn generate_uuid() -> String {
    let user_id = Uuid::new_v4();
    user_id.to_string()
}

pub fn generate_session_id() -> String {
    let user_id = Uuid::new_v4().to_string();
    user_id[0..13].to_string()
}
