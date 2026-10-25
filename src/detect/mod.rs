//! ハッシュのレア判定。外部 I/O を持たない。

mod bits;
mod detectors;
mod hex;
mod model;
mod registry;
mod scorer;

const MIN_RUN_LEN: usize = 4;

pub use model::{Hit, ScoredHash};
pub use registry::{list_ids, select};
pub use scorer::score_hash;
