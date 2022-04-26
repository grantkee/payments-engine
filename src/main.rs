use structopt::StructOpt;
use std::{process, path};
use payments_engine;

#[derive(StructOpt, Debug)]
struct Opt {
    path: path::PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let file = Opt::from_args();
    println!("file: {:?}", file.path);

    if let Err(e) = payments_engine::run(file.path).await {
        println!("Application error: {}", e);
        process::exit(1);
    }

    Ok(())
}
