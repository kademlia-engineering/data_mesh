/*
Last updated: 11-16-2023

Description: This crate exposes cryptography utilities

Author: James Dean
*/
use sha3::{Digest, Keccak512, Keccak256};

// Keccack 512
// Params: Vec<u8>
// Returns: [u8;64]
pub fn keccak_512(bytes: Vec<u8>) -> [u8;64] {
    let mut hasher = Keccak512::new();
    hasher.update(bytes);
    let slice = hasher.finalize();
    return slice.into();
}

// Keccack 256
// Params: Vec<u8>
// Returns: [u8;64]
pub fn keccak_256(bytes: Vec<u8>) -> [u8;32] {
    let mut hasher = Keccak256::new();
    hasher.update(bytes);
    let slice = hasher.finalize();
    return slice.into();
}