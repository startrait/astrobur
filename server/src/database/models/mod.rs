use crate::http::response;
use chrono::NaiveDateTime;

pub struct User {
    pub id: i32,
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

impl Into<response::UrlInfo> for Url {
    fn into(self) -> response::UrlInfo {
        response::UrlInfo {
            code: self.code,
            destination: self.destination,
            track_qr_scans: self.track_qr_scans,
            query_parameters: self.query_parameters,
            qr_svg: None,
            active: self.active,
        }
    }
}
