pub use error::Error;
pub use transaction::TransactionInfo;
pub use std::path;

mod error;
mod transaction;

pub async fn run(path: path::PathBuf) -> Result<(), Error>{
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_path(path)?;

    for result in rdr.deserialize() {
        let record: TransactionInfo = result?;
        println!("{:?}", record);
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_handle_whitespace() {
        let result = run(path::PathBuf::from("./tests/CSVs/transactions.csv"));
        assert!(result.await.is_ok())
    }

    #[tokio::test]
    async fn correct_path_must_exist() {
        let result = run(path::PathBuf::from("./tests/NONEXISTENT_PATH.csv"));
        assert!(result.await.is_err())
    }
}
