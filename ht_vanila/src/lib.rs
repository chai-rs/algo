#![allow(dead_code)]

use std::collections::LinkedList;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
struct HashTable<K, V>
where
    K: Hash + PartialEq,
{
    buckets: Vec<LinkedList<(K, V)>>,
    size: usize,
    capacity: usize,
}

// Constructor
impl<K, V> HashTable<K, V>
where
    K: Hash + PartialEq,
{
    pub fn new(capacity: usize) -> Self {
        HashTable {
            buckets: Self::new_buckets(capacity),
            size: 0,
            capacity,
        }
    }

    fn new_buckets(capacity: usize) -> Vec<LinkedList<(K, V)>> {
        let mut buckets = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buckets.push(LinkedList::new());
        }
        buckets
    }
}

// Implementation
impl<K, V> HashTable<K, V>
where
    K: Hash + PartialEq,
{
    pub fn insert(&mut self, key: K, value: V) {
        self.resize();
        if Self::buckets_insert(&mut self.buckets, self.capacity, key, value) {
            self.size += 1;
        }
    }

    fn buckets_insert(
        buckets: &mut Vec<LinkedList<(K, V)>>,
        capacity: usize,
        key: K,
        value: V,
    ) -> bool {
        let index = Self::hash_index(&key, capacity);
        let el = buckets.get_mut(index).unwrap();

        for (k, v) in el.iter_mut() {
            if *k == key {
                *v = value;
                return false;
            }
        }

        el.push_back((key, value));
        true
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = Self::hash_index(&key, self.capacity);
        let el = self.buckets.get(index).unwrap();

        for (k, v) in el.iter() {
            if *k == *key {
                return Some(v);
            }
        }

        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = Self::hash_index(&key, self.capacity);
        let el = self.buckets.get_mut(index).unwrap();
        let mut extract = el
            .extract_if(|(k, _)| *k == *key)
            .collect::<LinkedList<_>>();

        extract.pop_front().map(|(_, v)| {
            self.size -= 1;
            v
        })
    }

    fn resize(&mut self) {
        let threshold = (self.capacity as f64) * 0.75;
        if threshold > self.size as f64 {
            return;
        }

        self.capacity *= 2;
        let mut new_buckets = Self::new_buckets(self.capacity);
        for el in self.buckets.drain(..) {
            for (k, v) in el.into_iter() {
                Self::buckets_insert(&mut new_buckets, self.capacity, k, v);
            }
        }
        self.buckets = new_buckets; // replace the resized
    }
}

// Hash Function
impl<K, V> HashTable<K, V>
where
    K: Hash + PartialEq,
{
    pub fn hash_index(key: &K, capacity: usize) -> usize {
        Self::hash(key) % capacity
    }

    pub fn hash(key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hashtable_hash() {
        let key = "key".to_string();
        assert!(
            HashTable::<String, String>::hash(&key) == HashTable::<String, String>::hash(&key),
            "hashed the same key must equal"
        );
    }

    #[test]
    fn test_hashtable_hash_index() {
        let key = "key".to_string();
        let cap = 10;
        assert!(
            HashTable::<String, String>::hash_index(&key, cap)
                == HashTable::<String, String>::hash_index(&key, cap),
            "hashed the same key must equal"
        );
    }

    #[test]
    fn test_hashtable_insert() {
        let mut ht: HashTable<String, String> = HashTable::new(10);

        ht.insert("key1".into(), "value1".into());
        ht.insert("key2".into(), "value2".into());
        ht.insert("key3".into(), "value3".into());

        assert!(ht.size == 3)
    }

    #[test]
    fn test_hashtable_get() {
        let mut ht = HashTable::new(10);
        ht.insert(String::from("name"), String::from("Alice"));

        let name_key = String::from("name");
        let value = ht.get(&name_key);
        println!("{:?}", value);
    }

    #[test]
    fn test_hashtable_remove() {
        let mut ht = HashTable::new(10);
        ht.insert("name", "Alice");
        ht.insert("age", "25");

        println!("Size before: {}", ht.size);

        let removed = ht.remove(&"name");
        println!("Removed: {:?}", removed);
        println!("Size after: {}", ht.size);

        let not_found = ht.remove(&"city");
        println!("Not found: {:?}", not_found);
    }

    #[test]
    fn test_resize() {
        let mut ht = HashTable::new(4); // capacity = 4, threshold = 3

        println!("Initial - capacity: {}, size: {}", ht.capacity, ht.size);

        ht.insert("a", 1);
        println!(
            "After insert 'a' - capacity: {}, size: {}",
            ht.capacity, ht.size
        );

        ht.insert("b", 2);
        println!(
            "After insert 'b' - capacity: {}, size: {}",
            ht.capacity, ht.size
        );

        ht.insert("c", 3);
        println!(
            "After insert 'c' - capacity: {}, size: {}",
            ht.capacity, ht.size
        );

        ht.insert("d", 4); // ← ตรงนี้ควร resize!
        println!(
            "After insert 'd' - capacity: {}, size: {}",
            ht.capacity, ht.size
        );

        // ตรวจสอบว่า data ยังอยู่ครบไหม
        assert_eq!(ht.get(&"a"), Some(&1));
        assert_eq!(ht.get(&"b"), Some(&2));
        assert_eq!(ht.get(&"c"), Some(&3));
        assert_eq!(ht.get(&"d"), Some(&4));
    }
}
