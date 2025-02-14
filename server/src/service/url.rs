use crate::app::AppState;
use crate::database::models::url::Url;
use crate::error::BurError;
use crate::service::check_if_exists;
use qrcode::render::svg;
use qrcode::QrCode;
use sqlx::postgres::PgPool;
use sqlx::Row;
use std::sync::Arc;

pub async fn create_url(state: Arc<AppState>, url: Url) -> Result<Url, BurError> {
    check_if_exists(state.db.as_ref(), "urls", "code", &url.code).await?;

    let mut tx = state.db.begin().await?;
    let url_id: i32 = sqlx::query(
        "INSERT INTO urls(
            code,
            destination,
            query_parameters,
            organization_id,
            active,
            expiry_date,
            track_qr_scans
        ) VALUES($1,$2,$3,$4,$5,$6,$7) RETURNING ID",
    )
    .bind(&url.code)
    .bind(&url.destination)
    .bind(&url.query_parameters)
    .bind(&url.organization_id)
    .bind(&url.active)
    .bind(&url.expiry_date)
    .bind(&url.track_qr_scans)
    .fetch_one(&mut *tx)
    .await?
    .get("id");

    let _ = sqlx::query(
        "INSERT INTO url_trackings(url_id,total_click_count,qr_scan_count) VALUES ($1, 0,0)",
    )
    .bind(&url_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(url)
}

pub async fn get_url_details_from_code(db: &PgPool, code: &str) -> Result<Url, BurError> {
    let row = sqlx::query(
        "
        SELECT 
                destination,
                query_parameters,
                organization_id,
                active,
                expiry_date,
                track_qr_scans
        FROM urls
        WHERE code = $1
    ",
    )
    .bind(&code)
    .fetch_one(db)
    .await?;

    let url = Url {
        code: code.to_owned(),
        track_qr_scans: row.get("track_qr_scans"),
        query_parameters: row.get("query_parameters"),
        organization_id: row.get("organization_id"),
        active: row.get("active"),
        expiry_date: row.get("expiry_date"),
        destination: row.get("destination"),
    };

    Ok(url)
}

pub fn get_qr_svg(url: &str) -> Result<String, BurError> {
    let code = QrCode::new(url).unwrap();
    let svg_xml = code.render::<svg::Color>().build();
    Ok(svg_xml)
}
