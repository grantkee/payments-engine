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
    println!("file: {:?}", file.path);

    let mut engine = Engine::default();
    println!("engine defaults transactions: {:?}", engine.transactions);
    println!("engine defaults clients: {:?}", engine.clients);

    if let Err(e) = engine.stream(file.path).await {
        println!("Application error while reading CSV: {}", e);
        process::exit(1);
    }

    Ok(())
}
