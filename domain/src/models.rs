use geo_types::Point;
use time::{OffsetDateTime, Time};
use uuid::Uuid;

/// 事故識別子
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawAccidentIdentifier {
    /// 都道府県コード
    pub prefecture_code: String,
    /// 警察署コード
    pub police_station_code: String,
    /// 本票番号
    pub main_number: u32,
}

/// 交通事故事故
pub struct RawAccident {
    /// 事故ID
    pub id: Uuid,
    /// 都道府県コード
    pub prefecture_code: String,
    /// 警察署コード
    pub police_station_code: String,
    /// 本票番号
    pub main_number: i32,
    /// 事故内容
    pub accident_detail_code: String,
    /// 死者数
    pub number_of_deaths: i32,
    /// 負傷者数
    pub number_of_injuries: i32,
    /// 路線コード
    pub route_code: String,
    /// 路線区分コード
    pub route_class_code: String,
    /// 地点コード
    pub location_code: i32,
    /// 市区町村コード
    pub city_jis_code: String,
    /// 発生日時
    pub occurred_at: OffsetDateTime,
    /// 昼夜コード
    pub day_night_code: String,
    /// 日の出時刻
    pub sunrise_time: Time,
    /// 日の入時刻
    pub sunset_time: Time,
    /// 天候コード
    pub weather_code: String,
    /// 地区コード
    pub district_code: String,
    /// 路面状態コード
    pub surface_condition_code: String,
    /// 道路形状コード
    pub road_model_code: String,
    /// 信号機コード
    pub traffic_signal_code: String,
    /// 一時停止規制標識コード（当事者A）
    pub stop_regulation_sign_a_code: String,
    /// 一時停止規制表示コード（当事者A）
    pub stop_regulation_display_a_code: String,
    /// 一時停止規制標識コード（当事者B）
    pub stop_regulation_sign_b_code: String,
    /// 一時停止規制表示コード（当事者B）
    pub stop_regulation_display_b_code: String,
    /// 車道幅員コード
    pub road_width_code: String,
    /// 道路線形コード
    pub road_alignment_code: String,
    /// 衝突地点コード
    pub collision_point_code: String,
    /// ゾーン規制コード
    pub zone_regulation_code: String,
    /// 中央分離帯施設コード
    pub central_separation_code: String,
    /// 歩車道区分コード
    pub road_segmentation_code: String,
    /// 事故類型コード
    pub accident_type_code: String,
    /// 年齢コード（当事者A）
    pub age_a_code: String,
    /// 年齢コード（当事者B）
    pub age_b_code: String,
    /// 当事者種別コード（当事者A）
    pub party_a_code: String,
    /// 当事者種別コード（当事者B）
    pub party_b_code: String,
    /// 用途コード（当事者A）
    pub purpose_a_code: String,
    /// 用途コード（当事者B）
    pub purpose_b_code: String,
    /// 車両種別コード（当事者A)
    pub vehicle_type_a_code: String,
    /// 車両種別コード（当事者B)
    pub vehicle_type_b_code: String,
    /// オートマチック車コード（当事者A）
    pub automatic_a_code: String,
    /// オートマチック車コード（当事者B）
    pub automatic_b_code: String,
    /// サポカーコード（当事者A）
    pub support_car_a_code: String,
    /// サポカーコード（当事者B）
    pub support_car_b_code: String,
    /// 速度規制（指定のみ）コード（当事者A）
    pub speed_regulation_a_code: String,
    /// 速度規制（指定のみ）コード（当事者B）
    pub speed_regulation_b_code: String,
    /// 車両の衝突部位（当事者A）
    pub collision_part_a: String,
    /// 車両の衝突部位（当事者b）
    pub collision_part_b: String,
    /// 車両の損壊程度コード（当事者A）
    pub vehicle_damage_a_code: String,
    /// 車両の損壊程度コード（当事者B）
    pub vehicle_damage_b_code: String,
    /// エアバッグの装備コード（当事者A）
    pub airbag_a_code: String,
    /// エアバッグの装備コード（当事者B）
    pub airbag_b_code: String,
    /// サイドエアバッグの装備コード（当事者A）
    pub side_airbag_a_code: String,
    /// サイドエアバッグの装備コード（当事者B）
    pub side_airbag_b_code: String,
    /// 人身損傷程度コード（当事者A）
    pub injury_a_code: String,
    /// 人身損傷程度コード（当事者B）
    pub injury_b_code: String,
    /// 地点（JGD2011）
    pub location: Point,
    /// 曜日コード
    pub week_code: String,
    /// 祝日コード
    pub holiday_code: String,
    /// 認知機能検査経過日数コード（当事者A）
    pub cognitive_days_a: i32,
    /// 認知機能検査経過日数コード（当事者B）
    pub cognitive_days_b: i32,
    /// 運転練習の方法コード（当事者A）
    pub driving_practice_a_code: String,
    /// 運転練習の方法コード（当事者B）
    pub driving_practice_b_code: String,
}

/// 交通事故当事者以外の関与者
pub struct RawInvolvedPerson {
    /// 事故関与者ID
    pub id: Uuid,
    /// 事故ID
    pub accident_id: Uuid,
    /// 補充票番号
    pub sub_number: i32,
    /// 当事者種別コード
    pub party_code: String,
    /// 用途別コード
    pub purpose_code: Option<String>,
    /// 車両形状等コード
    pub vehicle_type_code: Option<String>,
    /// サポカーコード
    pub support_car_code: String,
    /// エアバッグの装備コード
    pub airbag_code: String,
    /// サイドエアバッグの装備コード
    pub side_airbag_code: String,
    /// 人身損傷程度コード
    pub injury_code: String,
    /// 車両の衝突部位コード
    pub collision_part: String,
    /// 車両の損壊程度
    pub vehicle_damage_code: Option<String>,
}
