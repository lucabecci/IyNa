#[cfg(test)]
mod ring_test {
    use crate::node::Node;
    use crate::ring::HashRing;
    #[test]
    fn test_add_new_node() {
        let replication = 3;
        let mut ring = HashRing::new(replication);
        let node = Node {
            id: "B".to_string(),
        };
        ring.add_node(node.clone());

        let my_key = "test";
        let result = ring.get_node(&my_key);

        assert_eq!(result, Some(&node));
        assert_eq!(ring.len(), 3);
    }

    #[test]
    fn test_add_multiple_nodes_different_distribution() {
        let replication = 3;
        let mut ring = HashRing::new(replication);
        let node1 = Node {
            id: "A".to_string(),
        };
        let node2 = Node {
            id: "B".to_string(),
        };
        ring.add_node(node1.clone());
        ring.add_node(node2.clone());

        assert_eq!(ring.len(), 6);
        let all_nodes = ring.nodes();
        assert!(all_nodes.contains(&&node1));
        assert!(all_nodes.contains(&&node2));
    }

    #[test]
    fn test_remove_node() {
        let replication = 3;
        let mut ring = HashRing::new(replication);
        let node1 = Node {
            id: "A".to_string(),
        };
        let node2 = Node {
            id: "B".to_string(),
        };
        ring.add_node(node1.clone());
        ring.add_node(node2.clone());

        let key = "some-key";
        let assigned_node = ring.get_node(&key).cloned();
        assert!(assigned_node.is_some());

        ring.remove_node(&node2);

        let after = ring.get_node(&key).cloned();
        assert!(after.is_some());
        assert_eq!(ring.len(), 3);
        assert!(!ring.nodes().contains(&&node2));
        assert!(ring.nodes().contains(&&node1));
    }
}
