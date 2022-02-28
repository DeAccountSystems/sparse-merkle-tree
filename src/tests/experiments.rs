use crate::*;
use crate::{
    blake2b::Blake2bHasher, default_store::DefaultStore, SparseMerkleTree, merge::{MergeValue, merge}
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

    let key_1: H256 = H256::from(hex_to_hash("0000000000000000000000000000000000000000000000000000000000000000"));
    let value_1: H256 = H256::from(hex_to_hash("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key_1, value_1);
    let key_2: H256 = H256::from(hex_to_hash("0100000000000000000000000000000000000000000000000000000000000000"));
    let value_2: H256 = H256::from(hex_to_hash("11ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key_2, value_2);
    let key_3: H256 = H256::from(hex_to_hash("0300000000000000000000000000000000000000000000000000000000000000"));
    let value_3: H256 = H256::from(hex_to_hash("33ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    tree.update(key_3, value_3);

    let key: H256 = H256::from(hex_to_hash("0000000000000000000000000000000000000000000000000000000000000000"));
    let proof = tree.merkle_proof(vec![key]).unwrap();

    proof.compile(vec![(key_3, value_3)]).unwrap();

    // let key_4: H256 = H256::from(hex_to_hash("0400000000000000000000000000000000000000000000000000000000000000"));
    // proof.compile(vec![(key_4, H256::zero())]).unwrap();

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
fn print_merge() {
    let height = 0;
    let node_key = H256::from(hex_to_hash("0000000000000000000000000000000000000000000000000000000000000001"));
    let lhs = MergeValue::Value(H256::from(hex_to_hash("0000000000000000000000000000000000000000000000000000000000000002")));
    let rhs = MergeValue::Value(H256::from(hex_to_hash("0000000000000000000000000000000000000000000000000000000000000003")));

    let ret = merge::<Blake2bHasher>(height, &node_key, &lhs, &rhs);
    println!("return: {}", ret);
}

#[test]
fn gen_big_tree() {
    let mut tree = SMT::default();
    // 100 -> 302
    // 10000 -> 500, 7.46s
    // 100000 ->
    // 100000000 ->
    for i in 1..100000u64 {
        let key: H256 = blake2b_256(&i.to_le_bytes()).into();
        let value = blake2b_256(&i.to_be_bytes()).into();
        tree.update(key, value).expect("update");
    }
    println!("SMT root 1: {:?}\n", tree.root());
}
