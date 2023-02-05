pub mod models;

use crate::utils::{self, CsvWrite};
use anyhow::Context;
use anyhow::Result;
use models::StoryRecord;
use serde::Serialize;
use tracing::{debug, info};

impl<T: Serialize> CsvWrite for Vec<StoryRecord<T>> {
    fn write(self, path: &str, filename: &str) -> Result<()> {
        info!("Writing to csv");
        let csv_name = format!("{filename}.csv");

        let path_buff = utils::validate_path(path)?.join(csv_name);

        let mut writer =
            csv::Writer::from_path(path_buff).context("File is open in another application")?;

        for (row, data) in self.into_iter().enumerate() {
            debug!("Writing row {row}");
            writer.serialize(data).context("Couldn't write to file.")?;
        }

        writer.flush().context("Couldn't flush buffer.")?;

        info!("Flushed buffer");

        Ok(())
    }
}
