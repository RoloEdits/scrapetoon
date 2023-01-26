use anyhow::Result;
use std::path::Path;

pub mod daily;
pub mod story;

pub trait CsvWrite {
    fn write(self, path: &Path, filename: &str) -> Result<()>;
}
