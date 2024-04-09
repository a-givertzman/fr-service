use std::{collections::HashMap, hash::BuildHasherDefault};
use hashers::fx_hash::FxHasher;
///
/// HashMap from std::collections with simple & fast hasher
///  - This hashing algorithm should not be used for cryptographic, or in scenarios where DOS attacks are a concern.
pub type HashMapFxHasher<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
