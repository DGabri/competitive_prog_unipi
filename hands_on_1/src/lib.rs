pub struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left == None,
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right == None,
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    /* BELOW MY FUNCTIONS ARE IMPLEMENTED */
    pub fn is_bst(&self) -> bool {
        if self.nodes.is_empty() {
            return true; // tree is empty so it is a bst
        }

        self.check_bst(0).0 // call helper function to check if the tree is a BST and return the first tuple value returned by the helper function
    }

    // function to get the min between 3 vals
    pub fn get_min(first: u32, second: u32, third: u32) -> u32 {
        first.min(second).min(third)
    }

    // function to get the max between 3 vals
    pub fn get_max(first: u32, second: u32, third: u32) -> u32 {
        first.max(second).max(third)
    }

    // helper function to check if a tree is a binary search tree
    // returns a triple (isBst: bool, minChildrenVal: u32, maxCHildrenVal: u32) -- u32 as defined in tree.key
    pub fn check_bst(&self, node_id: usize) -> (bool, u32, u32) {
        let node = &self.nodes[node_id];

        // Visit dx subtree
        let (is_dx_bst, dx_min, dx_max) = match node.id_right {
            Some(dx_child) => self.check_bst(dx_child),
            None => (true, u32::MAX, u32::MIN), // base case
        };

        // Visit sx subtree
        let (is_sx_bst, sx_min, sx_max) = match node.id_left {
            Some(sx_child) => self.check_bst(sx_child),
            None => (true, u32::MAX, u32::MIN), // base case
        };

        // combine conditions for recursion
        (
            ((is_dx_bst && is_sx_bst) && (sx_max < node.key) && (dx_min > node.key)),
            Self::get_min(node.key, dx_min, sx_min),
            Self::get_max(node.key, dx_max, sx_max),
        )
    }

    // same function logic as is_bst
    pub fn max_path_sum(&self) -> u32 {
        if self.nodes.is_empty() {
            return 0;
        }
        self.get_max_path(0).1 // call the helper function and return the second tuple value returned by the helper function which is the max path sum
    }

    // returns a tuple
    pub fn get_max_path(&self, node_id: usize) -> (u32, u32) {
        let node = &self.nodes[node_id];

        // base case, leaf
        if node.id_left.is_none() && node.id_right.is_none() {
            return (node.key, node.key);
        }

        // recursion on left and then right child (if present)
        let (sx_to_leaf, sx_path_sum) = match node.id_left {
            Some(sx) => self.get_max_path(sx),
            None => (0, 0),
        };

        let (dx_to_leaf, dx_path_sum) = match node.id_right {
            Some(dx) => self.get_max_path(dx),
            None => (0, 0),
        };

        // result combination
        (
            node.key + sx_to_leaf.max(dx_to_leaf),
            sx_path_sum
                .max(dx_path_sum)
                .max(node.key + sx_to_leaf + dx_to_leaf),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_bst() {
        let tree = Tree { nodes: vec![] };
        assert!(tree.is_bst(), "Empty tree is a BST");
    }

    #[test]
    fn test_single_node_bst() {
        let tree = Tree::with_root(10);
        assert!(tree.is_bst(), "A tree with one node is a BST");
    }

    #[test]
    fn test_valid_bst() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 4, true);
        tree.add_node(0, 14, false);
        tree.add_node(1, 2, true);
        tree.add_node(1, 8, false);
        tree.add_node(2, 12, true);
        tree.add_node(2, 20, false);

        assert!(tree.is_bst(), "This should be a BST");
    }

    #[test]
    fn test_invalid_bst_left_subtree() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(1, 30, false);

        assert!(
            !tree.is_bst(),
            "30 is dx child of 5 so 30 is in sx subtree => invalid because 30 > 10 (root) and is in sx subtree"
        );
    }

    #[test]
    fn test_bst_with_duplicate_node_values() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 10, true);

        assert!(
            !tree.is_bst(),
            "A tree with duplicate node values is not a BST"
        );
    }

    #[test]
    fn test_valid_bst_big() {
        let mut tree = Tree::with_root(8);
        tree.add_node(0, 3, true);
        tree.add_node(0, 10, false);
        tree.add_node(1, 1, true);
        tree.add_node(1, 6, false);
        tree.add_node(2, 9, true);
        tree.add_node(2, 14, false);
        tree.add_node(5, 7, true);
        tree.add_node(6, 13, true);

        assert!(
            !tree.is_bst(),
            "This is invalid, 7 is less than 8 but 7 is child of 9 (dx subtree)"
        );
    }

    #[test]
    fn test_linear_bst() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 9, true);
        tree.add_node(1, 7, true);

        assert!(tree.is_bst(), "This is a valid BST");
    }

    /* MAXIMUM PATH SUM TESTS */
    #[test]
    fn test_mps_no_node() {
        let tree = Tree { nodes: vec![] };
        assert_eq!(tree.max_path_sum(), 0);
    }

    #[test]
    fn test_mps_single_node() {
        let tree = Tree::with_root(10);
        assert_eq!(tree.max_path_sum(), 10);
    }

    #[test]
    fn test_mps_single_path() {
        let mut tree = Tree::with_root(5);
        tree.add_node(0, 4, true);
        tree.add_node(0, 7, false);
        assert_eq!(tree.max_path_sum(), 16); // 5 + 4 + 7 = 16
    }

    #[test]
    fn test_mps_linear_tree() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 8, true);
        tree.add_node(1, 7, true);
        tree.add_node(2, 3, true);
        assert_eq!(tree.max_path_sum(), 28); // 10 + 8 + 7 + 3 = 28
    }

    #[test]
    fn test_mps_piramid_tree_one() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 2, true);
        tree.add_node(0, 21, false);

        tree.add_node(1, 20, true);
        tree.add_node(2, 3, true);
        tree.add_node(2, 40, false);
        assert_eq!(tree.max_path_sum(), 93); // 20 + 2 + 10 + 21 + 40 = 93
    }

    #[test]
    fn test_mps_piramid_tree_two() {
        // Remove 40
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 2, true);
        tree.add_node(0, 21, false);

        tree.add_node(1, 20, true);
        tree.add_node(2, 3, true);
        assert_eq!(tree.max_path_sum(), 56); // 20 + 2 + 10 +21 + 3 = 56
    }
}
