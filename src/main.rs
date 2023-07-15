#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use sg_audio::*;
use sg_gui::*;
// mod audio;

use log::{debug, error, info, log, trace, warn, LevelFilter};
use std::io::Write;

fn main() {
    env_logger::Builder::from_default_env()
        .format(
            |buf: &mut env_logger::fmt::Formatter, record: &log::Record| {
                writeln!(buf, "{} - {}", record.level(), record.args())
            },
        )
        .filter(None, LevelFilter::Info)
        .init();
    trace!("Logger initialized.");

    // TODO: Need better threading so GUI and Audio don't block each other.
    // gui.run() must be kept on main thread to avoid platform-specific issues.
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let gui = sg_gui::GUI::new().await;
            // play();
            gui.run();
        });

    trace!("Program ending");
}
