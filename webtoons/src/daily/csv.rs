use super::models::Daily;
use crate::utils::{self, CsvWrite};
use anyhow::{Context, Result};

impl CsvWrite for Vec<Daily> {
    fn write(self, path: &str, filename: &str) -> Result<()> {
        let path_buf = utils::validate_path(path)?.join(filename);

        let mut writer = csv::Writer::from_path(path_buf)?;

        for data in self {
            writer.serialize(data).context("Couldn't write to file.")?;
        }

        writer.flush().context("Couldn't flush buffer.")?;

        Ok(())
    }
}
