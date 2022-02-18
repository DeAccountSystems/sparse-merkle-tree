use crate::*;
use crate::{
    blake2b::Blake2bHasher, default_store::DefaultStore, SparseMerkleTree,
};

type SMT = SparseMerkleTree<Blake2bHasher, H256, DefaultStore<H256>>;

fn blake2b_256(data: &[u8]) -> [u8; 32] {
    let mut result = [0u8; 32];
    let mut blake2b = blake2b_rs::Blake2bBuilder::new(32).personal(b"SMT").build();
    blake2b.update(data);
    blake2b.finalize(&mut result);
    result
}

fn hex_to_hash(input: &str) -> [u8; 32] {
    let mut ret = [0u8; 32];
    let data = hex::decode(input).expect("Invalid hex");
    ret.copy_from_slice(&data);

    ret
}

#[test]
fn sort_leaves() {
    let mut leaves = vec![
        H256::from(hex_to_hash("0000000000000000000000000000000000000000000000000000000000000001")),
        H256::from(hex_to_hash("0000000000000000000000000000000000000000000000000000000000000002")),
        H256::from(hex_to_hash("0000000000000000000000000000000000000000000000000000000000000003")),
        // ...
        H256::from(hex_to_hash("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd")),
        H256::from(hex_to_hash("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe")),
        H256::from(hex_to_hash("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")),
    ];

    leaves.sort_by_key(|a| a.clone());

    for leaf in leaves {
        println!("{}", leaf);
    }
}

#[test]
fn insert_one_leaf() {
    let mut tree = SMT::default();
    tree.root();

    let key: H256 = H256::from(hex_to_hash("0000000000000000000000000000000000000000000000000000000000000000"));
    let value: H256 = H256::from(hex_to_hash("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key, value);

    tree.root();

    // let proof = tree.merkle_proof(vec![new_key.clone()]).unwrap();
}

#[test]
fn insert_two_leaf() {
    let mut tree = SMT::default();
    tree.root();

    let key: H256 = H256::from(hex_to_hash("0000000000000000000000000000000000000000000000000000000000000000"));
    let value: H256 = H256::from(hex_to_hash("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key, value);

    let key: H256 = H256::from(hex_to_hash("0100000000000000000000000000000000000000000000000000000000000000"));
    let value: H256 = H256::from(hex_to_hash("11ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key, value);

    tree.root();

    // let proof = tree.merkle_proof(vec![new_key.clone()]).unwrap();
}

#[test]
fn insert_four_leaf() {
    let mut tree = SMT::default();
    tree.root();

    let key: H256 = H256::from(hex_to_hash("0000000000000000000000000000000000000000000000000000000000000000"));
    let value: H256 = H256::from(hex_to_hash("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key, value);

    let key: H256 = H256::from(hex_to_hash("0100000000000000000000000000000000000000000000000000000000000000"));
    let value: H256 = H256::from(hex_to_hash("11ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key, value);

    let key: H256 = H256::from(hex_to_hash("0200000000000000000000000000000000000000000000000000000000000000"));
    let value: H256 = H256::from(hex_to_hash("22ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key, value);

    let key: H256 = H256::from(hex_to_hash("0300000000000000000000000000000000000000000000000000000000000000"));
    let value: H256 = H256::from(hex_to_hash("33ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key, value);

    tree.root();

    // let proof = tree.merkle_proof(vec![new_key.clone()]).unwrap();
}

#[test]
fn merkle_proof_00() {
    let mut tree = SMT::default();

    let key: H256 = H256::from(hex_to_hash("0000000000000000000000000000000000000000000000000000000000000000"));
    let value: H256 = H256::from(hex_to_hash("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key, value);
    let key: H256 = H256::from(hex_to_hash("0100000000000000000000000000000000000000000000000000000000000000"));
    let value: H256 = H256::from(hex_to_hash("11ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key, value);
    let key: H256 = H256::from(hex_to_hash("0300000000000000000000000000000000000000000000000000000000000000"));
    let value: H256 = H256::from(hex_to_hash("33ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key, value);

    let key: H256 = H256::from(hex_to_hash("0000000000000000000000000000000000000000000000000000000000000000"));
    tree.merkle_proof(vec![key]);

    // let proof = tree.merkle_proof(vec![new_key.clone()]).unwrap();
}

#[test]
fn merkle_proof_03() {
    let mut tree = SMT::default();

    let key: H256 = H256::from(hex_to_hash("0000000000000000000000000000000000000000000000000000000000000000"));
    let value: H256 = H256::from(hex_to_hash("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key, value);
    let key: H256 = H256::from(hex_to_hash("0100000000000000000000000000000000000000000000000000000000000000"));
    let value: H256 = H256::from(hex_to_hash("11ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key, value);
    let key: H256 = H256::from(hex_to_hash("0300000000000000000000000000000000000000000000000000000000000000"));
    let value: H256 = H256::from(hex_to_hash("33ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key, value);

    let key: H256 = H256::from(hex_to_hash("0000000000000000000000000000000000000000000000000000000000000000"));
    tree.merkle_proof(vec![key]);

    // let proof = tree.merkle_proof(vec![new_key.clone()]).unwrap();
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
    println!("proof_bin 1 = {:?}", proof.compile(vec![(new_key.clone(), H256::zero())]));
    println!("leaf 1 = {:?}", tree.get(&new_key));
    // let compiled_proof = proof.compile(vec![(new_key.clone().into(), H256::zero())]).unwrap();
    // file.write_all(format!("New merkle proof 1: {:?}\n", compiled_proof).as_bytes());

    println!("");

    let new_value = blake2b_256(&1000u64.to_be_bytes()).into();
    tree.update(new_key.clone(), new_value).expect("update");
    let proof = tree.merkle_proof(vec![new_key.clone()]).unwrap();
    println!("root 2 = {:?}", tree.root());
    println!("proof 2 = {:?}", proof.merkle_path());
    println!("proof_bin 2 = {:?}", proof.compile(vec![(new_key.clone(), new_value.clone())]));
    println!("leaf 2 = {:?}", tree.get(&new_key));

    // println!("verify 2 = {:?}", proof.verify(tree.root(), vec![(new_key.clone(), new_value)]));
    // let compiled_proof = proof.compile(vec![leaf.clone()]).unwrap();
    // file.write_all(format!("New merkle proof 2: {:?}\n", compiled_proof).as_bytes());
}
