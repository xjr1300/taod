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
    pub traffic_accident_zoom_level: u8,
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
