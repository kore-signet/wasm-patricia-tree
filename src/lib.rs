mod utils;

use bincode::Options;
use patricia_tree::StringPatriciaSet;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Trie {
    inner: StringPatriciaSet
}

impl Serialize for Trie {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        self.inner.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Trie {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        StringPatriciaSet::deserialize(deserializer).map(|inner| Trie { inner })
    }
}

#[wasm_bindgen]
impl Trie {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Trie {
        Trie {
            inner: StringPatriciaSet::new()
        }
    }

    pub fn add(&mut self, s: &str) {
        self.inner.insert(s);
    }

    pub fn search(&self, prefix: &str, limit: usize) -> Vec<String> {
        self.inner.iter_prefix(prefix).take(limit).map(String::from).collect()
    }

    pub fn to_bytes(&self) -> Box<[u8]> {
        bincode::DefaultOptions::new().serialize(self).unwrap().into_boxed_slice()
    }

    pub fn from_bytes(data: &[u8]) -> Result<Trie, JsValue> {
        bincode::DefaultOptions::new().deserialize(data).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}