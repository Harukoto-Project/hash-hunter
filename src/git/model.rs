/// git から取得したコミット。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    pub hash: String,
}
