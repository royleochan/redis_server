use bytes::Bytes;

use std::collections::HashMap;

pub struct Store {
    key_val_store: HashMap<Bytes, Bytes>,
}

impl Store {
    pub fn init() -> Self {
        let key_val_store = HashMap::new();
        return Store {
            key_val_store: key_val_store,
        };
    }

    pub fn set_key_val(&mut self, key: Bytes, val: Bytes) {
        self.key_val_store.insert(key, val);
    }

    pub fn get_from_key_val_store(&self, key: Bytes) -> Bytes {
        self.key_val_store.get(&key).unwrap().clone()
    }
}
