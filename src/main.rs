use payments_engine::Engine;
use std::{path, process};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    path: path::PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file = Opt::from_args();
    let mut engine = Engine::default();

    if let Err(e) = engine.stream(file.path).await {
        println!("Application error while reading CSV: {}", e);
        process::exit(1);
    }

    if let Err(e) = engine.write().await {
        println!("Application error while writing CSV: {}", e);
        process::exit(1);
    }

    Ok(())
}
