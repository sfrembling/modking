use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub fn progress_bar(msg: &str) -> ProgressBar {
    // taken from https://github.com/console-rs/indicatif/blob/main/examples/long-spinner.rs
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    pb.set_message(msg.to_owned());
    pb
}

pub fn progress_bar_with_length(msg: &str, len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len).with_message(msg.to_owned());
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    pb
}
