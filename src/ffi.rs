use std::mem;
use crate::{blake2b::Blake2bHasher, default_store::DefaultStore, H256, SparseMerkleTree};

type SMT = SparseMerkleTree<Blake2bHasher, H256, DefaultStore<H256>>;

#[no_mangle]
extern "C" fn init_smt_tree() -> *mut SMT {
    let tree = SMT::default();
    println!("root = {:?}", tree.root());

    let pointer = Box::into_raw(Box::new(tree));
    println!("pointer of tree = {:?}", pointer);

    mem::forget(pointer);
    pointer
}

#[no_mangle]
extern "C" fn update_smt_tree(tree_ptr: *mut SMT, key_ptr: *const u8, value_ptr: *const u8) -> i32 {
    println!("pointer of tree = {:?}", tree_ptr);

    let tree: &mut SMT = unsafe {
        &mut *(Box::from_raw(tree_ptr) as Box<SMT>)
    };

    let key = unsafe {
        let mut tmp = [0u8; 32];
        key_ptr.copy_to(tmp.as_mut_ptr(), 32);
        H256::from(tmp)
    };
    let value = unsafe {
        let mut tmp = [0u8; 32];
        value_ptr.copy_to(tmp.as_mut_ptr(), 32);
        H256::from(tmp)
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

#[no_mangle]
extern "C" fn get_smt_root(tree_ptr: *mut SMT) -> *const u8 {
    println!("pointer of tree = {:?}", tree_ptr);

    let tree: &mut SMT = unsafe {
        &mut *(Box::from_raw(tree_ptr) as Box<SMT>)
    };

    let root = tree.root();
    println!("root = {:?}", root);

    root.as_slice().as_ptr()
}

#[no_mangle]
extern "C" fn get_smt_merkle_proof(tree_ptr: *mut SMT, key_ptr: *const [u8; 32], value_ptr: *const [u8; 32]) -> *const u8 {
    println!("pointer of tree = {:?}", tree_ptr);

    let tree: &mut SMT = unsafe {
        &mut *(Box::from_raw(tree_ptr) as Box<SMT>)
    };

    let key = unsafe {
        H256::from(key_ptr.read().clone())
    };
    let value = unsafe {
        H256::from(value_ptr.read().clone())
    };
    println!("key = {:?}", key);
    println!("value = {:?}", value);

    match tree.merkle_proof(vec![key.clone()]) {
        Ok(proof) => {
            match proof.compile(vec![(key, value)]) {
                Ok(compiled_proof) => {
                    let bin: Vec<u8> = compiled_proof.into();
                    bin.as_ptr()
                },
                Err(e) => {
                    println!("get proof failed: {:?}", e);
                    vec![].as_ptr()
                }
            }
        },
        Err(e) => {
            println!("get proof failed: {:?}", e);
            vec![].as_ptr()
        }
    }
}
