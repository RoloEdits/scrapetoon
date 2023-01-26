use anyhow::{Context, Result};
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
#[macro_export]
macro_rules! regex {
    ($regex:expr) => {
        ::regex::Regex::new($regex).unwrap()
    };
}

pub fn get_current_utc_date() -> String {
    Utc::now().date_naive().to_string()
}

pub fn create_date_folder(filepath: &str) -> Result<PathBuf> {
    let path = Path::new(filepath);

    let date_now = get_current_utc_date();
    let date_path = path.join(date_now);

    if !date_path
        .try_exists()
        .context("Failed to check is date folder already exists")?
    {
        fs::create_dir(&date_path).context("Create date folder")?;
    }

    Ok(date_path)
}

pub fn path_enforcer(filepath: &str) -> Result<&Path> {
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
    fn should_create_date_folder() {
        let given_path = r"D:\temp";

        let date = get_current_utc_date();

        let result = create_date_folder(given_path)
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();

        let test = format!("{given_path}\\{date}");

        assert_eq!(result, test);
    }

    #[test]
    #[ignore]
    fn should_create_valid_folder() {
        let given_path = r"D:\temp\temp";

        let result = path_enforcer(given_path).unwrap().to_str().unwrap();

        assert_eq!(result, given_path);
    }
}
