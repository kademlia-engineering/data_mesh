// Add the necessary dependencies in your Cargo.toml
// [dependencies]
// immudb = "0.1" // Use the latest version

use immudb::ImmudbClient;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::error::Error;

// Define your struct
#[derive(Serialize, Deserialize, Debug)]
struct Record {
    blockchain_id: u32,
    data: String,
    // other fields...
}

impl Record {
    // Store the record in immudb
    async fn store(&self, client: &ImmudbClient) -> Result<(), Box<dyn Error>> {
        let key = self.blockchain_id.to_be_bytes(); // Convert blockchain_id to bytes as key
        let value = serde_json::to_vec(self)?; // Serialize the record to JSON for storage
        client.set(&key, &value).await?;
        Ok(())
    }

    // Retrieve records by blockchain_id
    async fn read(client: &ImmudbClient, blockchain_id: u32) -> Result<Vec<Record>, Box<dyn Error>> {
        let key = blockchain_id.to_be_bytes();
        let value = client.get(&key).await?;
        let record: Record = serde_json::from_slice(&value)?;
        Ok(vec![record]) // Returns a Vec of Record, assuming each ID maps to a single record
    }
}