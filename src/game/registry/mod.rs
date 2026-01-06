use std::collections::HashMap;

pub mod block_registry;

pub struct Registry<Def> {
    ids: HashMap<&'static str, usize>,
    entries: Vec<Def>,
}

impl<Def> Registry<Def> {
    pub fn new() -> Self {
        Self {
            ids: HashMap::new(),
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, def: Def, name: &'static str) -> usize {
        let id = self.entries.len();
        self.entries.push(def);
        self.ids.insert(name, id);
        id
    }

    pub fn get(&self, id: usize) -> Option<&Def> {
        self.entries.get(id)
    }

    pub fn by_name(&self, name: &str) -> Option<&Def> {
        self.ids.get(name).and_then(|&id| self.entries.get(id))
    }

    pub fn id_by_name(&self, name: &str) -> usize {
        self.ids
            .get(name)
            .copied()
            .unwrap_or_else(|| panic!("Unknown Block {:?}", name))
    }

    #[allow(dead_code)]
    pub fn contains(&self, id: usize) -> bool {
        self.entries.get(id).is_some()
    }
}
