use std::f64::consts::PI;

/// 日本測地系2011
pub const SRID_JGD2001: u32 = 6668;

/// 測地基準系1980(GRS80)楕円体長半径(m)
/// 日本測地型2011の楕円体における長半径
pub const GRS80_MAJOR_AXIS: f64 = 6378137.0;

/// タイル座標
#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub struct TileCoordinate {
    /// ズームレベル
    pub z: u8,
    /// X座標
    pub x: u32,
    /// Y座標
    pub y: u32,
}

/// 経度、緯度
#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    /// 経度(度)
    pub lon: f64,
    /// 緯度(度)
    pub lat: f64,
}

/// 範囲
#[derive(Debug, Clone, Copy)]
pub struct Range {
    /// 最小値
    pub min: f64,
    /// 最大値
    pub max: f64,
}

/// バウンダリーボックス
#[derive(Debug, Clone, Copy)]
pub struct BBox {
    /// X座標の最小値
    pub x_min: f64,
    /// Y座標の最小値
    pub y_min: f64,
    /// X座標の最大値
    pub x_max: f64,
    /// Y座標の最大値
    pub y_max: f64,
}

impl BBox {
    /// バウンダリーボックスを緯度方向の距離を基準に指定された率だけ上下左右方向に拡大する。
    ///
    /// # 引数
    ///
    /// * `ratio` - 拡大率
    ///
    /// # 戻り値
    ///
    /// 拡大したバウンダリーボックス
    pub fn extent(&self, ratio: f64) -> Self {
        let distance = (self.y_max - self.y_min) * ratio;

        Self {
            x_min: self.x_min - distance,
            y_min: self.y_min - distance,
            x_max: self.x_max + distance,
            y_max: self.y_max + distance,
        }
    }
}

/// 経度、緯度及びズームレベルからタイル座標を計算する。
///
/// # 引数
///
/// * coordinate: 緯度、経度
/// * `zoom` - ズームレベル
///
/// # 戻り値
///
/// タイル座標
pub fn degree_to_tile(coordinate: Coordinate, zoom: u8) -> TileCoordinate {
    let lat_rad = coordinate.lat.to_radians();
    let n = 1 << zoom;
    let x_tile = ((coordinate.lon + 180.0) / 360.0 * n as f64).floor() as u32;
    let y_tile = ((1.0 - lat_rad.tan().asinh() / PI) / 2.0 * n as f64).floor() as u32;

    TileCoordinate {
        z: zoom,
        x: x_tile,
        y: y_tile,
    }
}

/// タイル座標からタイルの左上の経度、緯度を計算する。
///
/// # 引数
///
/// * `tc` - タイル座標
///
/// # 戻り値
///
/// タイルの左上の経度、緯度
pub fn tile_to_degree(tc: TileCoordinate) -> Coordinate {
    let n = 1 << tc.z;
    let lon_deg = tc.x as f64 / n as f64 * 360.0 - 180.0;
    let lat_rad = (PI * (1.0 - 2.0 * tc.y as f64 / n as f64)).sinh().atan();
    let lat_deg = lat_rad.to_degrees();

    Coordinate {
        lon: lon_deg,
        lat: lat_deg,
    }
}

/// メルカトル座標系のY座標から緯度を計算する。
///
/// # 引数
///
/// * `y` - メルカトル座標系のY座標(m)
///
/// # 戻り値
///
/// 緯度(度)
fn mercator_to_latitude(y: f64) -> f64 {
    y.sinh().atan().to_degrees()
}

/// タイルの緯度の範囲を計算する。
///
/// # 引数
///
/// * `tile_y` - タイル座標のX座標
///
/// # 戻り値
///
/// 緯度の範囲
pub fn tile_latitude_range(tile_y: u32, zoom: u8) -> Range {
    let n = (1 << zoom) as f64;
    let unit = 1.0 / n;
    let y1 = tile_y as f64 * unit;
    let yw = y1 + unit;
    let min = mercator_to_latitude(PI * (1.0 - 2.0 * yw));
    let max = mercator_to_latitude(PI * (1.0 - 2.0 * y1));

    Range { min, max }
}

/// タイルの経度の範囲を計算する。
///
/// # 引数
///
/// * `tile_x` - タイル座標のY座標
///
/// # 戻り値
///
/// 経度の範囲
pub fn tile_longitude_range(tile_x: u32, zoom: u8) -> Range {
    let n = (1 << zoom) as f64;
    let unit = 360.0 / n;
    let min = -180.0 + tile_x as f64 * unit;
    let max = min + unit;

    Range { min, max }
}

/// タイル座標からバウンダリーボックスを計算する。
///
/// # 引数
///
/// * `tc` - タイル座標
///
/// # 戻り値
///
/// バウンダリーボックス
pub fn tile_bbox(tc: TileCoordinate) -> BBox {
    let Range {
        min: x_min,
        max: x_max,
    } = tile_longitude_range(tc.x, tc.z);
    let Range {
        min: y_min,
        max: y_max,
    } = tile_latitude_range(tc.y, tc.z);

    BBox {
        x_min,
        y_min,
        x_max,
        y_max,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 富士山三角点
    const FUJI_LON: f64 = 138.0 + 43.0 / 60.0 + 39.0 / 3600.0;
    const FUJI_LAT: f64 = 35.0 + 21.0 / 60.0 + 39.0 / 3600.0;

    #[test]
    fn degree_to_tile_ok() {
        let coordinate = Coordinate {
            lon: FUJI_LON,
            lat: FUJI_LAT,
        };
        let expected_list = vec![
            (1, (1, 0)),
            (2, (3, 1)),
            (3, (7, 3)),
            (4, (14, 6)),
            (5, (28, 12)),
            (6, (56, 25)),
            (7, (113, 50)),
            (8, (226, 101)),
            (9, (453, 202)),
            (10, (906, 404)),
            (11, (1813, 808)),
            (12, (3626, 1617)),
            (13, (7252, 3234)),
            (14, (14505, 6469)),
            (15, (29011, 12939)),
            (16, (58022, 25878)),
            (17, (116045, 51756)),
            (18, (232090, 103513)),
        ];
        for (zoom, (expected_tile_x, expected_tile_y)) in expected_list {
            let actual = degree_to_tile(coordinate, zoom);
            assert_eq!(expected_tile_x, actual.x, "{}", zoom);
            assert_eq!(expected_tile_y, actual.y, "zoom level: {}", zoom);
        }
    }

    #[test]
    fn tile_to_degree_ok() {
        let expected = (138.724365234375, 35.362176059146805);
        let actual = tile_to_degree(TileCoordinate {
            z: 15,
            x: 29011,
            y: 12939,
        });
        assert!(
            (expected.0 - actual.lon).abs() < 1e-6,
            "expect: {}, actual: {}",
            expected.0,
            actual.lon
        );
        assert!(
            (expected.1 - actual.lat).abs() < 1e-6,
            "expect: {}, actual: {}",
            expected.1,
            actual.lat
        );
    }

    #[test]
    fn tile_latitude_range_ok() {
        let expected = (35.353216101238225, 35.362176059146805);
        let actual = tile_latitude_range(12939, 15);
        assert!((expected.0 - actual.min).abs() < 1e-6);
        assert!((expected.1 - actual.max).abs() < 1e-6);
    }

    #[test]
    fn tile_longitude_range_ok() {
        let expected = (138.724365234375, 138.7353515625);
        let actual = tile_longitude_range(29011, 15);
        assert!((expected.0 - actual.min).abs() < 1e-6);
        assert!((expected.1 - actual.max).abs() < 1e-6);
    }

    #[test]
    fn tile_bbox_ok() {
        let expected = (
            138.724365234375,
            35.353216101238225,
            138.7353515625,
            35.362176059146805,
        );
        let actual = tile_bbox(TileCoordinate {
            z: 15,
            x: 29011,
            y: 12939,
        });
        assert!((expected.0 - actual.x_min).abs() < 1e-6);
        assert!((expected.1 - actual.y_min).abs() < 1e-6);
        assert!((expected.2 - actual.x_max).abs() < 1e-6);
        assert!((expected.3 - actual.y_max).abs() < 1e-6);
    }
}
