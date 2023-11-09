-- 都道府県テーブル
CREATE TABLE prefectures (
    -- 都道府県コード
    code CHAR(2) NOT NULL,
    -- 都道府県名
    name VARCHAR(10) NOT NULL,
    -- 通常の都道府県コード
    jis_code CHAR(2) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);
CREATE INDEX idx_prefectures_jis_code ON prefectures (jis_code);

-- 警察署テーブル
CREATE TABLE police_stations (
    -- 都道府県コード
    prefecture_code CHAR(2) NOT NULL,
    -- 警察署コード
    police_station_code CHAR(3) NOT NULL,
    -- 警察署名
    police_station_name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (prefecture_code, police_station_code),
    -- 外部参照制約 都道府県テーブル
    FOREIGN KEY (prefecture_code) REFERENCES prefectures(code)
);

-- 事故内容テーブル
CREATE TABLE accident_details (
    -- 事故内容コード
    code CHAR(1) NOT NULL,
    -- 事故内容名
    name VARCHAR(10) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 路線テーブル
CREATE TABLE routes (
    -- 路線コードの範囲
    lower_code CHAR(4) NOT NULL,
    upper_code CHAR(4) NOT NULL,
    -- 路線名
    name VARCHAR(40) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (lower_code, upper_code)
);

-- 路線区分テーブル
CREATE TABLE route_classes (
    -- 路線区分コード
    code CHAR(1) NOT NULL,
    -- 路線区分名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 市区町村テーブル
CREATE TABLE cities (
    -- 市区町村コード
    city_jis_code CHAR(5) NOT NULL,
    -- 都道府県コード
    prefecture_jis_code CHAR(2) NOT NULL,
    -- 市区町村名
    city_name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (city_jis_code)
);

-- 昼夜テーブル
CREATE TABLE day_nights (
    -- 昼夜コード
    code CHAR(2) NOT NULL,
    -- 昼夜名
    name VARCHAR(10) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 天候テーブル
CREATE TABLE weathers (
    -- 天候コード
    code CHAR(1) NOT NULL,
    -- 天候名
    name VARCHAR(10) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 地区（地形）テーブル
CREATE TABLE districts (
    -- 地区コード
    code CHAR(1) NOT NULL,
    -- 地区名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 路面状態テーブル
CREATE TABLE surface_conditions (
    -- 路面状態コード
    code CHAR(1) NOT NULL,
    -- 路面状態名
    name VARCHAR(10) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 道路形状テーブル
CREATE TABLE road_models (
    -- 道路形状コード
    code CHAR(2) NOT NULL,
    -- 道路形状名
    name VARCHAR(30) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 信号機テーブル
CREATE TABLE traffic_signals (
    -- 信号機コード
    code CHAR(1) NOT NULL,
    -- 信号機名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 一時停止規制標識テーブル
CREATE TABLE stop_regulation_signs (
    -- 一時停止規制標識コード
    code CHAR(2) NOT NULL,
    -- 一時停止規制標識名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 一時停止規制表示テーブル
CREATE TABLE stop_regulation_displays (
    -- 一時停止規制表示コード
    code CHAR(2) NOT NULL,
    -- 一時停止規制表示名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 車道幅員テーブル
CREATE TABLE road_widths (
    -- 車道幅員コード
    code CHAR(2) NOT NULL,
    -- 車道幅員名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 道路線形テーブル
CREATE TABLE road_alignments (
    -- 道路線形コード
    code CHAR(1) NOT NULL,
    -- 道路線形名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 衝突地点テーブル
CREATE TABLE collision_points (
    -- 衝突地点コード
    code CHAR(2) NOT NULL,
    -- 衝突地点名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- ゾーン規制テーブル
CREATE TABLE zone_regulations (
    -- ゾーン規制コード
    code CHAR(2) NOT NULL,
    -- ゾーン規制名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 中央分離帯施設テーブル
CREATE TABLE central_separations (
    -- 中央分離帯施設コード
    code CHAR(1) NOT NULL,
    -- 中央分離帯施設名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 歩車道区分テーブル
CREATE TABLE road_segmentations (
    -- 歩車道区分コード
    code CHAR(1) NOT NULL,
    -- 歩車道区分名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 事故類型テーブル
CREATE TABLE accident_types (
    -- 事故類型コード
    code CHAR(2) NOT NULL,
    -- 事故類型名
    name VARCHAR(10) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 年齢テーブル
CREATE TABLE ages (
    -- 年齢コード
    code CHAR(2) NOT NULL,
    -- 年齢名
    name VARCHAR(10) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 当事者種別テーブル
CREATE TABLE parties (
    -- 当事者種別コード
    code CHAR(2) NOT NULL,
    -- 当事者種別名
    name VARCHAR(30) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 用途テーブル
CREATE TABLE purposes (
    -- 用途コード
    code CHAR(2) NOT NULL,
    -- 用途名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 車両形状テーブル
CREATE TABLE vehicle_types (
    -- 車両形状コード
    code CHAR(2) NOT NULL,
    -- 車両形状名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- オートマチック車テーブル
CREATE TABLE automatics (
    -- オートマチック車コード
    code CHAR(1) NOT NULL,
    -- オートマチック車名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- サポカーテーブル
CREATE TABLE support_cars (
    -- サポカーコード
    code CHAR(2) NOT NULL,
    -- サポカー名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 速度規制（指定のみ）テーブル
CREATE TABLE speed_regulations (
    -- 速度規制（指定のみ）コード
    code CHAR(2) NOT NULL,
    -- 速度規制（指定のみ）名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 車両の損壊程度テーブル
CREATE TABLE vehicle_damages (
    -- 車両の損壊程度コード
    code CHAR(1) NOT NULL,
    -- 車両の損壊程度名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- エアバッグの装備テーブル
CREATE TABLE airbags (
    -- エアバッグの装備コード
    code CHAR(1) NOT NULL,
    -- エアバッグの装備名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- サイドエアバッグの装備テーブル
CREATE TABLE side_airbags (
    -- サイドエアバッグの装備コード
    code CHAR(1) NOT NULL,
    -- サイドエアバッグの装備名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 人身損傷程度テーブル
CREATE TABLE injuries (
    -- 人身損傷程度コード
    code CHAR(1) NOT NULL,
    -- 人身損傷程度名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 曜日テーブル
CREATE TABLE weeks (
    -- 曜日コード
    code CHAR(1) NOT NULL,
    -- 曜日名
    name VARCHAR(10) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 祝日テーブル
CREATE TABLE holidays (
    -- 祝日コード
    code CHAR(1) NOT NULL,
    -- 祝日名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 運転練習の方法テーブル
CREATE TABLE driving_practices (
    -- 運転練習の方法コード
    code CHAR(1) NOT NULL,
    -- 運転練習の方法名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 乗車別テーブル
CREATE TABLE riding_types (
    -- 乗車別コード
    code CHAR(1) NOT NULL,
    -- 乗車別名
    name VARCHAR(10) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 乗車等の区分テーブル
CREATE TABLE riding_classes (
    -- 乗車等の区分コード
    code CHAR(2) NOT NULL,
    -- 乗車等の区分名
    name VARCHAR(20) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (code)
);

-- 交通事故（本票）テーブル
CREATE TABLE accidents (
    -- 交通事故ID
    id UUID NOT NULL,
    -- 都道府県コード
    prefecture_code CHAR(2) NOT NULL,
    -- 警察署コード
    police_station_code CHAR(3) NOT NULL,
    -- 本票番号
    main_number INTEGER NOT NULL,
    -- 事故内容コード
    accident_detail_code CHAR(1) NOT NULL,
    -- 死者数
    number_of_deaths INTEGER NOT NULL,
    -- 負傷者数
    number_of_injuries INTEGER NOT NULL,
    -- 路線コード
    route_code CHAR(4) NOT NULL,
    -- 路線区分コード
    route_class_code CHAR(1) NOT NULL,
    -- 地点コード
    location_code INTEGER NOT NULL,
    -- 市区町村コード
    city_jis_code CHAR(5) NOT NULL,
    -- 発生日時
    occurred_at TIMESTAMP WITH TIME ZONE NOT NULL,
    -- 昼夜コード
    day_night_code CHAR(2) NOT NULL,
    -- 日の出時刻
    sunrise_time TIME NOT NULL,
    -- 日の入時刻
    sunset_time TIME NOT NULL,
    -- 天候コード
    weather_code CHAR(1) NOT NULL,
    -- 地形（地区）コード
    district_code CHAR(1) NOT NULL,
    -- 路面状態コード
    surface_condition_code CHAR(1) NOT NULL,
    -- 道路形状コード
    road_model_code CHAR(2) NOT NULL,
    -- 信号機コード
    traffic_signal_code CHAR(1) NOT NULL,
    -- 一時停止規制標識コード（当事者A）
    stop_regulation_sign_a_code CHAR(2) NOT NULL,
    -- 一時停止規制表示コード（当事者A）
    stop_regulation_display_a_code CHAR(2) NOT NULL,
    -- 一時停止規制標識コード（当事者B）
    stop_regulation_sign_b_code CHAR(2) NOT NULL,
    -- 一時停止規制表示コード（当事者B）
    stop_regulation_display_b_code CHAR(2) NOT NULL,
    -- 車道幅員コード
    road_width_code CHAR(2) NOT NULL,
    -- 道路線形コード
    road_alignment_code CHAR(1) NOT NULL,
    -- 衝突地点コード
    collision_point_code CHAR(2) NOT NULL,
    -- ゾーン規制コード
    zone_regulation_code CHAR(2) NOT NULL,
    -- 中央分離帯施設コード
    central_separation_code CHAR(1) NOT NULL,
    -- 歩車道区分コード
    road_segmentation_code CHAR(1) NOT NULL,
    -- 事故類型コード
    accident_type_code CHAR(2) NOT NULL,
    -- 年齢コード（当事者A）
    age_a_code CHAR(2) NOT NULL,
    -- 年齢コード（当事者B）
    age_b_code CHAR(2) NOT NULL,
    -- 当事者種別コード（当事者A）
    party_a_code CHAR(2) NOT NULL,
    -- 当事者種別コード（当事者B）
    party_b_code CHAR(2) NOT NULL,
    -- 用途コード（当事者A）
    purpose_a_code CHAR(2) NOT NULL,
    -- 用途コード（当事者B）
    purpose_b_code CHAR(2) NOT NULL,
    -- 車両種別コード（当事者A)
    vehicle_type_a_code CHAR(2) NOT NULL,
    -- 車両種別コード（当事者B)
    vehicle_type_b_code CHAR(2) NOT NULL,
    -- オートマチック車コード（当事者A）
    automatic_a_code CHAR(2) NOT NULL,
    -- オートマチック車コード（当事者B）
    automatic_b_code CHAR(2) NOT NULL,
    -- サポカーコード（当事者A）
    support_car_a_code CHAR(2) NOT NULL,
    -- サポカーコード（当事者B）
    support_car_b_code CHAR(2) NOT NULL,
    -- 速度規制（指定のみ）コード（当事者A）
    speed_regulation_a_code CHAR(2) NOT NULL,
    -- 速度規制（指定のみ）コード（当事者B）
    speed_regulation_b_code CHAR(2) NOT NULL,
    -- 車両の衝突部位（当事者A）
    collision_part_a CHAR(2) NOT NULL,
    -- 車両の衝突部位（当事者b）
    collision_part_b CHAR(2) NOT NULL,
    -- 車両の損壊程度コード（当事者A）
    vehicle_damage_a_code CHAR(1) NOT NULL,
    -- 車両の損壊程度コード（当事者B）
    vehicle_damage_b_code CHAR(1) NOT NULL,
    -- エアバッグの装備コード（当事者A）
    airbag_a_code CHAR(1) NOT NULL,
    -- エアバッグの装備コード（当事者B）
    airbag_b_code CHAR(1) NOT NULL,
    -- サイドエアバッグの装備コード（当事者A）
    side_airbag_a_code CHAR(1) NOT NULL,
    -- サイドエアバッグの装備コード（当事者B）
    side_airbag_b_code CHAR(1) NOT NULL,
    -- 人身損傷程度コード（当事者A）
    injury_a_code CHAR(1) NOT NULL,
    -- 人身損傷程度コード（当事者B）
    injury_b_code CHAR(1) NOT NULL,
    -- 地点（JGD2011）
    location GEOMETRY(POINT, 6668) NOT NULL,
    -- 曜日コード
    week_code CHAR(1) NOT NULL,
    -- 祝日コード
    holiday_code CHAR(1) NOT NULL,
    -- 認知機能検査経過日数コード（当事者A）
    cognitive_days_a INTEGER NOT NULL,
    -- 認知機能検査経過日数コード（当事者B）
    cognitive_days_b INTEGER NOT NULL,
    -- 運転練習の方法コード（当事者A）
    driving_practice_a_code CHAR(1) NOT NULL,
    -- 運転練習の方法コード（当事者B）
    driving_practice_b_code CHAR(1) NOT NULL,
    -- 主キー制約
    PRIMARY KEY (id),
    -- ユニークキー制約
    UNIQUE (prefecture_code, police_station_code, main_number),
    -- 外部参照制約 都道府県
    FOREIGN KEY (prefecture_code) REFERENCES prefectures(code),
    -- 外部参照制約 警察署
    FOREIGN KEY (prefecture_code, police_station_code) REFERENCES police_stations(prefecture_code, police_station_code),
    -- 外部参照制約 事故内容
    FOREIGN KEY (accident_detail_code) REFERENCES accident_details(code),
    -- 外部参照制約 路線区分
    FOREIGN KEY (route_class_code) REFERENCES route_classes(code),
    -- 外部参照制約 市区町村
    FOREIGN KEY (city_jis_code) REFERENCES cities(city_jis_code),
    -- 外部参照制約 昼夜
    FOREIGN KEY (day_night_code) REFERENCES day_nights(code),
    -- 外部参照制約 天候
    FOREIGN KEY (weather_code) REFERENCES weathers(code),
    -- 外部参照制約 地区
    FOREIGN KEY (district_code) REFERENCES districts(code),
    -- 外部参照制約 路面状態
    FOREIGN KEY (surface_condition_code) REFERENCES surface_conditions(code),
    -- 外部参照制約 道路形状
    FOREIGN KEY (road_model_code) REFERENCES road_models(code),
    -- 外部参照制約 信号機
    FOREIGN KEY (traffic_signal_code) REFERENCES traffic_signals(code),
    -- 外部参照制約 一時停止規制標識（当事者A）
    FOREIGN KEY (stop_regulation_sign_a_code) REFERENCES stop_regulation_signs(code),
    -- 外部参照制約 一時停止規制表示（当事者A）
    FOREIGN KEY (stop_regulation_display_a_code) REFERENCES stop_regulation_displays(code),
    -- 外部参照制約 一時停止規制標識（当事者B）
    FOREIGN KEY (stop_regulation_sign_b_code) REFERENCES stop_regulation_signs(code),
    -- 外部参照制約 一時停止規制表示（当事者B）
    FOREIGN KEY (stop_regulation_display_b_code) REFERENCES stop_regulation_displays(code),
    -- 外部参照制約 車道幅員
    FOREIGN KEY (road_width_code) REFERENCES road_widths(code),
    -- 外部参照制約 道路線形
    FOREIGN KEY (road_alignment_code) REFERENCES road_alignments(code),
    -- 外部参照制約 衝突地点
    FOREIGN KEY (collision_point_code) REFERENCES collision_points(code),
    -- 外部参照制約 ゾーン規制
    FOREIGN KEY (zone_regulation_code) REFERENCES zone_regulations(code),
    -- 外部参照制約 中央分離帯施設
    FOREIGN KEY (central_separation_code) REFERENCES central_separations(code),
    -- 外部参照制約 歩車道区分
    FOREIGN KEY (road_segmentation_code) REFERENCES road_segmentations(code),
    -- 外部参照制約 事故類型
    FOREIGN KEY (accident_type_code) REFERENCES accident_types(code),
    -- 外部参照制約 年齢（当事者A）
    FOREIGN KEY (age_a_code) REFERENCES ages(code),
    -- 外部参照制約 年齢（当事者B）
    FOREIGN KEY (age_b_code) REFERENCES ages(code),
    -- 外部参照制約 当事者種別（当事者A）
    FOREIGN KEY (party_a_code) REFERENCES parties(code),
    -- 外部参照制約 当事者種別（当事者B）
    FOREIGN KEY (party_b_code) REFERENCES parties(code),
    -- 外部参照制約 用途（当事者A）
    FOREIGN KEY (purpose_a_code) REFERENCES purposes(code),
    -- 外部参照制約 用途（当事者B）
    FOREIGN KEY (purpose_b_code) REFERENCES purposes(code),
    -- 外部参照制約 車両形状（当事者A）
    FOREIGN KEY (vehicle_type_a_code) REFERENCES vehicle_types(code),
    -- 外部参照制約 車両形状（当事者B）
    FOREIGN KEY (vehicle_type_b_code) REFERENCES vehicle_types(code),
    -- 外部参照制約 オートマチック車（当事者A）
    FOREIGN KEY (automatic_a_code) REFERENCES automatics(code),
    -- 外部参照制約 オートマチック車（当事者B）
    FOREIGN KEY (automatic_b_code) REFERENCES automatics(code),
    -- 外部参照制約 サポカー（当事者A）
    FOREIGN KEY (support_car_a_code) REFERENCES support_cars(code),
    -- 外部参照制約 サポカー（当事者B）
    FOREIGN KEY (support_car_b_code) REFERENCES support_cars(code),
    -- 外部参照制約 速度規制（指定のみ）（当事者A）
    FOREIGN KEY (speed_regulation_a_code) REFERENCES speed_regulations(code),
    -- 外部参照制約 速度規制（指定のみ）（当事者B）
    FOREIGN KEY (speed_regulation_b_code) REFERENCES speed_regulations(code),
    -- 外部参照制約 車両の損壊程度（当事者A）
    FOREIGN KEY (vehicle_damage_a_code) REFERENCES vehicle_damages(code),
    -- 外部参照制約 車両の損壊程度（当事者B）
    FOREIGN KEY (vehicle_damage_b_code) REFERENCES vehicle_damages(code),
    -- 外部参照制約 エアバッグの装備（当事者A）
    FOREIGN KEY (airbag_a_code) REFERENCES airbags(code),
    -- 外部参照制約 エアバッグの装備（当事者B）
    FOREIGN KEY (airbag_b_code) REFERENCES airbags(code),
    -- 外部参照制約 サイドエアバッグの装備（当事者A）
    FOREIGN KEY (side_airbag_a_code) REFERENCES side_airbags(code),
    -- 外部参照制約 サイドエアバッグの装備（当事者B）
    FOREIGN KEY (side_airbag_b_code) REFERENCES side_airbags(code),
    -- 外部参照制約 人身損傷程度（当事者A）
    FOREIGN KEY (injury_a_code) REFERENCES injuries(code),
    -- 外部参照制約 人身損傷程度（当事者B）
    FOREIGN KEY (injury_b_code) REFERENCES injuries(code),
    -- 外部参照制約 曜日
    FOREIGN KEY (week_code) REFERENCES weeks(code),
    -- 外部参照制約 祝日
    FOREIGN KEY (holiday_code) REFERENCES holidays(code),
    -- 外部参照制約 運転練習の方法（当事者A）
    FOREIGN KEY (driving_practice_a_code) REFERENCES driving_practices(code),
    -- 外部参照制約 運転練習の方法（当事者B）
    FOREIGN KEY (driving_practice_b_code) REFERENCES driving_practices(code)
);
-- 交通事故テーブルに空間インデックスを作成
CREATE INDEX idx_accidents_location ON accidents USING GIST (location);

-- 交通事故当事者以外関与者テーブル
CREATE TABLE involved_persons (
    -- 交通事故当事者以外関与者ID
    id UUID NOT NULL,
    -- 交通事故ID
    accident_id UUID NOT NULL,
    -- 補充票番号
    sub_number INTEGER NOT NULL,
    -- 当事者種別コード
    party_code CHAR(2) NOT NULL,
    -- 用途別コード
    purpose_code CHAR(2),
    -- 車両形状コード
    vehicle_type_code CHAR(2),
    -- 乗車別コード
    riding_type_code CHAR(1) NOT NULL,
    -- 乗車等区分コード
    riding_class_code CHAR(2) NOT NULL,
    -- サポカーコード
    support_car_code CHAR(2) NOT NULL,
    -- エアバッグの装備コード
    airbag_code CHAR(1) NOT NULL,
    -- サイドエアバッグの装備コード
    side_airbag_code CHAR(1) NOT NULL,
    -- 人身損傷程度コード
    injury_code CHAR(1) NOT NULL,
    -- 車両の衝突部位
    collision_part CHAR(2),
    -- 車両の損傷程度コード
    vehicle_damage_code CHAR(1),
    -- 主キー制約
    PRIMARY KEY (id),
    -- 外部参照制約 交通事故
    FOREIGN KEY (accident_id) REFERENCES accidents(id),
    -- 外部参照制約 当事者種別
    FOREIGN KEY (party_code) REFERENCES parties(code),
    -- 外部参照制約 用途
    FOREIGN KEY (purpose_code) REFERENCES purposes(code),
    -- 外部参照制約 車両形状
    FOREIGN KEY (vehicle_type_code) REFERENCES vehicle_types(code),
    -- 外部参照制約 乗車別
    FOREIGN KEY (riding_type_code) REFERENCES riding_types(code),
    -- 外部参照制約 乗車等区分
    FOREIGN KEY (riding_class_code) REFERENCES riding_classes(code),
    -- 外部参照制約 サポカー
    FOREIGN KEY (support_car_code) REFERENCES support_cars(code),
    -- 外部参照制約 エアバッグの装備
    FOREIGN KEY (airbag_code) REFERENCES airbags(code),
    -- 外部参照制約 サイドエアバッグの装備
    FOREIGN KEY (side_airbag_code) REFERENCES side_airbags(code),
    -- 外部参照制約 人身損傷程度
    FOREIGN KEY (injury_code) REFERENCES injuries(code),
    -- 外部参照制約 車両の損傷程度
    FOREIGN KEY (vehicle_damage_code) REFERENCES vehicle_damages(code)
);
