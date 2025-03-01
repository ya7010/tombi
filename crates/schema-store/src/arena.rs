use std::{num::NonZeroUsize, sync::Arc};

use ahash::AHashMap;

#[derive(Debug, Clone)]
pub struct Arena<K, V> {
    data: Vec<V>,
    key_map: Arc<tokio::sync::RwLock<AHashMap<K, NonZeroUsize>>>,
}

impl<K, V> Arena<K, V>
where
    K: std::hash::Hash + Eq,
{
    pub fn new() -> Self {
        Arena {
            data: Vec::new(),
            key_map: Arc::new(tokio::sync::RwLock::new(AHashMap::new())),
        }
    }

    pub async fn insert(&self, key: K, d: V) -> &V {
        let mut key_map = self.key_map.write().await;

        unsafe {
            let mutable_self = self as *const Self as *mut Self;
            (*mutable_self).data.push(d);
        }

        key_map.insert(key, NonZeroUsize::new(self.data.len()).unwrap());

        &self.data[self.data.len() - 1]
    }

    pub async fn get(&self, key: &K) -> Option<&V> {
        self.key_map
            .read()
            .await
            .get(key)
            .map(|index| &self.data[index.get() - 1])
    }

    pub async fn contains_key(&self, key: &K) -> bool {
        self.key_map.read().await.contains_key(key)
    }

    pub async fn update(&self, key: &K, d: V) -> &V {
        let key_map = self.key_map.write().await;
        if let Some(&index) = key_map.get(key) {
            unsafe {
                let mutable_self = self as *const Self as *mut Self;
                (*mutable_self).data[index.get() - 1] = d;
            }

            &self.data[index.get() - 1]
        } else {
            unreachable!("key not found");
        }
    }
}

impl<K, V> Default for Arena<K, V> {
    fn default() -> Self {
        Self {
            data: Vec::default(),
            key_map: Arc::new(tokio::sync::RwLock::default()),
        }
    }
}
