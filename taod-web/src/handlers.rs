use std::fmt::{Debug, Display};

use actix_web::http::{header::ContentType, StatusCode};
use actix_web::{web, HttpResponse, HttpResponseBuilder, Responder};
use geojson::{FeatureCollection, GeoJson};
use serde_json::value::Value::{Number as SerdeNumber, String as SerdeString};
use sqlx::PgPool;

use crate::models::Accident;

/// HTTPリクエストハンドラの戻り値の型
pub type HandlerResult = Result<HttpResponse, actix_web::error::Error>;

/// 500 Internal Server Errorを作成する。
///
/// # 引数
///
/// * `err` - エラー
///
/// # 戻り値
///
/// `actix_web::error::ErrorBadRequest`
pub fn e500<E>(err: E) -> actix_web::Error
where
    E: Debug + Display + 'static,
{
    actix_web::error::ErrorInternalServerError(err)
}

/// ヘルスチェックハンドラ
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// 交通事故リストハンドラ
pub async fn accident_list(pool: web::Data<PgPool>, path: web::Path<(String,)>) -> HandlerResult {
    let (prefecture_code,) = path.into_inner();
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
        WHERE ci.prefecture_jis_code = $1
        "#,
        prefecture_code
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(e500)?;

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
