use std::collections::HashMap;

use geo_types::{Geometry, Point};
use geozero::wkb;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Transaction};

use crate::files::{RawAccident, RawInvolvedPerson};

type GeometryF64 = Geometry<f64>;
type PgTransaction<'a> = Transaction<'a, Postgres>;

/// データベースコネクションプールを返す。
///
/// # 戻り値
///
/// データベースコネクションプール
pub async fn connection_pool() -> anyhow::Result<Pool<Postgres>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}

/// 本票の都道府県コードとJIS規格の都道府県コードの対応を記録したハッシュマップを返す。
///
/// # 引数
///
/// * `pool` - データベースコネクションプール
///
/// # 戻り値
///
/// 本票の都道府県コードとJIS規格の都道府県コードの対応を記録したハッシュマップ
pub async fn prefecture_hash_map(pool: &Pool<Postgres>) -> anyhow::Result<HashMap<String, String>> {
    let mut prefectures = HashMap::new();
    let rows = sqlx::query!(
        r#"
        SELECT code, jis_code
        FROM prefectures
        "#
    )
    .fetch_all(pool)
    .await?;
    for row in rows {
        prefectures.insert(row.code, row.jis_code);
    }

    Ok(prefectures)
}

/// 交通事故をデータベースに登録する。
///
/// # 引数
///
/// * `tx` - データベーストランザクション
/// * `accidents` - 交通事故を格納したベクタ
///
/// # 戻り値
///
/// `()`
pub async fn register_accidents(
    tx: &mut PgTransaction<'_>,
    accidents: &[RawAccident],
) -> anyhow::Result<()> {
    for (index, accident) in accidents.iter().enumerate() {
        let location = Point::new(accident.location.x(), accident.location.y());
        let location: GeometryF64 = location.into();
        insert_accident(tx, accident, location).await.map_err(|e| {
            anyhow::anyhow!(
                "交通事故をデータベースに登録する際に、INSERT文を実行できませんでした。{}: {:?}: {}データ目",
                e,
                accident,
                index,
            )
        })?;
    }

    Ok(())
}

async fn insert_accident(
    tx: &mut PgTransaction<'_>,
    accident: &RawAccident,
    location: GeometryF64,
) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO accidents (
            id,
            prefecture_code,
            police_station_code,
            main_number,
            accident_detail_code,
            number_of_deaths,
            number_of_injuries,
            route_code,
            route_class_code,
            location_code,
            city_jis_code,
            occurred_at,
            day_night_code,
            sunrise_time,
            sunset_time,
            weather_code,
            district_code,
            surface_condition_code,
            road_model_code,
            traffic_signal_code,
            stop_regulation_sign_a_code,
            stop_regulation_display_a_code,
            stop_regulation_sign_b_code,
            stop_regulation_display_b_code,
            road_width_code,
            road_alignment_code,
            collision_point_code,
            zone_regulation_code,
            central_separation_code,
            road_segmentation_code,
            accident_type_code,
            age_a_code,
            age_b_code,
            party_a_code,
            party_b_code,
            purpose_a_code,
            purpose_b_code,
            vehicle_type_a_code,
            vehicle_type_b_code,
            automatic_a_code,
            automatic_b_code,
            support_car_a_code,
            support_car_b_code,
            speed_regulation_a_code,
            speed_regulation_b_code,
            collision_part_a,
            collision_part_b,
            vehicle_damage_a_code,
            vehicle_damage_b_code,
            airbag_a_code,
            airbag_b_code,
            side_airbag_a_code,
            side_airbag_b_code,
            injury_a_code,
            injury_b_code,
            location,
            week_code,
            holiday_code,
            cognitive_days_a,
            cognitive_days_b,
            driving_practice_a_code,
            driving_practice_b_code
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9,
            $10,
            $11,
            $12,
            $13,
            $14,
            $15,
            $16,
            $17,
            $18,
            $19,
            $20,
            $21,
            $22,
            $23,
            $24,
            $25,
            $26,
            $27,
            $28,
            $29,
            $30,
            $31,
            $32,
            $33,
            $34,
            $35,
            $36,
            $37,
            $38,
            $39,
            $40,
            $41,
            $42,
            $43,
            $44,
            $45,
            $46,
            $47,
            $48,
            $49,
            $50,
            $51,
            $52,
            $53,
            $54,
            $55,
            ST_SetSRID($56::geometry, 6668),
            $57,
            $58,
            $59,
            $60,
            $61,
            $62
        );"#,
        accident.id,
        accident.prefecture_code,
        accident.police_station_code,
        accident.main_number,
        accident.accident_detail_code,
        accident.number_of_deaths,
        accident.number_of_injuries,
        accident.route_code,
        accident.route_class_code,
        accident.location_code,
        accident.city_jis_code,
        accident.occurred_at,
        accident.day_night_code,
        accident.sunrise_time,
        accident.sunset_time,
        accident.weather_code,
        accident.district_code,
        accident.surface_condition_code,
        accident.road_model_code,
        accident.traffic_signal_code,
        accident.stop_regulation_sign_a_code,
        accident.stop_regulation_display_a_code,
        accident.stop_regulation_sign_b_code,
        accident.stop_regulation_display_b_code,
        accident.road_width_code,
        accident.road_alignment_code,
        accident.collision_point_code,
        accident.zone_regulation_code,
        accident.central_separation_code,
        accident.road_segmentation_code,
        accident.accident_type_code,
        accident.age_a_code,
        accident.age_b_code,
        accident.party_a_code,
        accident.party_b_code,
        accident.purpose_a_code,
        accident.purpose_b_code,
        accident.vehicle_type_a_code,
        accident.vehicle_type_b_code,
        accident.automatic_a_code,
        accident.automatic_b_code,
        accident.support_car_a_code,
        accident.support_car_b_code,
        accident.speed_regulation_a_code,
        accident.speed_regulation_b_code,
        accident.collision_part_a,
        accident.collision_part_b,
        accident.vehicle_damage_a_code,
        accident.vehicle_damage_b_code,
        accident.airbag_a_code,
        accident.airbag_b_code,
        accident.side_airbag_a_code,
        accident.side_airbag_b_code,
        accident.injury_a_code,
        accident.injury_b_code,
        wkb::Encode(location) as _,
        accident.week_code,
        accident.holiday_code,
        accident.cognitive_days_a,
        accident.cognitive_days_b,
        accident.driving_practice_a_code,
        accident.driving_practice_b_code,
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

/// 交通事故当事者以外の関係者をデータベースに登録する。
///
/// # 引数
///
/// * `tx` - データベーストランザクション
/// * `involved_persons` - 交通事故当事者以外の関係者を格納したベクタ
///
/// # 戻り値
///
/// `()`
pub async fn register_involved_persons(
    tx: &mut PgTransaction<'_>,
    involved_persons: &[RawInvolvedPerson],
) -> anyhow::Result<()> {
    for (index, involved_person) in involved_persons.iter().enumerate() {
        insert_involved_person(tx, involved_person).await.map_err(|e| {
            anyhow::anyhow!(
                "交通事故当事者以外の関係者をデータベースに登録する際に、INSERT文を実行できませんでした。{}: {:?}: {}データ目",
                e,
                involved_person,
                index,
            )
        })?;
    }

    Ok(())
}

async fn insert_involved_person(
    tx: &mut PgTransaction<'_>,
    involved_person: &RawInvolvedPerson,
) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO involved_persons (
            id,
            accident_id,
            sub_number,
            party_code,
            purpose_code,
            vehicle_type_code,
            riding_type_code,
            riding_class_code,
            support_car_code,
            airbag_code,
            side_airbag_code,
            injury_code,
            collision_part,
            vehicle_damage_code
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9,
            $10,
            $11,
            $12,
            $13,
            $14
        );
        "#,
        involved_person.id,
        involved_person.accident_id,
        involved_person.sub_number,
        involved_person.party_code,
        involved_person.purpose_code,
        involved_person.vehicle_type_code,
        involved_person.riding_type_code,
        involved_person.riding_class_code,
        involved_person.support_car_code,
        involved_person.airbag_code,
        involved_person.side_airbag_code,
        involved_person.injury_code,
        involved_person.collision_part,
        involved_person.vehicle_damage_code
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}
