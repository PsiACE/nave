#![cfg_attr(feature = "docinclude", feature(external_doc))]
#![cfg_attr(feature = "docinclude", doc(include = "../README.md"))]

extern crate twox_hash;

mod hash_ring;

pub use hash_ring::HashRing;
