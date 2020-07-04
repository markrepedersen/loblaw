use std::collections::HashMap;

/// Trie implementation for string prefix matching.
#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    #[inline]
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    #[inline]
    fn root(&self) -> &TrieNode {
        &self.root
    }

    #[inline]
    fn root_mut(&mut self) -> &mut TrieNode {
        &mut self.root
    }

    #[inline]
    pub fn contains(&self, word: &str) -> bool {
        let mut cur_node = &self.root().clone();
        let mut matching_word = String::new();
        for c in word.chars() {
            if cur_node.contains_key(c) {
                matching_word.push(c);
                cur_node = cur_node.get_child(c).unwrap();
            } else {
                return false;
            }
        }

        matching_word == word && cur_node.is_end
    }

    #[inline]
    /// Insert a word into the Trie.
    pub fn insert(&mut self, word: &str) {
        let mut cur_node = self.root_mut();
        for c in word.chars() {
            if cur_node.contains_key(c) {
                cur_node = cur_node.get_child_mut(c).unwrap();
            } else {
                cur_node.children_mut().insert(c, TrieNode::new());
                cur_node = cur_node.get_child_mut(c).unwrap();
            }
        }

        cur_node.set_end();
    }

    #[inline]
    /// Get the longest word that matches the given prefix.
    /// If there is more than one, return the first.
    /// If no words match prefix, return None.
    pub fn longest_prefix(&self, prefix: &str) -> Option<String> {
        let mut matching_word = String::new();
        let mut cur_node = &self.root.clone();
        for c in prefix.chars() {
            if let Some(child) = cur_node.get_child(c) {
                matching_word.push(c);
                cur_node = child;
            } else {
                return None;
            }
        }

        if !cur_node.is_end || matching_word.is_empty() {
            None
        } else {
            Some(matching_word)
        }
    }
}

#[derive(Default, Debug, Clone)]
struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_end: bool,
}

impl TrieNode {
    #[inline]
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end: false,
        }
    }

    fn contains_key(&self, key: char) -> bool {
        self.children.contains_key(&key)
    }

    #[inline]
    pub fn get_child(&self, key: char) -> Option<&TrieNode> {
        self.children().get(&key)
    }

    #[inline]
    pub fn get_child_mut(&mut self, key: char) -> Option<&mut TrieNode> {
        self.children_mut().get_mut(&key)
    }

    #[inline]
    pub fn children(&self) -> &HashMap<char, TrieNode> {
        &self.children
    }

    #[inline]
    pub fn children_mut(&mut self) -> &mut HashMap<char, TrieNode> {
        &mut self.children
    }

    #[inline]
    pub fn set_end(&mut self) {
        self.is_end = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion() {
        let mut trie = Trie::new();
        trie.insert("tr");
        trie.insert("tri");
        trie.insert("trie");
        trie.insert("hello");

        assert!(trie.contains("tr"));
        assert!(trie.contains("tri"));
        assert!(trie.contains("trie"));
        assert!(trie.contains("hello"));
        assert!(!trie.contains("hell"));
    }

    #[test]
    fn test_longest_prefix_sad_path() {
        let mut trie = Trie::new();
        trie.insert("tr");
        trie.insert("tri");
        trie.insert("trie");
        trie.insert("hello");

        assert_eq!(trie.longest_prefix("hello mark"), None);
    }

    #[test]
    fn test_longest_prefix_happy_path() {
        let mut trie = Trie::new();
        trie.insert("tr");
        trie.insert("tri");
        trie.insert("trie");
        trie.insert("hello");

        assert_eq!(trie.longest_prefix("trie"), Some("trie".to_string()));
        assert_eq!(trie.longest_prefix("hello"), Some("hello".to_string()));
    }
}
