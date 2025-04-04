use crate::node::Node;
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

type HashValue = u64;

pub struct HashRing {
    ring: BTreeMap<HashValue, Node>,
    replicas: usize,
}

impl HashRing {
    pub fn new(replicas: usize) -> Self {
        Self {
            ring: BTreeMap::new(),
            replicas,
        }
    }

    pub fn add_node(&mut self, node: Node) {
        for i in 0..self.replicas {
            let virtual_id = format!("{}_{}", node.id, i);
            let hash = Self::hash(&virtual_id);
            self.ring.insert(hash, node.clone());
        }
    }

    pub fn remove_node(&mut self, node: &Node) {
        for i in 0..self.replicas {
            let virtual_id = format!("{}_{}", node.id, i);
            let hash = Self::hash(&virtual_id);
            self.ring.remove(&hash);
        }
    }

    pub fn get_node<T: Hash>(&self, key: &T) -> Option<&Node> {
        if self.is_empty() {
            return None;
        }
        let hash = Self::hash(&key);

        self.ring
            .range(hash..)
            .next()
            .or_else(|| self.ring.iter().next()) // if the next not exists back to the other side
            .map(|(_, node)| node)
    }

    pub fn nodes(&self) -> Vec<&Node> {
        let mut seen = std::collections::HashSet::new();
        self.ring
            .values()
            .filter(|&node| seen.insert(node.clone()))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.ring.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ring.is_empty()
    }

    fn hash<T: Hash>(item: &T) -> HashValue {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        hasher.finish()
    }
}
