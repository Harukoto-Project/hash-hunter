use clap::Parser;

/// `hash-hunter` コマンドの引数。
#[derive(Debug, Parser)]
#[command(
    name = "hash-hunter",
    version,
    about = "Hunt rare commit hashes in a Git repository"
)]
pub struct Args {
    /// Git の URL またはローカルパス
    #[arg(required_unless_present = "list_detectors")]
    pub repo: Option<String>,

    /// 表示する上位件数
    #[arg(short = 'n', long = "top", default_value_t = 20)]
    pub top: usize,

    /// このスコア(bits)未満は表示しない
    #[arg(long = "min-score", default_value_t = 0.0)]
    pub min_score: f64,

    /// 対象ブランチ(未指定は全ブランチ)
    #[arg(long)]
    pub branch: Option<String>,

    /// 有効にする検出器 ID を限定
    #[arg(long, value_delimiter = ',')]
    pub only: Vec<String>,

    /// 無効にする検出器 ID
    #[arg(long, value_delimiter = ',')]
    pub exclude: Vec<String>,

    /// 検出器の一覧を表示して終了
    #[arg(long = "list-detectors")]
    pub list_detectors: bool,
}
