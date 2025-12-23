use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scope<K: Hash + Eq, V> {
    pub current: HashMap<K, V>,
    pub parent: Option<Box<Self>>,
}

impl<K: Hash + Eq, V> Default for Scope<K, V> {
    fn default() -> Self {
        Self {
            current: HashMap::new(),
            parent: None,
        }
    }
}
impl<K: Hash + Eq, V> Scope<K, V> {
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.current
            .get(k)
            .or_else(|| self.parent.as_ref().and_then(|p| p.get(k)))
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.current.insert(k, v)
    }

    pub fn last_scope(&mut self) {
        *self = *std::mem::take(self).parent.unwrap();
    }

    pub fn new_scope(&mut self) {
        let took = std::mem::take(self);
        *self = Self {
            current: HashMap::new(),
            parent: Some(Box::new(took)),
        }
    }
}
