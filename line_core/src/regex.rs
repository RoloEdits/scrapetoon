
#[macro_export]
macro_rules! regex {
    ($regex:expr) => { ::regex::Regex::new($regex).unwrap()};
}