use clap::{Parser, Subcommand};

use taod_cli::insert;

/// コマンドライン引数
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// #[clap(flatten)]
    /// global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Commands,
}

/// /// グローバルオプション
/// #[derive(Debug, clap::Args)]
/// struct GlobalOpts {
///     /// Color
///     #[clap(long, arg_enum, global = true, default_value_t = Color::Auto)]
///     color: Color,
///
///     /// Verbosity level (can be specified multiple times)
///     #[clap(long, short, global = true, parse(from_occurrences))]
///     verbose: usize,
///     //... other global options
/// }

#[derive(Debug, Subcommand)]
enum Commands {
    /// データベースに交通事故データを登録
    ///
    /// cargo run -- insert <file>
    Insert {
        /// 本票ファイル(cp932エンコーディング)
        main_file: String,
        /// 補充票ファイル（cp932エンコーディング）
        support_file: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Insert {
            main_file,
            support_file,
        } => {
            insert::insert(main_file, support_file).await?;
        }
    }

    Ok(())
}
