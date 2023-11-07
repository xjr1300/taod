use geometries::GeometryF64;
use geozero::wkb;
use time::OffsetDateTime;
use uuid::Uuid;

/// 交通事故
#[derive(Debug)]
pub struct Accident {
    /// 交通事故ID
    pub id: Uuid,
    /// 都道府県コード
    pub prefecture_code: String,
    /// 都道府県名
    pub prefecture_name: String,
    /// 警察署コード
    pub police_station_code: Option<String>,
    /// 警察署名
    pub police_station_name: String,
    /// 市区町村コード
    pub city_code: String,
    /// 市区町村名
    pub city_name: String,
    /// 発生日時
    pub occurred_at: OffsetDateTime,
    /// 死亡者数
    pub number_of_deaths: i32,
    /// 負傷者数
    pub number_of_injuries: i32,
    /// 天候コード
    pub weather_code: String,
    /// 天候名
    pub weather_name: String,
    /// 路面状態コード
    pub surface_condition_code: String,
    /// 路面状態名
    pub surface_condition_name: String,
    /// 発生箇所
    pub location: wkb::Decode<GeometryF64>,
}
