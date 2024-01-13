#![doc = include_str!("../README.md")]

use std::{time::{SystemTime, Duration}, io::Write, ffi::OsStr};

use log::{info};
use sailfish::TemplateOnce;
use serde::Deserialize;

#[derive(TemplateOnce, Deserialize)]
#[template(path = "index.stpl")]
struct IndexTemplate {
    meta: MetaTemplate,
    navbar: NavbarTemplate,
    claims: Vec<Claim>
}

#[derive(Deserialize)]
struct Claim {
    title: String,
    description: String
}

#[derive(Deserialize)]
struct Url {
    url: String,
    title: String
}

#[derive(Deserialize)]

struct MetaTemplate {
    title: String
}

#[derive(Deserialize)]
struct NavbarTemplate {
    links: Vec<Url>
}

#[derive(Deserialize)]
struct HeroTemplate {
    headline: String,
    subtitle: String,

}


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
            // Read TOML file from disk
            let data = std::fs::read_to_string(&path).expect("Failed to read file");
            let ctx: IndexTemplate = toml::from_str(&data).expect("Failed to deserialize");
            // Render appropriate template
            let start_time = SystemTime::now();
            let render = ctx.render_once().expect("Failed to render");
            let duration = start_time.elapsed().expect("Failed to time");
            let render = render.replace("{GENERATION_TIME}", &format!("{}", duration.as_nanos()));
            info!("Rendered {} in {} nanoseconds", entry.path().to_string_lossy(), duration.as_nanos());
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
