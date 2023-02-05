use anyhow::{Context, Result};
use chrono::Utc;
use std::fs;
use std::path::Path;

#[macro_export]
macro_rules! regex {
    ($regex:expr) => {
        ::regex::Regex::new($regex).unwrap()
    };
}
#[must_use]
pub fn get_current_utc_date_verbose() -> String {
    Utc::now().to_string()
}

pub trait CsvWrite {
    /// # Errors
    fn write(self, path: &str, filename: &str) -> Result<()>;
}

///
///
/// # Arguments
///
/// * `filepath`:
///
/// returns: Result<&Path, Error>
///
/// # Examples
///
/// ```
///
/// ```
///
/// # Errors
pub fn validate_path(filepath: &str) -> Result<&Path> {
    let path = Path::new(filepath);

    if !path.try_exists().context("Check if given path exists")? {
        fs::create_dir(path).context("Create given path so it's existence is enforced")?;
    }

    Ok(path)
}

#[cfg(test)]
mod utils_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[ignore]
    fn should_create_valid_folder() {
        let given_path = r"D:\temp\temp";

        let result = validate_path(given_path).unwrap().to_str().unwrap();

        assert_eq!(result, given_path);
    }
}
