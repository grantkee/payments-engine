use structopt::StructOpt;
use std::path;
use serde::Deserialize;

#[derive(StructOpt, Debug)]
struct Opt {
    path: path::PathBuf,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "type")]
    transaction: String,
    client: u16,
    tx: u32,
    amount: f64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let file = Opt::from_args();
    println!("file: {:?}", file.path);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file.path)
        .expect("error reading from path");
    
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
