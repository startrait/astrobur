use crate::http::api::response::url::UrlInfo;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub struct Url {
    pub track_qr_scans: bool,
    pub query_parameters: serde_json::Value,
    pub organization_id: i32,
    pub active: bool,
    pub expiry_date: Option<NaiveDateTime>,
    pub code: String,
    pub destination: String,
    pub tracked_data: Option<UrlTracking>,
}

impl Into<UrlInfo> for Url {
    fn into(self) -> UrlInfo {
        UrlInfo {
            code: self.code,
            destination: self.destination,
            track_qr_scans: self.track_qr_scans,
            query_parameters: self.query_parameters,
            qr_svg: None,
            active: self.active,
            tracked_data: self.tracked_data,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UrlTracking {
    pub url_id: i32,
    pub total_click_count: i32,
    pub qr_scan_count: i32,
}
