use indicatif::{ProgressBar, ProgressStyle};

pub struct ProgressBarFactory {}

impl ProgressBarFactory {
    pub fn new(count: u16) -> ProgressBar {
        let bar = ProgressBar::new(count as u64);

        bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar:.white/black} {pos}/{len}").unwrap());

        bar
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
