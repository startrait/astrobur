use crate::database::models::url::Url;
use crate::error::BurError;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct UrlGenerationRequest {
    pub track_qr_scans: bool,
    pub query_parameters: Option<HashMap<String, String>>,
    pub organization_id: i32,
    pub active: bool,
    pub expiry_date: Option<String>,
    pub code: String,
    pub destination: String,
}

impl TryInto<Url> for UrlGenerationRequest {
    type Error = BurError;

    fn try_into(self) -> Result<Url, Self::Error> {
        let query_params = serde_json::to_value(self.query_parameters)?;
        let expiry_date = match self.expiry_date {
            Some(exp_date) => Some(NaiveDateTime::parse_from_str(
                &exp_date,
                "%Y-%m-%d %H:%M:%S",
            )?),
            None => None,
        };

        // if let Some ()
        Ok(Url {
            track_qr_scans: self.track_qr_scans,
            query_parameters: query_params,
            organization_id: self.organization_id,
            active: self.active,
            expiry_date: expiry_date,
            code: self.code,
            destination: self.destination,
            tracked_data: None
        })
    }
}
