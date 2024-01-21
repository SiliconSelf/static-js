#![doc = include_str!("../../README.md")]

use std::{time::{SystemTime, Duration}, io::Write, ffi::OsStr};

use log::{error, info, warn};
use sailfish::TemplateOnce;

use crate::templates::index::IndexTemplate;

mod templates;

#[tokio::main]
async fn main() {
    simple_logger::init().expect("Failed to initialize logging");
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    for entry in walkdir::WalkDir::new("pages") {
        let Ok(entry) = entry else { continue; };
        let path = entry.path().to_owned();
        if !path.is_file() { continue; }
        if path.extension() != Some(OsStr::new("toml")) { continue; };
        let tx = tx.clone();
        tokio::task::spawn_blocking(move || {
            let path_string = path.to_string_lossy();
            // Read TOML file from disk
            let Ok(data) = std::fs::read_to_string(&path) else { error!("Failed to read file for {path_string}!"); return; };
            let Ok(ctx) = toml::from_str::<IndexTemplate>(&data) else { error!("Failed to deserialize {path_string}!"); return; };
            // Render appropriate template
            let start_time = SystemTime::now();
            let Ok(mut render) = ctx.render_once() else { error!("Failed to render template for {path_string}!"); return; };
            if let Ok(duration) = start_time.elapsed() {
                let micros = duration.as_micros();
                render = render.replace("{GENERATION_TIME}", &format!("{micros}"));
                info!("Rendered {} in {} microseconds", entry.path().to_string_lossy(), micros);
            } else {
                warn!("Template for {path_string} was successfully rendered, but duration failed to calculate!");
            }
            // Return render to main thread
            tx.send((path, render)).expect("Failed to send");
        });
    }
    drop(tx);
    loop {
        match rx.try_recv() {
            Ok((mut path, render)) => {
                path.set_extension("html");
                let mut output = std::fs::File::create(path).expect("Failed to open file");
                output.write_all(render.as_bytes()).expect("Failed to write file");
            },
            Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
                std::thread::sleep(Duration::from_millis(250));
            }
            Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => break,
        }
    }
}
