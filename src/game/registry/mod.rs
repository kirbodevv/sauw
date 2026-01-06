pub mod block_registry;

pub struct Registry<Def> {
    entries: Vec<Def>,
}

impl<Def> Registry<Def> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, def: Def) -> usize {
        let id = self.entries.len();
        self.entries.push(def);
        id
    }

    pub fn get(&self, id: usize) -> Option<&Def> {
        self.entries.get(id)
    }

    #[allow(dead_code)]
    pub fn contains(&self, id: usize) -> bool {
        self.entries.get(id).is_some()
    }
}
