use std::collections::HashMap;
use std::hash::Hash;
#[derive(Debug, Clone)]
pub struct KeyedStack<K, V> {
    stacks: HashMap<K, Vec<(usize, V)>>,
    length: usize,
}

impl<K, V> Default for KeyedStack<K, V> {
    fn default() -> Self {
        Self {
            stacks: HashMap::new(),
            length: 0,
        }
    }
}

impl<K: Hash + Eq, V> KeyedStack<K, V> {
    pub const fn len(&self) -> usize {
        self.length
    }
    pub fn push(&mut self, key: K, value: V) {
        self.stacks
            .entry(key)
            .or_default()
            .push((self.length, value));
        self.length += 1;
    }
    pub fn pop(&mut self, key: &K) -> Option<V> {
        self.length -= 1;
        self.stacks
            .get_mut(key)
            .and_then(|stack| stack.pop().map(|(_, v)| v))
    }
    pub fn find(&self, key: &K) -> Option<(usize, &V)> {
        self.stacks
            .get(key)
            .and_then(|stack| stack.last().map(|(idx, v)| (*idx, v)))
    }
}
