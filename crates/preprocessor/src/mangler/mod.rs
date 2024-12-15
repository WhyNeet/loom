use std::{borrow::Cow, cell::RefCell, collections::HashMap};

pub struct Mangler {
    gen: RefCell<u64>,
    mangle_map: RefCell<HashMap<String, String>>,
}

impl Mangler {
    pub fn new() -> Self {
        Self {
            gen: RefCell::new(0),
            mangle_map: RefCell::new(HashMap::new()),
        }
    }

    pub fn submangler(&self) -> Self {
        Self {
            gen: self.gen.clone(),
            mangle_map: RefCell::new(HashMap::new()),
        }
    }

    pub fn mangle(&self, identifier: Cow<String>) -> String {
        if let Some(identifier) = self.mangle_map.borrow().get(identifier.as_str()) {
            return identifier.clone();
        }

        let new_ident = self.gen.borrow().to_string();

        *self.gen.borrow_mut() += 1;

        self.mangle_map
            .borrow_mut()
            .insert(identifier.to_string(), new_ident.clone());

        new_ident
    }

    pub fn forget(&self, identifier: &str) -> Option<String> {
        self.mangle_map.borrow_mut().remove(identifier)
    }

    pub fn rng(&self) -> String {
        *self.gen.borrow_mut() += 1;

        (*self.gen.borrow() - 1).to_string()
    }
}
