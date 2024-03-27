#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental


use risc0_zkvm::guest::env;

use nmt_rs::row_inclusion::Proof as RowInclusionProof;
use nmt_rs::{simple_merkle::proof, NamespaceId, NamespacedHash};

use celestia_types::{nmt::Namespace, Blob};
//use risc0_zkvm::serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use std::fmt;

risc0_zkvm::guest::entry!(main);


fn main() {
    // TODO: Implement your guest code here

    // read the input
    //let input: u32 = env::read();

    const NUM_LEAVES: u32 = 272;
    let mut data_root = [0u8; 32 as usize];
    let result: bool = env::read();
    env::read_slice(&mut data_root);

    // read num rows
    let num_rows: u32 = env::read();
    // read namespace ID
    let namespace = env::read::<Namespace>();

    let mut leaves = [[0u8; 512]; NUM_LEAVES as usize];
    for i in 0..NUM_LEAVES {
        env::read_slice(&mut leaves[i as usize]);
    }

    let mut start = 0;
    for i in 0..(num_rows as usize) {
        let root = env::read::<NamespacedHash<29>>();
        let row_inclusion_proof = env::read::<RowInclusionProof>();
        if !row_inclusion_proof.verify(data_root) {
            println!("NOT GOOD");
            env::commit(&false);
            return;
        }
        let proof: celestia_types::nmt::NamespaceProof = env::read();
        let end = start + (proof.end_idx() as usize - proof.start_idx() as usize);
        let result = proof.verify_range(&root, &leaves[start..end], namespace.into_inner());
        start = end;
        if result.is_err() {
            println!("NOT GOOD");
            env::commit(&false);
            return;
        }
    }

    // TODO: do something with the input

    // write public output to the journal
    println!("we good");
    env::commit(&true);
}
