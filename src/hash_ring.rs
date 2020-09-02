use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hasher;

use twox_hash::XxHash64;

/// HashRing
#[derive(Debug, Clone)]
pub struct HashRing<T> {
    replicas: isize,
    ring: HashMap<String, T>,
    sorted_keys: Vec<String>,
}

impl<T: ToString + Clone> HashRing<T> {
    /// Creates a new hash ring with the specified nodes.
    /// Replicas is the number of virtual nodes each node has to make a better distribution.
    pub fn new(nodes: Vec<T>, replicas: isize) -> HashRing<T> {
        let mut new_hash_ring: HashRing<T> =
            HashRing { replicas, ring: HashMap::new(), sorted_keys: Vec::new() };

        for i in nodes {
            let n = &i;
            new_hash_ring.add_node(n);
        }
        new_hash_ring
    }

    /// Adds a node to the hash ring
    pub fn add_node(&mut self, node: &T) {
        for i in 0..self.replicas {
            let key = self.gen_key(format!("{}:{}", node.to_string(), i));
            self.ring.insert(key.clone(), (*node).clone());
            self.sorted_keys.push(key.clone());
        }

        self.sorted_keys = BinaryHeap::from(self.sorted_keys.clone()).into_sorted_vec();
    }

    /// Deletes a node from the hash ring
    pub fn remove_node(&mut self, node: &T) {
        for i in 0..self.replicas {
            let key = self.gen_key(format!("{}:{}", node.to_string(), i));
            if !self.ring.contains_key(&key) {
                break;
            }

            self.ring.remove(&key);
            let mut index = 0;
            for j in 0..self.sorted_keys.len() {
                if self.sorted_keys[j] == key {
                    index = j;
                    break;
                }
            }

            if !self.sorted_keys.is_empty() {
                self.sorted_keys.remove(index);
            }
        }
    }

    /// Gets the node a specific key belongs to
    pub fn get_node(&self, key: String) -> Option<&T> {
        if self.sorted_keys.is_empty() {
            return None;
        }

        let generated_key = self.gen_key(key);
        let nodes = self.sorted_keys.clone();

        for i in nodes.clone() {
            let node = &i;
            if generated_key <= *node {
                return Some(self.ring.get(node).unwrap());
            }
        }

        let node = &nodes[0];
        Some(self.ring.get(node).unwrap())
    }

    /// Generates a key from a string value
    fn gen_key(&self, key: String) -> String {
        let digest = hash(key.as_bytes());
        format!("{:x}", digest)
    }
}

fn hash(t: &[u8]) -> u64 {
    let mut hasher = XxHash64::default();
    hasher.write(t);
    hasher.finish()
}

#[cfg(test)]
mod test {
    use crate::HashRing;

    #[derive(Debug, PartialEq, Clone)]
    struct CustomNodeInfo {
        pub host: &'static str,
        pub port: u16,
    }

    impl ToString for CustomNodeInfo {
        fn to_string(&self) -> String {
            format!("{}:{}", self.host, self.port)
        }
    }

    #[test]
    fn test_empty_ring() {
        let hash_ring: HashRing<CustomNodeInfo> = HashRing::new(vec![], 10);
        assert_eq!(None, hash_ring.get_node("hello".to_string()));
    }

    #[test]
    fn test_custom_nodes() {
        let mut nodes: Vec<CustomNodeInfo> = Vec::new();
        nodes.push(CustomNodeInfo { host: "localhost", port: 15324 });
        nodes.push(CustomNodeInfo { host: "localhost", port: 15325 });
        nodes.push(CustomNodeInfo { host: "localhost", port: 15326 });
        nodes.push(CustomNodeInfo { host: "localhost", port: 15327 });
        nodes.push(CustomNodeInfo { host: "localhost", port: 15328 });
        nodes.push(CustomNodeInfo { host: "localhost", port: 15329 });

        let mut hash_ring: HashRing<CustomNodeInfo> = HashRing::new(nodes, 10);

        assert_eq!(
            Some("localhost:15326".to_string()),
            hash_ring.get_node("hello".to_string()).map(|x| x.to_string(),)
        );
        assert_eq!(
            Some("localhost:15327".to_string()),
            hash_ring.get_node("dude".to_string()).map(|x| x.to_string(),)
        );

        hash_ring.remove_node(&CustomNodeInfo { host: "localhost", port: 15329 });
        assert_eq!(
            Some("localhost:15326".to_string()),
            hash_ring.get_node("hello".to_string()).map(|x| x.to_string(),)
        );

        hash_ring.add_node(&CustomNodeInfo { host: "localhost", port: 15329 });
        assert_eq!(
            Some("localhost:15326".to_string()),
            hash_ring.get_node("hello".to_string()).map(|x| x.to_string(),)
        );
    }
}
