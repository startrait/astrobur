use chrono::NaiveDateTime;
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct Url {
    pub track_qr_scans: bool,
    pub query_parameters: serde_json::Value,
    pub organization_id: i32,
    pub active: bool,
    pub expiry_date: Option<NaiveDateTime>,
    pub code: String,
    pub destination: String,
}
