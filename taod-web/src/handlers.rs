use std::borrow::Cow;
use std::fmt::{Debug, Display};

use actix_web::http::{header::ContentType, StatusCode};
use actix_web::{web, HttpResponse, HttpResponseBuilder, Responder, ResponseError};
use geojson::{FeatureCollection, GeoJson};
use serde_json::to_string_pretty;
use serde_json::value::Value::{Number as SerdeNumber, String as SerdeString};
use sqlx::PgPool;

use crate::map::SRID_JGD2001;
use crate::map::{tile_bbox, TileCoordinate};
use crate::models::Accident;
use crate::settings::Settings;

/// アプリケーションエラーレスポンス
#[derive(Debug, serde::Serialize, thiserror::Error)]
pub enum AppErrorResponse {
    /// リクエストが不正
    BadRequest(AppErrorContent),
    /// サーバー内部エラー
    InternalServerError(AppErrorContent),
}

impl Display for AppErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for AppErrorResponse {
    fn status_code(&self) -> StatusCode {
        match self {
            AppErrorResponse::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppErrorResponse::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        match self {
            AppErrorResponse::BadRequest(content) => HttpResponse::build(status_code)
                .json(AppResponseErrorBody::new(status_code, content)),
            AppErrorResponse::InternalServerError(content) => HttpResponse::build(status_code)
                .json(AppResponseErrorBody::new(status_code, content)),
        }
    }
}

/// アプリケーションエラーコンテンツ
#[derive(Debug, serde::Serialize)]
pub struct AppErrorContent {
    /// アプリケーションエラー
    pub app_error: Option<AppError>,
    /// エラーメッセージ
    pub message: Cow<'static, str>,
}

/// アプリケーションエラー
#[derive(Debug, Clone, Copy, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum AppError {
    /// 特に説明を必要としないエラー
    None = 0,
    /// データベースエラー
    Database = 1,
    /// 交通事故ズームレベルエラー
    AccidentZoomLevel = 2,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppResponseErrorBody {
    /// レスポンスステータスコード
    pub status_code: u16,
    /// アプリケーションエラー
    #[serde(rename(serialize = "appErrorCode"))]
    pub app_error: Option<AppError>,
    /// エラーメッセージ
    pub message: String,
}

impl AppResponseErrorBody {
    fn new(status_code: StatusCode, content: &AppErrorContent) -> Self {
        Self {
            status_code: status_code.as_u16(),
            app_error: content.app_error,
            message: content.message.to_string(),
        }
    }
}

/// ヘルスチェックハンドラ
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// 交通事故リストハンドラ
pub async fn accident_list(
    settings: web::Data<Settings>,
    pool: web::Data<PgPool>,
    tile_coordinate: web::Path<TileCoordinate>,
) -> actix_web::Result<HttpResponse> {
    // ズームレベルを確認
    if tile_coordinate.z < settings.web_app.accident_zoom_level {
        return Err(AppErrorResponse::BadRequest(AppErrorContent {
            app_error: Some(AppError::AccidentZoomLevel),
            message: format!(
                "交通事故はズームレベル{}以上から取得できます。",
                settings.web_app.accident_zoom_level
            )
            .into(),
        })
        .into());
    }
    // タイル内の交通事故を取得
    let bbox = tile_bbox(tile_coordinate.into_inner());
    let bbox = bbox.extent(settings.web_app.accident_buffer_ratio);
    let accidents = sqlx::query_as!(
        Accident,
        r#"
        SELECT
            a.id,
            ci.prefecture_jis_code prefecture_code,
            pr.name prefecture_name,
            CONCAT(a.prefecture_code, a.police_station_code) police_station_code,
            po.police_station_name,
            a.city_jis_code city_code,
            ci.city_name,
            a.occurred_at,
            a.number_of_deaths,
            a.number_of_injuries,
            a.weather_code,
            we.name weather_name,
            a.surface_condition_code,
            su.name surface_condition_name,
            a.location as "location!: _"
        FROM accidents a
        INNER JOIN prefectures pr ON a.prefecture_code = pr.code
        INNER JOIN police_stations po ON a.prefecture_code = po.prefecture_code
            AND a.police_station_code = po.police_station_code
        INNER JOIN cities ci ON a.city_jis_code = ci.city_jis_code
        INNER JOIN weathers we ON a.weather_code = we.code
        INNER JOIN surface_conditions su ON a.surface_condition_code = su.code
        WHERE
            ST_CONTAINS(
                ST_MakeEnvelope(
                    $1,
                    $2,
                    $3,
                    $4,
                    $5
                ),
                a.location
            )
        "#,
        bbox.x_min,
        bbox.y_min,
        bbox.x_max,
        bbox.y_max,
        SRID_JGD2001 as i32,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        AppErrorResponse::InternalServerError(AppErrorContent {
            app_error: Some(AppError::Database),
            message: e.to_string().into(),
        })
    })?;

    // GeoJSONに変換
    let features = accidents
        .into_iter()
        .map(|accident| {
            let properties = Some(accident_properties(&accident));
            let geometry: Option<geojson::Geometry> =
                Some(geojson::Value::from(&accident.location.geometry.unwrap()).into());
            let id = Some(geojson::feature::Id::String(accident.id.to_string()));
            geojson::Feature {
                bbox: None,
                geometry,
                id,
                properties,
                foreign_members: None,
            }
        })
        .collect::<Vec<_>>();
    let feature_collection = FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    };
    let geo_json = GeoJson::from(feature_collection).to_string();
    let response = HttpResponseBuilder::new(StatusCode::OK)
        .content_type(ContentType::json())
        .body(geo_json);

    Ok(response)
}

fn accident_properties(accident: &Accident) -> geojson::JsonObject {
    let mut props = geojson::JsonObject::new();
    props.insert("id".to_string(), SerdeString(accident.id.to_string()));
    props.insert(
        "prefectureCode".to_string(),
        SerdeString(accident.prefecture_code.to_string()),
    );
    props.insert(
        "prefectureName".to_string(),
        SerdeString(accident.prefecture_name.to_string()),
    );
    props.insert(
        "cityCode".to_string(),
        SerdeString(accident.city_code.to_string()),
    );
    props.insert(
        "cityName".to_string(),
        SerdeString(accident.city_name.to_string()),
    );
    props.insert(
        "policeStationCode".to_string(),
        SerdeString(accident.police_station_code.as_ref().unwrap().to_string()),
    );
    props.insert(
        "policeStationName".to_string(),
        SerdeString(accident.police_station_name.to_string()),
    );
    props.insert(
        "occurredAt".to_string(),
        SerdeString(
            accident
                .occurred_at
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap(),
        ),
    );
    props.insert(
        "numberOfDeaths".to_string(),
        SerdeNumber(serde_json::Number::from(accident.number_of_deaths)),
    );
    props.insert(
        "numberOfInjuries".to_string(),
        SerdeNumber(serde_json::Number::from(accident.number_of_injuries)),
    );
    props.insert(
        "weatherCode".to_string(),
        SerdeString(accident.weather_code.to_string()),
    );
    props.insert(
        "weatherName".to_string(),
        SerdeString(accident.weather_name.to_string()),
    );
    props.insert(
        "surfaceConditionCode".to_string(),
        SerdeString(accident.surface_condition_code.to_string()),
    );
    props.insert(
        "surfaceConditionName".to_string(),
        SerdeString(accident.surface_condition_name.to_string()),
    );

    props
}
