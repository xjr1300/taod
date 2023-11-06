use std::collections::HashMap;
use std::path::Path;

use geo_types::Point;
use time::macros::offset;
use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time};

use crate::models::RawAccident;

fn read_str_column(
    row: &csv::StringRecord,
    row_index: usize,
    column_index: usize,
) -> anyhow::Result<String> {
    let value = row.get(column_index).ok_or(anyhow::anyhow!(
        "{}行目: 列番号が範囲外です。",
        row_index + 1
    ))?;

    Ok(value.to_string())
}

fn read_i32_column(
    row: &csv::StringRecord,
    row_index: usize,
    column_index: usize,
) -> anyhow::Result<i32> {
    let value = row.get(column_index).ok_or(anyhow::anyhow!(
        "{}行目: 列番号が範囲外です。",
        row_index + 1
    ))?;

    value.parse::<i32>().map_err(|_| {
        anyhow::anyhow!(
            "{}行目 {}列: 数値に変換できません。",
            row_index + 1,
            column_index + 1
        )
    })
}

fn read_datetime_columns(
    row: &csv::StringRecord,
    row_index: usize,
    column_index: usize,
) -> anyhow::Result<OffsetDateTime> {
    let year = read_i32_column(row, row_index, column_index)?;
    let month = read_i32_column(row, row_index, column_index + 1)? as u8;
    let day = read_i32_column(row, row_index, column_index + 2)? as u8;
    let hour = read_i32_column(row, row_index, column_index + 3)? as u8;
    let minute = read_i32_column(row, row_index, column_index + 4)? as u8;

    offset_datetime(year, month, day, hour, minute)
        .map_err(|e| anyhow::anyhow!("{}行目 {}列: {}", row_index + 1, column_index + 1, e))
}

fn offset_datetime(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
) -> anyhow::Result<OffsetDateTime> {
    let month =
        Month::try_from(month).map_err(|_| anyhow::anyhow!("月({})が範囲外です。", month))?;
    let date = Date::from_calendar_date(year, month as time::Month, day)?;
    let time = Time::from_hms(hour, minute, 0u8)
        .map_err(|_| anyhow::anyhow!("時刻({}:{})が範囲外です。", hour, minute))?;
    let datetime = PrimitiveDateTime::new(date, time);

    Ok(datetime.assume_offset(offset!(+9)))
}

fn read_time_columns(
    row: &csv::StringRecord,
    row_index: usize,
    column_index: usize,
) -> anyhow::Result<Time> {
    let hour = read_i32_column(row, row_index, column_index)? as u8;
    let minute = read_i32_column(row, row_index, column_index + 1)? as u8;

    time(hour, minute).map_err(|_| {
        anyhow::anyhow!(
            "{}行目 {}列: 時刻({}:{})が範囲外です。",
            row_index + 1,
            column_index + 1,
            hour,
            minute
        )
    })
}

fn time(hour: u8, minute: u8) -> anyhow::Result<Time> {
    Time::from_hms(hour, minute, 0u8)
        .map_err(|_| anyhow::anyhow!("時刻({}:{})が範囲外です。", hour, minute))
}

fn read_point_column(
    row: &csv::StringRecord,
    row_index: usize,
    column_index: usize,
) -> anyhow::Result<Point> {
    let latitude = dms_to_latitude(&read_str_column(row, row_index, column_index)?)?;
    let longitude = dms_to_longitude(&read_str_column(row, row_index, column_index + 1)?)?;

    Ok(Point::new(longitude, latitude))
}

fn dms_to_latitude(hms: &str) -> anyhow::Result<f64> {
    let degree = hms[..2].parse::<f64>()?;
    let minute = hms[2..4].parse::<f64>()?;
    let second = hms[4..].parse::<f64>()? / 1000.0;

    Ok(degree + minute / 60.0 + second / 3600.0)
}

fn dms_to_longitude(hms: &str) -> anyhow::Result<f64> {
    let degree = hms[..3].parse::<f64>()?;
    let minute = hms[3..5].parse::<f64>()?;
    let second = hms[5..].parse::<f64>()? / 1000.0;

    Ok(degree + minute / 60.0 + second / 3600.0)
}

fn row_to_accident(
    row: &csv::StringRecord,
    row_index: usize,
    cities: &HashMap<String, String>,
) -> anyhow::Result<RawAccident> {
    let prefecture_code = read_str_column(row, row_index, 1)?;
    let route = read_str_column(row, row_index, 7)?;
    let city_code = read_str_column(row, row_index, 9)?;
    let city_jis_code = cities
        .get(&format!("{}{}", prefecture_code, city_code))
        .ok_or(anyhow::anyhow!(
            "{}行目: 市区町村コード({})が見つかりません。",
            row_index + 2,
            city_code
        ))?
        .to_string();

    Ok(RawAccident {
        id: uuid::Uuid::new_v4(),
        prefecture_code,
        police_station_code: read_str_column(row, row_index, 2)?,
        main_number: read_i32_column(row, row_index, 3)?,
        accident_detail_code: read_str_column(row, row_index, 4)?,
        number_of_deaths: read_i32_column(row, row_index, 5)?,
        number_of_injuries: read_i32_column(row, row_index, 6)?,
        route_code: route[0..4].to_string(),
        route_class_code: route[4..5].to_string(),
        location_code: read_i32_column(row, row_index, 8)?,
        city_jis_code,
        occurred_at: read_datetime_columns(row, row_index, 10)?,
        day_night_code: read_str_column(row, row_index, 15)?,
        sunrise_time: read_time_columns(row, row_index, 16)?,
        sunset_time: read_time_columns(row, row_index, 18)?,
        weather_code: read_str_column(row, row_index, 20)?,
        district_code: read_str_column(row, row_index, 21)?,
        surface_condition_code: read_str_column(row, row_index, 22)?,
        road_model_code: read_str_column(row, row_index, 23)?,
        traffic_signal_code: read_str_column(row, row_index, 24)?,
        stop_regulation_sign_a_code: read_str_column(row, row_index, 25)?,
        stop_regulation_display_a_code: read_str_column(row, row_index, 26)?,
        stop_regulation_sign_b_code: read_str_column(row, row_index, 27)?,
        stop_regulation_display_b_code: read_str_column(row, row_index, 28)?,
        road_width_code: read_str_column(row, row_index, 29)?,
        road_alignment_code: read_str_column(row, row_index, 30)?,
        collision_point_code: read_str_column(row, row_index, 31)?,
        zone_regulation_code: read_str_column(row, row_index, 32)?,
        central_separation_code: read_str_column(row, row_index, 33)?,
        road_segmentation_code: read_str_column(row, row_index, 34)?,
        accident_type_code: read_str_column(row, row_index, 35)?,
        age_a_code: read_str_column(row, row_index, 36)?,
        age_b_code: read_str_column(row, row_index, 37)?,
        party_a_code: read_str_column(row, row_index, 38)?,
        party_b_code: read_str_column(row, row_index, 39)?,
        purpose_a_code: read_str_column(row, row_index, 40)?,
        purpose_b_code: read_str_column(row, row_index, 41)?,
        vehicle_type_a_code: read_str_column(row, row_index, 42)?,
        vehicle_type_b_code: read_str_column(row, row_index, 43)?,
        automatic_a_code: read_str_column(row, row_index, 44)?,
        automatic_b_code: read_str_column(row, row_index, 45)?,
        support_car_a_code: read_str_column(row, row_index, 46)?,
        support_car_b_code: read_str_column(row, row_index, 47)?,
        speed_regulation_a_code: read_str_column(row, row_index, 48)?,
        speed_regulation_b_code: read_str_column(row, row_index, 49)?,
        collision_part_a: read_str_column(row, row_index, 50)?,
        collision_part_b: read_str_column(row, row_index, 51)?,
        vehicle_damage_a_code: read_str_column(row, row_index, 52)?,
        vehicle_damage_b_code: read_str_column(row, row_index, 53)?,
        airbag_a_code: read_str_column(row, row_index, 54)?,
        airbag_b_code: read_str_column(row, row_index, 55)?,
        side_airbag_a_code: read_str_column(row, row_index, 56)?,
        side_airbag_b_code: read_str_column(row, row_index, 57)?,
        injury_a_code: read_str_column(row, row_index, 58)?,
        injury_b_code: read_str_column(row, row_index, 59)?,
        location: read_point_column(row, row_index, 60)?,
        week_code: read_str_column(row, row_index, 62)?,
        holiday_code: read_str_column(row, row_index, 63)?,
        cognitive_days_a: read_i32_column(row, row_index, 64)?,
        cognitive_days_b: read_i32_column(row, row_index, 65)?,
        driving_practice_a_code: read_str_column(row, row_index, 66)?,
        driving_practice_b_code: read_str_column(row, row_index, 67)?,
    })
}

/// 本票を読み込み、交通事故を返す。
///
/// # 引数
///
/// * `path` - 本票のファイルパス
/// * `cities` - 本票の市区町村コードとJIS規格の市区町村コードの対応を記録したハッシュマップ
///
/// # 戻り値
///
/// 交通事故を格納したベクタ
pub fn read_accidents<P: AsRef<Path>>(
    path: P,
    cities: &HashMap<String, String>,
) -> anyhow::Result<Vec<RawAccident>> {
    let file = std::fs::read(path)?;
    let (reader, _, _) = encoding_rs::SHIFT_JIS.decode(&file);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader.as_bytes());

    let mut accidents = Vec::new();
    for (row_index, row) in reader.records().enumerate() {
        let row = row.unwrap();
        accidents.push(row_to_accident(&row, row_index, cities)?);
    }

    Ok(accidents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::{datetime, offset, time};

    #[test]
    fn offset_datetime_ok() {
        let datetime = offset_datetime(2021, 1, 1, 10, 30).unwrap();
        assert_eq!(datetime.year(), 2021);
        assert_eq!(datetime.month(), Month::January);
        assert_eq!(datetime.day(), 1);
        assert_eq!(datetime.hour(), 10);
        assert_eq!(datetime.minute(), 30);
        assert_eq!(datetime.second(), 0);
        assert_eq!(datetime.offset(), offset!(+9));
    }

    #[test]
    fn offset_datetime_fail() {
        assert!(offset_datetime(2021, 13, 1, 10, 30).is_err());
        assert!(offset_datetime(2021, 1, 32, 10, 30).is_err());
        assert!(offset_datetime(2021, 1, 1, 24, 30).is_err());
        assert!(offset_datetime(2021, 1, 1, 10, 60).is_err());
    }

    #[test]
    fn time_ok() {
        let time = time(10, 30).unwrap();
        assert_eq!(time.hour(), 10);
        assert_eq!(time.minute(), 30);
        assert_eq!(time.second(), 0);
    }

    #[test]
    fn time_fail() {
        assert!(time(24, 30).is_err());
        assert!(time(10, 60).is_err());
    }

    #[test]
    fn dms_to_latitude_ok() {
        assert_eq!(dms_to_latitude("350000000").unwrap(), 35.0);
        assert_eq!(dms_to_latitude("353000000").unwrap(), 35.0 + 30.0 / 60.0);
        let result = dms_to_latitude("353030123").unwrap();
        let expected = 35.0 + 30.0 / 60.0 + 30.123 / 3600.0;
        assert!(
            (result - expected).abs() < 1e-11,
            "expected: {}, result: {}",
            expected,
            result
        );
    }

    #[test]
    fn dms_to_longitude_ok() {
        assert_eq!(dms_to_longitude("1350000000").unwrap(), 135.0);
        assert_eq!(dms_to_longitude("1353000000").unwrap(), 135.0 + 30.0 / 60.0);
        let result = dms_to_longitude("1353030123").unwrap();
        let expected = 135.0 + 30.0 / 60.0 + 30.123 / 3600.0;
        assert!(
            (result - expected).abs() < 1e-11,
            "expected: {}, result: {}",
            expected,
            result
        );
    }

    fn city_hash_map() -> HashMap<String, String> {
        let mut cities = HashMap::new();
        cities.insert(String::from("10104"), String::from("01104"));

        cities
    }

    #[test]
    fn row_to_accident_ok() {
        let row = "1,10,059,0001,2,000,001,40010,0000,104,2022,01,22,14,18,12,06,59,16,33,5,1,3,14,7,00,00,00,00,04,9,01,70,1,4,21,35,25,03,04,31,31,01,01,1,1,00,00,04,04,30,30,3,3,2,2,2,2,2,4,430234789,1412612831,7,3,9999,9999,1,1";
        let row = row.split(",").collect::<Vec<&str>>();
        let row = csv::ByteRecord::from(row);
        let row = csv::StringRecord::from_byte_record(row).unwrap();
        let cities = city_hash_map();
        let accident = row_to_accident(&row, 0, &cities).unwrap();

        assert_eq!(accident.prefecture_code, "10");
        assert_eq!(accident.police_station_code, "059");
        assert_eq!(accident.main_number, 1);
        assert_eq!(accident.accident_detail_code, "2");
        assert_eq!(accident.number_of_deaths, 0);
        assert_eq!(accident.number_of_injuries, 1);
        assert_eq!(accident.route_code, "4001");
        assert_eq!(accident.route_class_code, "0");
        assert_eq!(accident.location_code, 0);
        assert_eq!(accident.city_jis_code, "01104");
        assert_eq!(
            accident.occurred_at,
            datetime!(2022-01-22 14:18).assume_offset(offset!(+9))
        );
        assert_eq!(accident.day_night_code, "12");
        assert_eq!(accident.sunrise_time, time!(06:59));
        assert_eq!(accident.sunset_time, time!(16:33));
        assert_eq!(accident.weather_code, "5");
        assert_eq!(accident.district_code, "1");
        assert_eq!(accident.surface_condition_code, "3");
        assert_eq!(accident.road_model_code, "14");
        assert_eq!(accident.traffic_signal_code, "7");
        assert_eq!(accident.stop_regulation_sign_a_code, "00");
        assert_eq!(accident.stop_regulation_display_a_code, "00");
        assert_eq!(accident.stop_regulation_sign_b_code, "00");
        assert_eq!(accident.stop_regulation_display_b_code, "00");
        assert_eq!(accident.road_width_code, "04");
        assert_eq!(accident.road_alignment_code, "9");
        assert_eq!(accident.collision_point_code, "01");
        assert_eq!(accident.zone_regulation_code, "70");
        assert_eq!(accident.central_separation_code, "1");
        assert_eq!(accident.road_segmentation_code, "4");
        assert_eq!(accident.accident_type_code, "21");
        assert_eq!(accident.age_a_code, "35");
        assert_eq!(accident.age_b_code, "25");
        assert_eq!(accident.party_a_code, "03");
        assert_eq!(accident.party_b_code, "04");
        assert_eq!(accident.purpose_a_code, "31");
        assert_eq!(accident.purpose_b_code, "31");
        assert_eq!(accident.vehicle_type_a_code, "01");
        assert_eq!(accident.vehicle_type_b_code, "01");
        assert_eq!(accident.automatic_a_code, "1");
        assert_eq!(accident.automatic_b_code, "1");
        assert_eq!(accident.support_car_a_code, "00");
        assert_eq!(accident.support_car_b_code, "00");
        assert_eq!(accident.speed_regulation_a_code, "04");
        assert_eq!(accident.speed_regulation_b_code, "04");
        assert_eq!(accident.collision_part_a, "30");
        assert_eq!(accident.collision_part_b, "30");
        assert_eq!(accident.vehicle_damage_a_code, "3");
        assert_eq!(accident.vehicle_damage_b_code, "3");
        assert_eq!(accident.airbag_a_code, "2");
        assert_eq!(accident.airbag_b_code, "2");
        assert_eq!(accident.side_airbag_a_code, "2");
        assert_eq!(accident.side_airbag_b_code, "2");
        assert_eq!(accident.injury_a_code, "2");
        assert_eq!(accident.injury_b_code, "4");
        assert_eq!(
            accident.location,
            Point::new(
                141.0 + 26.0 / 60.0 + 12.831 / 3600.0,
                43.0 + 2.0 / 60.0 + 34.789 / 3600.0
            )
        );
        assert_eq!(accident.week_code, "7");
        assert_eq!(accident.holiday_code, "3");
        assert_eq!(accident.cognitive_days_a, 9999);
        assert_eq!(accident.cognitive_days_b, 9999);
        assert_eq!(accident.driving_practice_a_code, "1");
        assert_eq!(accident.driving_practice_b_code, "1");
    }
}
