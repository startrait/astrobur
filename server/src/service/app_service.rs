use crate::app_state::AppState;
use crate::database::models::{Url, User};
use crate::http::error::{BurError, ErrorResponse};
use crate::service::AuthenticatedUser;
use sqlx::postgres::PgPool;
use sqlx::Row;
use std::sync::Arc;
use qrcode::render::svg;
use qrcode::QrCode;
use tracing::{error,info,debug,event};
use tracing::Level;

pub async fn create_user(user: User, state: Arc<AppState>) -> Result<(), BurError> {
    let db: Arc<PgPool> = state.db.clone();

    check_if_exists(db.as_ref(), "users", "email", &user.email).await?;

    let mut tx = db.as_ref().begin().await?;

    let user_id: i32 =
        sqlx::query("INSERT INTO users(name,email,password) VALUES($1,$2,$3) RETURNING ID")
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.password)
            .fetch_one(&mut *tx)
            .await?
            .get("id");

    let _ = sqlx::query("INSERT INTO organizations(name,root_user) VALUES($1,$2) RETURNING ID")
        .bind(&user.name)
        .bind(&user_id)
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;
    event!(Level::INFO, message = "User created.", user = &user.email );

    Ok(())
}

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

async fn check_if_exists(
    db: &PgPool,
    table: &str,
    column: &str,
    value: &String,
) -> Result<(), BurError> {
    let query = format!("SELECT 1 FROM {} WHERE {} = $1", table, column);

    let result = sqlx::query(&query).bind(&value).fetch_optional(db).await?;

    match result {
        Some(_) => Err(BurError::CustomError(ErrorResponse {
            error_code: 500,
            reason: format!("{} already exists", column),
        })),
        None => Ok(()),
    }
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
