use std::borrow::Cow;
use std::fmt::{Debug, Display};

use actix_web::body::MessageBody;
use actix_web::dev::ServiceResponse;
use actix_web::http::header::{self, ContentType};
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{web, HttpResponse, HttpResponseBuilder, Responder, ResponseError};
use geojson::{FeatureCollection, GeoJson};
use serde_json::to_string_pretty;
use serde_json::value::Value::{Number as SerdeNumber, String as SerdeString};
use sqlx::PgPool;

use geometries::WkbGeometryF64;

use crate::map::{tile_bbox, TileCoordinate};
use crate::map::{BBox, SRID_JGD2001};
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
    pub app_error: AppError,
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
    #[serde(serialize_with = "serialize_status_code")]
    pub status_code: StatusCode,
    /// アプリケーションエラー
    #[serde(rename(serialize = "appErrorCode"))]
    pub app_error: AppError,
    /// エラーメッセージ
    pub message: String,
}

fn serialize_status_code<S: serde::Serializer>(
    status_code: &StatusCode,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_u16(status_code.as_u16())
}

impl AppResponseErrorBody {
    fn new(status_code: StatusCode, content: &AppErrorContent) -> Self {
        Self {
            status_code,
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
    let bbox = calculate_extend_accident_bbox(
        tile_coordinate.into_inner(),
        settings.web_app.accident_zoom_level,
        settings.web_app.accident_buffer_ratio,
    )?;
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
            a.location as "location!: WkbGeometryF64"
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
            app_error: AppError::Database,
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

/// 交通事故リストハンドラ
///
/// PostGISから直接GeoJSONを取得する。
pub async fn accident_list_geojson(
    settings: web::Data<Settings>,
    pool: web::Data<PgPool>,
    tile_coordinate: web::Path<TileCoordinate>,
) -> actix_web::Result<HttpResponse> {
    let bbox = calculate_extend_accident_bbox(
        tile_coordinate.into_inner(),
        settings.web_app.accident_zoom_level,
        settings.web_app.accident_buffer_ratio,
    )?;

    let record = sqlx::query!(
        r#"
        SELECT json_build_object(
            'type', 'FeatureCollection',
            'features', json_agg(
                json_build_object(
                    'type', 'Feature',
                    'id', id,
                    'geometry', ST_AsGeoJSON(location, 9, 0)::json,
                    'properties', json_build_object(
                        'prefectureCode', prefecture_code,
                        'prefectureName', prefecture_name,
                        'cityCode', city_code,
                        'cityName', city_name,
                        'policeStationCode', police_station_code,
                        'policeStationName', police_station_name,
                        'occurredAt', occurred_at,
                        'numberOfDeaths', number_of_deaths,
                        'numberOfInjuries', number_of_injuries,
                        'weatherCode', weather_code,
                        'weatherName', weather_name,
                        'surfaceConditionCode', surface_condition_code,
                        'surfaceConditionName', surface_condition_name
                    )
                )
            )
        ) as "features!: sqlx::types::Json<serde_json::Value>"
        FROM (
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
                a.location
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
        )
        "#,
        bbox.x_min,
        bbox.y_min,
        bbox.x_max,
        bbox.y_max,
        SRID_JGD2001 as i32,
    )
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| {
        AppErrorResponse::InternalServerError(AppErrorContent {
            app_error: AppError::Database,
            message: e.to_string().into(),
        })
    })?;

    match record.features.get("features") {
        Some(serde_json::Value::Array(_)) => Ok(HttpResponseBuilder::new(StatusCode::OK)
            .content_type(ContentType::json())
            .body(record.features.to_string())),
        _ => Ok(HttpResponseBuilder::new(StatusCode::OK)
            .content_type(ContentType::json())
            .body(r#"{"features": [], "type": "FeatureCollection"}"#)),
    }
}

fn calculate_extend_accident_bbox(
    tile_coordinate: TileCoordinate,
    accident_zoom_level: u8,
    accident_buffer_ratio: f64,
) -> Result<BBox, AppErrorResponse> {
    // ズームレベルを確認
    if tile_coordinate.z < accident_zoom_level {
        return Err(AppErrorResponse::BadRequest(AppErrorContent {
            app_error: AppError::AccidentZoomLevel,
            message: format!(
                "交通事故はズームレベル{}以上から取得できます。",
                accident_zoom_level
            )
            .into(),
        }));
    }
    // タイルのバウンディングボックスを計算
    let bbox = tile_bbox(tile_coordinate);

    // バウンディングボックスを拡張
    Ok(bbox.extend(accident_buffer_ratio))
}

fn accident_properties(accident: &Accident) -> geojson::JsonObject {
    let mut props = geojson::JsonObject::new();
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

/// actix-webがエクストラクタで発生したエラーなどをJSONに変換するミドルウェア
pub fn default_error_handler<B>(
    mut service_response: ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>>
where
    B: actix_web::body::MessageBody,
    <B as MessageBody>::Error: std::fmt::Debug,
{
    // Content-Typeがapplication/jsonの場合はそのまま返す
    let content_type = service_response.headers().get(header::CONTENT_TYPE);
    if content_type.is_some() && content_type.unwrap() == "application/json" {
        return Ok(ErrorHandlerResponse::Response(
            service_response.map_into_left_body(),
        ));
    }

    // レスポンスヘッダーにContent-Typeを追加して、リクエストとレスポンスに分離
    service_response.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    let status_code = service_response.response().status();
    let (request, response) = service_response.into_parts();
    // actix-webが返すエラーレスポンスのボディは短いため同期処理
    let body_bytes =
        futures::executor::block_on(actix_http::body::to_bytes(response.into_body())).unwrap();
    // ボディを文字列に変換
    let body = std::str::from_utf8(&body_bytes).unwrap_or("Something wrong with me");
    // レスポンスを作成
    let body = AppResponseErrorBody {
        status_code,
        app_error: AppError::None,
        message: body.to_string(),
    };
    let response = HttpResponse::build(status_code).body(serde_json::to_string(&body).unwrap());
    let response = ServiceResponse::new(request, response)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(response))
}
