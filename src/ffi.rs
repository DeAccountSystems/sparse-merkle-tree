use std::any::Any;
use std::ffi::c_void;
use crate::{blake2b::Blake2bHasher, default_store::DefaultStore, error::Error, H256, merge::MergeValue, MerkleProof, SparseMerkleTree};

type SMT = SparseMerkleTree<Blake2bHasher, H256, DefaultStore<H256>>;

#[no_mangle]
extern "C" fn init_smt_tree() -> *mut SMT {
    let mut tree = SMT::default();
    println!("root = {:?}", tree.root());

    let pointer = Box::into_raw(Box::new(tree));
    println!("pointer of tree = {:?}", pointer);

    pointer
}

#[no_mangle]
extern "C" fn update_smt_tree(tree_ptr: *mut SMT, key_ptr: *const [u8; 32], value_ptr: *const [u8; 32]) -> i32 {
    println!("pointer of tree = {:?}", tree_ptr);

    let tree: &mut SMT = unsafe {
        &mut *(Box::from_raw(tree_ptr) as Box<SMT>)
    };

    let key = unsafe {
        H256::from(key_ptr.read())
    };
    let value = unsafe {
        H256::from(value_ptr.read())
    };
    println!("key = {:?}", key);
    println!("value = {:?}", value);

    println!("prev root = {:?}", tree.root().to_owned());
    match tree.update(key, value) {
        Ok(root) => {
            println!("current root = {:?}", root);
            0
        },
        Err(e) => {
            println!("update failed: {:?}", e);
            1
        }
    }
}
