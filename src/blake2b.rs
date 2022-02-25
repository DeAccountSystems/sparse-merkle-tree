use crate::{traits::Hasher, H256};
use blake2b_rs::{Blake2b, Blake2bBuilder};

const BLAKE2B_KEY: &[u8] = &[];
const BLAKE2B_LEN: usize = 32;
const PERSONALIZATION: &[u8] = b"sparsemerkletree";

pub struct Blake2bHasher(Blake2b);

impl Default for Blake2bHasher {
    fn default() -> Self {
        let blake2b = Blake2bBuilder::new(BLAKE2B_LEN)
            .personal(PERSONALIZATION)
            .key(BLAKE2B_KEY)
            .build();
        println!("blake2b init: {{ length: {} personal: {}, key: {:?} }}", BLAKE2B_LEN, String::from_utf8(PERSONALIZATION.to_vec()).unwrap(), BLAKE2B_KEY);

        Blake2bHasher(blake2b)
    }
}

impl Hasher for Blake2bHasher {
    fn write_h256(&mut self, h: &H256) {
        println!("blake2b write: {:?}", h);
        self.0.update(h.as_slice());
    }
    fn write_byte(&mut self, b: u8) {
        println!("blake2b write: {:?}", &[b][..]);
        self.0.update(&[b][..]);
    }
    fn finish(self) -> H256 {
        let mut hash = [0u8; 32];
        self.0.finalize(&mut hash);
        hash.into()
    }
}
