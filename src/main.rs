//! Flexible service manager with dependency support. **CURRENTLY IN DEVELOPMENT**
//!
//! # What?
//! serviced is a simple, yet extensible service manager. This crate currently serves the purpose
//! of reserving the name `serviced` in crates.io, and contains no other code than the standard
//! "Hello, world!".
#![crate_name = "serviced"]
#![cfg_attr(test, deny(warnings))]
#![deny(unused)]
#![deny(unstable_features)]
#![warn(missing_docs)]
#![allow(rustdoc::private_intra_doc_links)]

/// Entry-point for serviced.
fn main() {
    let bar = indicatif::ProgressBar::new(100)
        .with_style(
            indicatif::ProgressStyle::default_spinner()
                .tick_strings(&[
                    " .  ",
                    ".   ",
                    " .  ",
                    "  . ",
                    "   .",
                    "  . ",
                    &format!("{}", console::style(" OK ").green().bright()),
                ])
                .template("[{spinner}] {wide_msg} ( {elapsed_precise} / unlimited )"),
        )
        .with_message("Initializing serviced...");
    for _ in 0..50 {
        std::thread::sleep(std::time::Duration::from_millis(200));
        bar.tick()
    }
    bar.finish_with_message("Initialized serviced");
    drop(bar);
    std::thread::sleep(std::time::Duration::from_millis(400));

    let bar = indicatif::ProgressBar::new(100)
        .with_style(
            indicatif::ProgressStyle::default_spinner()
                .tick_strings(&[
                    " .  ",
                    ".   ",
                    " .  ",
                    "  . ",
                    "   .",
                    "  . ",
                    &format!("{}", console::style("FAIL").red().bright()),
                ])
                .template("[{spinner}] {wide_msg} ( {elapsed_precise} / unlimited )"),
        )
        .with_message("Starting serviced...");
    for _ in 0..50 {
        std::thread::sleep(std::time::Duration::from_millis(200));
        bar.tick()
    }
    bar.finish_with_message("Unable to start serviced: unimplemented");
}
