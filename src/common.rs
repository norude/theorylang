static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
fn get_id() -> usize {
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(usize);
impl Id {
    pub fn new() -> Self {
        Self(get_id())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Scope(Id);
impl Scope {
    pub fn new() -> Self {
        Self(Id::new())
    }
}
#[derive(Clone, PartialEq, Eq, Copy, Hash)]
pub struct Ident<'a>(pub &'a str);

impl std::fmt::Debug for Ident<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
impl std::fmt::Display for Ident<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0.0)
    }
}
