use crate::*;
use crate::{
    blake2b::Blake2bHasher, default_store::DefaultStore, error::Error, merge::MergeValue,
    MerkleProof, SparseMerkleTree,
};
use proptest::prelude::*;
use rand::prelude::{Rng, SliceRandom};

type SMT = SparseMerkleTree<Blake2bHasher, H256, DefaultStore<H256>>;

fn blake2b_256(data: &[u8]) -> [u8; 32] {
    let mut result = [0u8; 32];
    let mut blake2b = blake2b_rs::Blake2bBuilder::new(32).personal(b"SMT").build();
    blake2b.update(data);
    blake2b.finalize(&mut result);
    result
}


#[test]
fn gen_big_tree() {
    let mut tree = SMT::default();
    // 100 -> 302
    // 10000 -> 500, 7.46s
    // 1000000 ->
    // 100000000 ->
    for i in 1..100u64 {
        let key: H256 = blake2b_256(&i.to_le_bytes()).into();
        let value = blake2b_256(&i.to_be_bytes()).into();
        tree.update(key, value).expect("update");
    }
    println!("SMT root 1: {:?}\n", tree.root());

    let new_key: H256 = blake2b_256(&1000u64.to_le_bytes()).into();
    let proof = tree.merkle_proof(vec![new_key.clone()]).unwrap();
    println!("root 1 = {:?}", tree.root());
    println!("proof 1 = {:?}", proof.merkle_path());
    println!("leaf 1 = {:?}", tree.get(&new_key));
    // let compiled_proof = proof.compile(vec![(new_key.clone().into(), H256::zero())]).unwrap();
    // file.write_all(format!("New merkle proof 1: {:?}\n", compiled_proof).as_bytes());

    println!("");

    let new_value = blake2b_256(&1000u64.to_be_bytes()).into();
    tree.update(new_key.clone(), new_value).expect("update");
    let proof = tree.merkle_proof(vec![new_key.clone()]).unwrap();
    println!("root 2 = {:?}", tree.root());
    println!("proof 2 = {:?}", proof.merkle_path());
    println!("leaf 2 = {:?}", tree.get(&new_key));

    println!("verify 2 = {:?}", proof.verify(tree.root(), vec![(new_key.clone(), new_value)]));
    // let compiled_proof = proof.compile(vec![leaf.clone()]).unwrap();
    // file.write_all(format!("New merkle proof 2: {:?}\n", compiled_proof).as_bytes());
}
