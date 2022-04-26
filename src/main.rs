use structopt::StructOpt;
use std::path;

#[derive(StructOpt, Debug)]
struct Opt {
    path: path::PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let file = Opt::from_args();
    println!("file: {:?}", file.path);
    Ok(())
}
