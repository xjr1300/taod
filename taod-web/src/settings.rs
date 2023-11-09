use serde_aux::field_attributes::deserialize_number_from_string;

/// 設定
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    /// Webアプリ設定
    pub web_app: WebAppSettings,
}

/// Webアプリ設定
#[derive(Debug, Clone, serde::Deserialize)]
pub struct WebAppSettings {
    /// Webアプリホスト
    pub host: String,

    /// Webアプリポート
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,

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

/// Webアプリ運用環境
pub enum AppEnvironment {
    // ローカル環境
    Local,
    // 運用環境
    Production,
}

impl AppEnvironment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Production => "production",
        }
    }
}

impl TryFrom<String> for AppEnvironment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            _ => Err(format!(
                "環境変数APP_ENVIRONMENTの値({})が不正です。`local`または`production`を指定してください。",
                value
            )),
        }
    }
}

/// 設定を読み込む。
///
/// `<project-dir>/settings/[base|*].yml`ファイルを読み込む。
///
/// # 戻り値
///
/// 設定
#[allow(clippy::redundant_closure)]
pub fn get_settings() -> Result<Settings, config::ConfigError> {
    // 動作環境を取得
    let app_environment: AppEnvironment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        // clippyは次を冗長なクロージャとして警告するが、`(e)`を削除するとコンパイルエラーが発生するため、
        // 当該警告を許すように関数を修飾
        .map_err(|e| config::ConfigError::Message(e))?;
    // 設定ファイルディレクトリを取得
    let base_path = std::env::current_dir().expect("カレントディレクトリの取得に失敗しました。");
    // 運用環境別設定ファイルを取得
    let environment_file = format!("{}.yml", app_environment.as_str());
    let settings_dir = base_path.join("settings");
    // 設定を読み込み
    let settings = config::Config::builder()
        .add_source(config::File::from(settings_dir.join("base.yml")))
        .add_source(config::File::from(settings_dir.join(environment_file)))
        .build()?;

    settings.try_deserialize::<Settings>()
}
