use std::collections::HashMap;
use std::sync::Mutex;

use bytes::Bytes;

type Shard = Mutex<HashMap<String, Bytes>>;

pub struct ShardedDB {
    shards: Vec<Shard>
}

impl ShardedDB {
    pub fn new(n_shards: usize) -> Self {
        let mut shards = Vec::with_capacity(n_shards);
        for _ in 0..n_shards {
            shards.push(Mutex::new(HashMap::new()));
        }
        Self { shards }
    }
    pub fn insert(&self, key: &str, value: &Bytes) {
        let hash = 0; // TO-DO: Add proper hashing
        let shard = hash % self.shards.len();
        self.shards[shard]
            .lock()
            .unwrap()
            .insert(key.to_string(), value.clone());
    }
    pub fn get(&self, key: &str) -> Option<bytes::Bytes> {
        let hash = 0; // TO-DO: Add proper hashing
        let shard = hash % self.shards.len();
        self.shards[shard].lock().unwrap().get(key).cloned()
    }
}
