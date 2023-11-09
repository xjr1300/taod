use std::collections::HashMap;
use std::path::Path;

use crate::db::{prefecture_hash_map, register_accidents, register_involved_persons};
use crate::files::{read_accidents, read_involved_persons};
use db::connection_pool;

/// データベースに交通事故を登録する。
///
/// # 引数
///
/// * `main_path` - 本票ファイルパス
/// * `support_path` - 補充票ファイルパス
///
/// # 戻り値
///
/// `()`
pub async fn insert<P: AsRef<Path>>(main_file: P, _support_file: P) -> anyhow::Result<()> {
    // 都道府県コードとJIS規格の都道府県コードの対応を記録したハッシュマップ
    let pool = connection_pool().await?;
    let prefectures = prefecture_hash_map(&pool).await?;

    // 交通事故（本票）を読み込み
    let accidents = read_accidents(main_file, &prefectures)?;
    // 交通事故識別子と交通事故IDの対応を記録したハッシュマップを生成
    let mut accident_ids = HashMap::new();
    for accident in &accidents {
        accident_ids.insert(accident.identifier(), accident.id);
    }
    // 補充表を読み込み
    let involved_persons = read_involved_persons(_support_file, &accident_ids)?;

    // トランザクションを開始
    let mut tx = pool.begin().await.map_err(|_| {
        anyhow::anyhow!(
            "交通事故をデータベースに登録する際に、トランザクションを開始できませんでした。"
        )
    })?;

    // 交通事故をデータベースに登録
    register_accidents(&mut tx, &accidents).await?;
    // 交通事故当事者以外の関係者をデータベースに登録
    register_involved_persons(&mut tx, &involved_persons).await?;

    // トランザクションをコミット
    tx.commit().await.map_err(|_| {
        anyhow::anyhow!("交通事故をデータベースに登録する際に、コミットできませんでした。")
    })?;

    Ok(())
}
