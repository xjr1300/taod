use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Postgres, Transaction};

pub type PgTransaction<'a> = Transaction<'a, Postgres>;

/// データベースコネクションプールを返す。
///
/// # 戻り値
///
/// データベースコネクションプール
pub async fn connection_pool() -> anyhow::Result<PgPool> {
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
