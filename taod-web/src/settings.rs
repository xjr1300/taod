/// 設定
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    /// Webアプリ設定
    pub web_app: WebAppSettings,
}

/// Webアプリ設定
#[derive(Debug, Clone, serde::Deserialize)]
pub struct WebAppSettings {
    /// 交通事故表示最小ズームレベル
    pub accident_zoom_level: u8,

    /// 交通事故取得バッファ率
    /// タイルに含まれる交通事故を取得する際に、当該タイルの緯度方向の距離に対して、
    /// この値を乗じた距離だけ拡大したタイルの範囲を取得する。
    ///
    /// 例えば、タイル座標11/1813/808のタイルの緯度方向の距離は、0.143304(=35.460670 - 35.317366)である。
    /// この値が、0.001の場合、緯度方向の距離に乗じた値は0.000143304となる。
    /// よって、当該タイルの範囲を上下左右に0.000143304だけ拡大したタイルの範囲に含まれる交通事故を取得する。
    /// 緯度1度は、約111,120mである。この場合、111,120 * 0.000143304 = 15.92mとなり、これだけ当該タイルを
    /// 上下左右に広げた範囲で交通事故を取得する。
    pub accident_buffer_ratio: f64,
}

/// 設定を読み込む。
///
/// `<project-dir>/settings/[base|*].yml`ファイルを読み込む。
///
/// # 戻り値
///
/// 設定
pub fn get_settings() -> Result<Settings, config::ConfigError> {
    // 設定ファイルディレクトリを取得
    let base_path = std::env::current_dir().expect("カレントディレクトリの取得に失敗しました。");
    let settings_dir = base_path.join("settings");
    // 設定を読み込み
    let settings = config::Config::builder()
        .add_source(config::File::from(settings_dir.join("base.yml")))
        .build()?;

    settings.try_deserialize::<Settings>()
}
