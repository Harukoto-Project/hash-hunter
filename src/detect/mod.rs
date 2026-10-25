//! ハッシュのレア判定。外部 I/O を持たない。

mod detectors;
mod model;
mod registry;
mod scorer;

#[allow(unused_imports)]
pub use model::{Detector, Hit, ScoredHash};
