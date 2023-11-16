/*
Last updated: 11-15-2023

Description: This crate defines the data structures used in the codebase

Author: James Dean
*/
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    id: String,
    blockchain_id: String,
    timestamp: String,
    owner: String,
    data: serde_json::Value,
    signature: String
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    id: String
    blockchain_id: String,
    timestamp: i64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    height: String,
    nonce: String,
    node: String,
    signature: String
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    id: String,
    blockchain_id: String,
    address: String,
    status: String,
    block_count: String,
    transaction_count: String
}