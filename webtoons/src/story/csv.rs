pub mod models;

use crate::utils::CsvWrite;
use anyhow::Context;
use anyhow::Result;
use models::StoryRecord;
use serde::Serialize;
use std::path::Path;
use tracing::{debug, info};

impl<T: Serialize> CsvWrite for Vec<StoryRecord<T>> {
    fn write(self, path: &Path, filename: &str) -> Result<()> {
        info!("Writing to csv");
        let csv_name = format!("{filename}.csv");
        let mut writer = csv::Writer::from_path(path.join(csv_name))
            .context("File is open in another application")?;

        for data in self {
            debug!("Writing row");
            writer.serialize(data).context("Couldn't write to file.")?;
        }

        writer.flush().context("Couldn't flush buffer.")?;

        info!("Flushed buffer");

        Ok(())
    }
}
