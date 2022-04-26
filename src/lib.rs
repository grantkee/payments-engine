pub use error::Error;
pub use transaction::TransactionInfo;
pub use std::path;

mod error;
mod transaction;

pub async fn run(path: path::PathBuf) -> Result<(), Error>{
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    for result in rdr.deserialize() {
        let record: TransactionInfo = result?;
        println!("{:?}", record);
    }
    Ok(())
}
