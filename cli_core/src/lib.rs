use indicatif::{ProgressBar, ProgressStyle};

pub struct ProgressBarFactory {}

impl ProgressBarFactory {
    pub fn get_bar(count: u16) -> ProgressBar {
        let bar = ProgressBar::new(count as u64);

        bar.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] [{wide_bar:.white/black} {pos}/{len}",
            )
            .unwrap(),
        );

        bar
    }
}
