use super::models::Daily;
use crate::utils::CsvWrite;
use anyhow::{Context, Result};
use std::path::Path;

impl CsvWrite for Vec<Daily> {
    fn write(self, path: &Path, filename: &str) -> Result<()> {
        let mut writer = csv::Writer::from_path(path.join(filename)).unwrap();

        for data in self {
            writer.serialize(data).context("Couldn't write to file.")?;
        }

        writer.flush().context("Couldn't flush buffer.")?;

        Ok(())
    }
}
