//! # Garden Path
//!
//! Decision tree pruning via garden metaphor.
//!
//! Provides a garden-themed approach to decision tree management:
//! seeds as initial decisions, branches as decision paths, pruning
//! to remove low-value branches, grafting to merge trees, and
//! harvesting to extract final decisions.

/// Initial decision (seed).
pub mod seed {
    /// A decision seed with a score.
    #[derive(Debug, Clone)]
    pub struct Seed {
        label: String,
        score: f64,
        metadata: std::collections::HashMap<String, String>,
    }

    impl Seed {
        /// Create a new seed.
        pub fn new(label: &str, score: f64) -> Self {
            Self {
                label: label.to_string(),
                score,
                metadata: std::collections::HashMap::new(),
            }
        }

        /// Get the label.
        pub fn label(&self) -> &str {
            &self.label
        }

        /// Get the score.
        pub fn score(&self) -> f64 {
            self.score
        }

        /// Set the score.
        pub fn set_score(&mut self, score: f64) {
            self.score = score;
        }

        /// Add metadata.
        pub fn add_metadata(&mut self, key: &str, value: &str) {
            self.metadata.insert(key.to_string(), value.to_string());
        }

        /// Get metadata.
        pub fn get_metadata(&self, key: &str) -> Option<&str> {
            self.metadata.get(key).map(|s| s.as_str())
        }

        /// Check if seed is viable (score > threshold).
        pub fn is_viable(&self, threshold: f64) -> bool {
            self.score > threshold
        }

        /// Create a collection of seeds.
        pub fn collect(seeds: Vec<Seed>) -> SeedCollection {
            SeedCollection { seeds }
        }

        /// Merge two seeds by averaging their scores.
        pub fn merge(&self, other: &Seed) -> Seed {
            Seed {
                label: format!("({}+{})", self.label, other.label),
                score: (self.score + other.score) / 2.0,
                metadata: self.metadata.clone(),
            }
        }
    }

    impl PartialEq for Seed {
        fn eq(&self, other: &Self) -> bool {
            self.label == other.label && (self.score - other.score).abs() < 1e-10
        }
    }

    /// A collection of seeds.
    #[derive(Debug, Clone)]
    pub struct SeedCollection {
        seeds: Vec<Seed>,
    }

    impl SeedCollection {
        /// Create an empty collection.
        pub fn new() -> Self {
            Self { seeds: Vec::new() }
        }

        /// Add a seed.
        pub fn add(&mut self, seed: Seed) {
            self.seeds.push(seed);
        }

        /// Get seeds.
        pub fn seeds(&self) -> &[Seed] {
            &self.seeds
        }

        /// Number of seeds.
        pub fn len(&self) -> usize {
            self.seeds.len()
        }

        /// Check if empty.
        pub fn is_empty(&self) -> bool {
            self.seeds.is_empty()
        }

        /// Filter seeds above threshold.
        pub fn viable(&self, threshold: f64) -> Vec<&Seed> {
            self.seeds.iter().filter(|s| s.is_viable(threshold)).collect()
        }

        /// Get the best seed.
        pub fn best(&self) -> Option<&Seed> {
            self.seeds.iter().max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        }

        /// Sort seeds by score descending.
        pub fn sorted_by_score(&self) -> Vec<&Seed> {
            let mut s: Vec<&Seed> = self.seeds.iter().collect();
            s.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
            s
        }

        /// Average score.
        pub fn average_score(&self) -> f64 {
            if self.seeds.is_empty() {
                return 0.0;
            }
            self.seeds.iter().map(|s| s.score).sum::<f64>() / self.seeds.len() as f64
        }
    }

    impl Default for SeedCollection {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_seed_creation() {
            let s = Seed::new("decision_a", 0.8);
            assert_eq!(s.label(), "decision_a");
            assert!((s.score() - 0.8).abs() < 1e-10);
        }

        #[test]
        fn test_set_score() {
            let mut s = Seed::new("a", 0.5);
            s.set_score(0.9);
            assert!((s.score() - 0.9).abs() < 1e-10);
        }

        #[test]
        fn test_metadata() {
            let mut s = Seed::new("a", 0.5);
            s.add_metadata("source", "model_v1");
            assert_eq!(s.get_metadata("source"), Some("model_v1"));
            assert_eq!(s.get_metadata("missing"), None);
        }

        #[test]
        fn test_is_viable() {
            let s = Seed::new("a", 0.8);
            assert!(s.is_viable(0.5));
            assert!(!s.is_viable(0.9));
        }

        #[test]
        fn test_merge() {
            let a = Seed::new("a", 0.6);
            let b = Seed::new("b", 0.8);
            let merged = a.merge(&b);
            assert!((merged.score() - 0.7).abs() < 1e-10);
        }

        #[test]
        fn test_seed_equality() {
            let a = Seed::new("x", 0.5);
            let b = Seed::new("x", 0.5);
            assert_eq!(a, b);
        }

        #[test]
        fn test_seed_inequality() {
            let a = Seed::new("x", 0.5);
            let b = Seed::new("y", 0.5);
            assert_ne!(a, b);
        }

        #[test]
        fn test_collection_creation() {
            let col = SeedCollection::new();
            assert!(col.is_empty());
        }

        #[test]
        fn test_collection_add() {
            let mut col = SeedCollection::new();
            col.add(Seed::new("a", 0.5));
            col.add(Seed::new("b", 0.8));
            assert_eq!(col.len(), 2);
        }

        #[test]
        fn test_collection_viable() {
            let mut col = SeedCollection::new();
            col.add(Seed::new("a", 0.3));
            col.add(Seed::new("b", 0.8));
            assert_eq!(col.viable(0.5).len(), 1);
        }

        #[test]
        fn test_collection_best() {
            let mut col = SeedCollection::new();
            col.add(Seed::new("a", 0.3));
            col.add(Seed::new("b", 0.9));
            col.add(Seed::new("c", 0.5));
            assert_eq!(col.best().unwrap().label(), "b");
        }

        #[test]
        fn test_collection_sorted() {
            let mut col = SeedCollection::new();
            col.add(Seed::new("a", 0.3));
            col.add(Seed::new("b", 0.9));
            col.add(Seed::new("c", 0.5));
            let sorted = col.sorted_by_score();
            assert_eq!(sorted[0].label(), "b");
            assert_eq!(sorted[1].label(), "c");
            assert_eq!(sorted[2].label(), "a");
        }

        #[test]
        fn test_collection_average() {
            let mut col = SeedCollection::new();
            col.add(Seed::new("a", 0.4));
            col.add(Seed::new("b", 0.6));
            assert!((col.average_score() - 0.5).abs() < 1e-10);
        }

        #[test]
        fn test_collection_default() {
            let col = SeedCollection::default();
            assert!(col.is_empty());
        }

        #[test]
        fn test_collect_convenience() {
            let col = Seed::collect(vec![
                Seed::new("a", 0.5),
                Seed::new("b", 0.8),
            ]);
            assert_eq!(col.len(), 2);
        }
    }
}

/// Decision path (branch).
pub mod branch {
    /// A node in the decision tree.
    #[derive(Debug, Clone)]
    pub struct DecisionNode {
        pub(crate) id: String,
        pub(crate) label: String,
        pub(crate) score: f64,
        pub(crate) children: Vec<DecisionNode>,
    }

    impl DecisionNode {
        /// Create a new decision node.
        pub fn new(id: &str, label: &str, score: f64) -> Self {
            Self {
                id: id.to_string(),
                label: label.to_string(),
                score,
                children: Vec::new(),
            }
        }

        /// Get the ID.
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Get the label.
        pub fn label(&self) -> &str {
            &self.label
        }

        /// Get the score.
        pub fn score(&self) -> f64 {
            self.score
        }

        /// Set the score.
        pub fn set_score(&mut self, score: f64) {
            self.score = score;
        }

        /// Add a child node.
        pub fn add_child(&mut self, child: DecisionNode) {
            self.children.push(child);
        }

        /// Get children.
        pub fn children(&self) -> &[DecisionNode] {
            &self.children
        }

        /// Get mutable children.
        pub fn children_mut(&mut self) -> &mut Vec<DecisionNode> {
            &mut self.children
        }

        /// Check if leaf.
        pub fn is_leaf(&self) -> bool {
            self.children.is_empty()
        }

        /// Depth of the tree.
        pub fn depth(&self) -> usize {
            if self.children.is_empty() {
                1
            } else {
                1 + self.children.iter().map(|c| c.depth()).max().unwrap_or(0)
            }
        }

        /// Total number of nodes.
        pub fn count_nodes(&self) -> usize {
            1 + self.children.iter().map(|c| c.count_nodes()).sum::<usize>()
        }

        /// Total number of leaves.
        pub fn count_leaves(&self) -> usize {
            if self.children.is_empty() {
                1
            } else {
                self.children.iter().map(|c| c.count_leaves()).sum()
            }
        }

        /// Find a node by ID.
        pub fn find(&self, id: &str) -> Option<&DecisionNode> {
            if self.id == id {
                return Some(self);
            }
            for child in &self.children {
                if let Some(found) = child.find(id) {
                    return Some(found);
                }
            }
            None
        }

        /// Find a mutable node by ID.
        pub fn find_mut(&mut self, id: &str) -> Option<&mut DecisionNode> {
            if self.id == id {
                return Some(self);
            }
            for child in &mut self.children {
                if let Some(found) = child.find_mut(id) {
                    return Some(found);
                }
            }
            None
        }

        /// Get all paths from root to leaves.
        pub fn all_paths(&self) -> Vec<Vec<&DecisionNode>> {
            if self.children.is_empty() {
                return vec![vec![self]];
            }
            let mut paths = Vec::new();
            for child in &self.children {
                for mut path in child.all_paths() {
                    let mut full = vec![self];
                    full.append(&mut path);
                    paths.push(full);
                }
            }
            paths
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_node_creation() {
            let n = DecisionNode::new("n1", "decide", 0.5);
            assert_eq!(n.id(), "n1");
            assert_eq!(n.label(), "decide");
            assert!((n.score() - 0.5).abs() < 1e-10);
        }

        #[test]
        fn test_is_leaf() {
            let n = DecisionNode::new("n1", "a", 0.5);
            assert!(n.is_leaf());
        }

        #[test]
        fn test_add_child() {
            let mut root = DecisionNode::new("root", "r", 0.5);
            root.add_child(DecisionNode::new("c1", "c", 0.3));
            assert_eq!(root.children().len(), 1);
            assert!(!root.is_leaf());
        }

        #[test]
        fn test_depth_single() {
            let n = DecisionNode::new("n", "a", 0.5);
            assert_eq!(n.depth(), 1);
        }

        #[test]
        fn test_depth_tree() {
            let mut root = DecisionNode::new("root", "r", 0.5);
            let mut child = DecisionNode::new("c1", "c", 0.3);
            child.add_child(DecisionNode::new("c2", "cc", 0.1));
            root.add_child(child);
            assert_eq!(root.depth(), 3);
        }

        #[test]
        fn test_count_nodes() {
            let mut root = DecisionNode::new("root", "r", 0.5);
            root.add_child(DecisionNode::new("c1", "a", 0.3));
            root.add_child(DecisionNode::new("c2", "b", 0.3));
            assert_eq!(root.count_nodes(), 3);
        }

        #[test]
        fn test_count_leaves() {
            let mut root = DecisionNode::new("root", "r", 0.5);
            root.add_child(DecisionNode::new("c1", "a", 0.3));
            root.add_child(DecisionNode::new("c2", "b", 0.3));
            assert_eq!(root.count_leaves(), 2);
        }

        #[test]
        fn test_find() {
            let mut root = DecisionNode::new("root", "r", 0.5);
            root.add_child(DecisionNode::new("c1", "a", 0.3));
            assert!(root.find("c1").is_some());
            assert!(root.find("missing").is_none());
        }

        #[test]
        fn test_find_mut() {
            let mut root = DecisionNode::new("root", "r", 0.5);
            root.add_child(DecisionNode::new("c1", "a", 0.3));
            if let Some(node) = root.find_mut("c1") {
                node.set_score(0.9);
            }
            assert!((root.find("c1").unwrap().score() - 0.9).abs() < 1e-10);
        }

        #[test]
        fn test_all_paths() {
            let mut root = DecisionNode::new("root", "r", 0.5);
            root.add_child(DecisionNode::new("c1", "a", 0.3));
            root.add_child(DecisionNode::new("c2", "b", 0.3));
            let paths = root.all_paths();
            assert_eq!(paths.len(), 2);
            assert_eq!(paths[0].len(), 2);
            assert_eq!(paths[1].len(), 2);
        }

        #[test]
        fn test_set_score() {
            let mut n = DecisionNode::new("n", "a", 0.5);
            n.set_score(0.9);
            assert!((n.score() - 0.9).abs() < 1e-10);
        }

        #[test]
        fn test_children_mut() {
            let mut root = DecisionNode::new("root", "r", 0.5);
            root.add_child(DecisionNode::new("c1", "a", 0.3));
            root.children_mut().push(DecisionNode::new("c2", "b", 0.4));
            assert_eq!(root.children().len(), 2);
        }
    }
}

/// Remove low-value branches.
pub mod prune {
    use super::branch::DecisionNode;

    /// Pruning strategy.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum PruneStrategy {
        /// Remove branches with score below threshold.
        Threshold(f64),
        /// Keep only the top N branches.
        TopN(usize),
        /// Remove branches with score below parent score * factor.
        Relative(f64),
    }

    /// Prune a tree using the given strategy.
    pub fn prune(node: &mut DecisionNode, strategy: &PruneStrategy) -> usize {
        match strategy {
            PruneStrategy::Threshold(thresh) => prune_threshold(node, *thresh),
            PruneStrategy::TopN(n) => prune_top_n(node, *n),
            PruneStrategy::Relative(factor) => prune_relative(node, *factor),
        }
    }

    fn prune_threshold(node: &mut DecisionNode, threshold: f64) -> usize {
        let original_count = node.children.len();
        node.children.retain(|c| c.score >= threshold);
        let mut removed = original_count - node.children.len();
        for child in &mut node.children {
            removed += prune_threshold(child, threshold);
        }
        removed
    }

    fn prune_top_n(node: &mut DecisionNode, n: usize) -> usize {
        let original_count = node.children.len();
        node.children.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        node.children.truncate(n);
        let mut removed = original_count - node.children.len();
        for child in &mut node.children {
            removed += prune_top_n(child, n);
        }
        removed
    }

    fn prune_relative(node: &mut DecisionNode, factor: f64) -> usize {
        let threshold = node.score * factor;
        let original_count = node.children.len();
        node.children.retain(|c| c.score >= threshold);
        let mut removed = original_count - node.children.len();
        for child in &mut node.children {
            removed += prune_relative(child, factor);
        }
        removed
    }

    /// Prune to a maximum depth.
    pub fn prune_to_depth(node: &mut DecisionNode, max_depth: usize) -> usize {
        prune_depth_inner(node, max_depth, 1)
    }

    fn prune_depth_inner(node: &mut DecisionNode, max_depth: usize, current_depth: usize) -> usize {
        if current_depth >= max_depth {
            let removed = node.children.len();
            node.children.clear();
            return removed;
        }
        let mut removed = 0;
        for child in &mut node.children {
            removed += prune_depth_inner(child, max_depth, current_depth + 1);
        }
        removed
    }

    /// Collapse single-child chains (compress paths).
    pub fn collapse_chains(node: &mut DecisionNode) -> usize {
        let mut collapsed = 0;
        while node.children.len() == 1 {
            let child = node.children.pop().unwrap();
            node.label = format!("{} -> {}", node.label, child.label);
            node.score = child.score;
            node.children = child.children;
            collapsed += 1;
        }
        for child in &mut node.children {
            collapsed += collapse_chains(child);
        }
        collapsed
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn sample_tree() -> DecisionNode {
            let mut root = DecisionNode::new("root", "r", 0.5);
            root.add_child(DecisionNode::new("c1", "good", 0.9));
            root.add_child(DecisionNode::new("c2", "ok", 0.6));
            root.add_child(DecisionNode::new("c3", "bad", 0.2));
            root
        }

        #[test]
        fn test_prune_threshold() {
            let mut tree = sample_tree();
            let removed = prune(&mut tree, &PruneStrategy::Threshold(0.5));
            assert_eq!(removed, 1);
            assert_eq!(tree.children().len(), 2);
        }

        #[test]
        fn test_prune_threshold_all() {
            let mut tree = sample_tree();
            let removed = prune(&mut tree, &PruneStrategy::Threshold(1.0));
            assert_eq!(removed, 3);
            assert!(tree.children().is_empty());
        }

        #[test]
        fn test_prune_threshold_none() {
            let mut tree = sample_tree();
            let removed = prune(&mut tree, &PruneStrategy::Threshold(0.0));
            assert_eq!(removed, 0);
        }

        #[test]
        fn test_prune_top_n() {
            let mut tree = sample_tree();
            let removed = prune(&mut tree, &PruneStrategy::TopN(2));
            assert_eq!(removed, 1);
            assert_eq!(tree.children().len(), 2);
        }

        #[test]
        fn test_prune_top_n_zero() {
            let mut tree = sample_tree();
            let removed = prune(&mut tree, &PruneStrategy::TopN(0));
            assert_eq!(removed, 3);
        }

        #[test]
        fn test_prune_relative() {
            let mut tree = sample_tree();
            let removed = prune(&mut tree, &PruneStrategy::Relative(0.5));
            // root score is 0.5, threshold = 0.25, only 0.2 < 0.25
            assert_eq!(removed, 1);
        }

        #[test]
        fn test_prune_to_depth() {
            let mut root = DecisionNode::new("root", "r", 0.5);
            let mut child = DecisionNode::new("c1", "c", 0.3);
            child.add_child(DecisionNode::new("c2", "cc", 0.1));
            root.add_child(child);
            let removed = prune_to_depth(&mut root, 2);
            assert_eq!(removed, 1);
            assert!(root.children()[0].is_leaf());
        }

        #[test]
        fn test_prune_to_depth_no_change() {
            let mut tree = sample_tree();
            let removed = prune_to_depth(&mut tree, 10);
            assert_eq!(removed, 0);
        }

        #[test]
        fn test_collapse_chains() {
            let mut root = DecisionNode::new("root", "a", 0.5);
            let mut mid = DecisionNode::new("mid", "b", 0.3);
            mid.add_child(DecisionNode::new("leaf", "c", 0.1));
            root.add_child(mid);
            let collapsed = collapse_chains(&mut root);
            assert_eq!(collapsed, 2);
            assert!(root.is_leaf());
        }

        #[test]
        fn test_collapse_no_chains() {
            let mut tree = sample_tree();
            let collapsed = collapse_chains(&mut tree);
            assert_eq!(collapsed, 0);
        }
    }
}

/// Merge decision trees (graft).
pub mod graft {
    use super::branch::DecisionNode;

    /// Graft one tree onto another at a specified node.
    pub fn graft_at(host: &mut DecisionNode, scion: DecisionNode, target_id: &str) -> bool {
        if let Some(node) = host.find_mut(target_id) {
            node.add_child(scion);
            true
        } else {
            false
        }
    }

    /// Merge two trees by combining children at the root.
    pub fn merge_roots(tree_a: &mut DecisionNode, tree_b: DecisionNode) {
        for child in tree_b.children {
            tree_a.add_child(child);
        }
    }

    /// Merge two leaf nodes into a decision.
    pub fn merge_leaves(a: &DecisionNode, b: &DecisionNode) -> DecisionNode {
        let mut merged = DecisionNode::new(
            &format!("merged_{}_{}", a.id(), b.id()),
            &format!("({} | {})", a.label(), b.label()),
            (a.score() + b.score()) / 2.0,
        );
        merged.add_child(a.clone());
        merged.add_child(b.clone());
        merged
    }

    /// Interleave two trees at each level.
    pub fn interleave(tree_a: &mut DecisionNode, tree_b: &DecisionNode) {
        for (i, child_b) in tree_b.children.iter().enumerate() {
            if i < tree_a.children.len() {
                interleave(&mut tree_a.children[i], child_b);
            } else {
                tree_a.add_child(child_b.clone());
            }
        }
    }

    /// Replace a subtree at a given node ID.
    pub fn replace_subtree(root: &mut DecisionNode, target_id: &str, replacement: DecisionNode) -> bool {
        if root.id == target_id {
            *root = replacement;
            return true;
        }
        for child in &mut root.children {
            if replace_subtree(child, target_id, replacement.clone()) {
                return true;
            }
        }
        false
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_graft_at_found() {
            let mut host = DecisionNode::new("root", "r", 0.5);
            host.add_child(DecisionNode::new("c1", "a", 0.3));
            let scion = DecisionNode::new("s1", "s", 0.8);
            assert!(graft_at(&mut host, scion, "c1"));
            assert_eq!(host.find("c1").unwrap().children().len(), 1);
        }

        #[test]
        fn test_graft_at_not_found() {
            let mut host = DecisionNode::new("root", "r", 0.5);
            let scion = DecisionNode::new("s1", "s", 0.8);
            assert!(!graft_at(&mut host, scion, "missing"));
        }

        #[test]
        fn test_merge_roots() {
            let mut a = DecisionNode::new("a", "ra", 0.5);
            a.add_child(DecisionNode::new("a1", "x", 0.3));
            let mut b = DecisionNode::new("b", "rb", 0.6);
            b.add_child(DecisionNode::new("b1", "y", 0.4));
            merge_roots(&mut a, b);
            assert_eq!(a.children().len(), 2);
        }

        #[test]
        fn test_merge_leaves() {
            let a = DecisionNode::new("a", "la", 0.6);
            let b = DecisionNode::new("b", "lb", 0.8);
            let merged = merge_leaves(&a, &b);
            assert!((merged.score() - 0.7).abs() < 1e-10);
            assert_eq!(merged.children().len(), 2);
        }

        #[test]
        fn test_interleave() {
            let mut a = DecisionNode::new("root", "r", 0.5);
            a.add_child(DecisionNode::new("a1", "x", 0.3));
            let mut b = DecisionNode::new("root", "r", 0.5);
            b.add_child(DecisionNode::new("b1", "y", 0.4));
            b.add_child(DecisionNode::new("b2", "z", 0.2));
            interleave(&mut a, &b);
            assert_eq!(a.children().len(), 2);
        }

        #[test]
        fn test_replace_subtree_found() {
            let mut root = DecisionNode::new("root", "r", 0.5);
            root.add_child(DecisionNode::new("c1", "old", 0.3));
            let replacement = DecisionNode::new("new", "new", 0.9);
            assert!(replace_subtree(&mut root, "c1", replacement));
            assert_eq!(root.children()[0].id(), "new");
        }

        #[test]
        fn test_replace_subtree_not_found() {
            let mut root = DecisionNode::new("root", "r", 0.5);
            let replacement = DecisionNode::new("new", "new", 0.9);
            assert!(!replace_subtree(&mut root, "missing", replacement));
        }

        #[test]
        fn test_replace_root() {
            let mut root = DecisionNode::new("root", "r", 0.5);
            let replacement = DecisionNode::new("new_root", "nr", 0.9);
            assert!(replace_subtree(&mut root, "root", replacement));
            assert_eq!(root.id(), "new_root");
        }
    }
}

/// Extract final decisions from pruned tree (harvest).
pub mod harvest {
    use super::branch::DecisionNode;

    /// A harvested decision.
    #[derive(Debug, Clone)]
    pub struct HarvestedDecision {
        pub path: Vec<String>,
        pub labels: Vec<String>,
        pub final_score: f64,
    }

    impl HarvestedDecision {
        /// Create a new harvested decision.
        pub fn new(path: Vec<String>, labels: Vec<String>, score: f64) -> Self {
            Self {
                path,
                labels,
                final_score: score,
            }
        }

        /// Get the final decision label.
        pub fn final_label(&self) -> &str {
            self.labels.last().map(|s| s.as_str()).unwrap_or("")
        }

        /// Depth of the decision.
        pub fn depth(&self) -> usize {
            self.path.len()
        }
    }

    /// Harvest all leaf decisions from a tree.
    pub fn harvest_leaves(tree: &DecisionNode) -> Vec<HarvestedDecision> {
        let paths = tree.all_paths();
        paths
            .into_iter()
            .map(|path| {
                let ids: Vec<String> = path.iter().map(|n| n.id().to_string()).collect();
                let labels: Vec<String> = path.iter().map(|n| n.label().to_string()).collect();
                let score = path.last().map(|n| n.score()).unwrap_or(0.0);
                HarvestedDecision::new(ids, labels, score)
            })
            .collect()
    }

    /// Harvest only the best decision (highest leaf score).
    pub fn harvest_best(tree: &DecisionNode) -> Option<HarvestedDecision> {
        let leaves = harvest_leaves(tree);
        leaves.into_iter().max_by(|a, b| a.final_score.partial_cmp(&b.final_score).unwrap())
    }

    /// Harvest decisions above a score threshold.
    pub fn harvest_above_threshold(tree: &DecisionNode, threshold: f64) -> Vec<HarvestedDecision> {
        harvest_leaves(tree)
            .into_iter()
            .filter(|d| d.final_score >= threshold)
            .collect()
    }

    /// Compute the aggregate score of all decisions.
    pub fn aggregate_score(tree: &DecisionNode) -> f64 {
        let decisions = harvest_leaves(tree);
        if decisions.is_empty() {
            return 0.0;
        }
        decisions.iter().map(|d| d.final_score).sum::<f64>() / decisions.len() as f64
    }

    /// Rank decisions by score.
    pub fn rank_decisions(tree: &DecisionNode) -> Vec<HarvestedDecision> {
        let mut decisions = harvest_leaves(tree);
        decisions.sort_by(|a, b| b.final_score.partial_cmp(&a.final_score).unwrap());
        decisions
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn sample_tree() -> DecisionNode {
            let mut root = DecisionNode::new("root", "start", 0.5);
            root.add_child(DecisionNode::new("a", "option_a", 0.9));
            root.add_child(DecisionNode::new("b", "option_b", 0.3));
            root
        }

        #[test]
        fn test_harvested_decision_creation() {
            let d = HarvestedDecision::new(
                vec!["root".into(), "a".into()],
                vec!["start".into(), "option_a".into()],
                0.9,
            );
            assert_eq!(d.final_label(), "option_a");
            assert_eq!(d.depth(), 2);
        }

        #[test]
        fn test_harvest_leaves() {
            let tree = sample_tree();
            let leaves = harvest_leaves(&tree);
            assert_eq!(leaves.len(), 2);
        }

        #[test]
        fn test_harvest_best() {
            let tree = sample_tree();
            let best = harvest_best(&tree).unwrap();
            assert!((best.final_score - 0.9).abs() < 1e-10);
        }

        #[test]
        fn test_harvest_best_empty() {
            let tree = DecisionNode::new("root", "r", 0.5);
            let best = harvest_best(&tree);
            assert!(best.is_some());
            assert_eq!(best.unwrap().final_label(), "r");
        }

        #[test]
        fn test_harvest_above_threshold() {
            let tree = sample_tree();
            let above = harvest_above_threshold(&tree, 0.5);
            assert_eq!(above.len(), 1);
        }

        #[test]
        fn test_harvest_above_threshold_all() {
            let tree = sample_tree();
            let above = harvest_above_threshold(&tree, 0.0);
            assert_eq!(above.len(), 2);
        }

        #[test]
        fn test_aggregate_score() {
            let tree = sample_tree();
            let avg = aggregate_score(&tree);
            assert!((avg - 0.6).abs() < 1e-10);
        }

        #[test]
        fn test_rank_decisions() {
            let tree = sample_tree();
            let ranked = rank_decisions(&tree);
            assert!((ranked[0].final_score - 0.9).abs() < 1e-10);
            assert!((ranked[1].final_score - 0.3).abs() < 1e-10);
        }

        #[test]
        fn test_harvest_deep_tree() {
            let mut root = DecisionNode::new("root", "r", 0.5);
            let mut mid = DecisionNode::new("mid", "m", 0.4);
            mid.add_child(DecisionNode::new("leaf", "l", 0.8));
            root.add_child(mid);
            let leaves = harvest_leaves(&root);
            assert_eq!(leaves.len(), 1);
            assert_eq!(leaves[0].depth(), 3);
        }
    }
}

pub use seed::{Seed, SeedCollection};
pub use branch::DecisionNode;
pub use prune::PruneStrategy;
pub use harvest::HarvestedDecision;
