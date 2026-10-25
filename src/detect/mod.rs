//! ハッシュのレア判定。外部 I/O を持たない。

mod bits;
mod detectors;
mod hex;
mod model;
mod registry;
mod scorer;

#[allow(unused_imports)]
pub use model::{Detector, Hit, ScoredHash};
#[allow(unused_imports)]
pub use registry::select;
#[allow(unused_imports)]
pub use scorer::score_hash;
