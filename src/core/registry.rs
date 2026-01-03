use std::{collections::HashMap, hash::Hash};

pub struct Registry<Id, Def> {
    entries: HashMap<Id, Def>,
}

impl<Id, Def> Registry<Id, Def>
where
    Id: Hash + Eq + Copy,
{
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: Id, def: Def) {
        self.entries.insert(id, def);
    }

    pub fn get(&self, id: Id) -> Option<&Def> {
        self.entries.get(&id)
    }

    pub fn contains(&self, id: Id) -> bool {
        self.entries.contains_key(&id)
    }
}
