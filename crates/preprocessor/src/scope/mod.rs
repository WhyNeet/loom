use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

pub struct Scope {
    stack_frame: RefCell<HashSet<String>>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            stack_frame: RefCell::new(HashSet::new()),
        }
    }

    pub fn add_to_stack(&self, name: String) -> String {
        let mut remapped = name.clone();
        let mut i = 0;

        while self.stack_frame.borrow().contains(remapped.as_str()) {
            remapped = format!("{name}{i}");
            i += 1;
        }

        self.stack_frame.borrow_mut().insert(remapped.clone());

        remapped
    }
}

pub struct Remapper {
    remaps: RefCell<HashMap<String, String>>,
}

impl Remapper {
    pub fn new() -> Self {
        Self {
            remaps: RefCell::new(HashMap::new()),
        }
    }

    pub fn remap(&self, from: String, into: String) -> Option<String> {
        self.remaps.borrow_mut().insert(from, into)
    }

    pub fn get_remapped(&self, from: &str) -> Option<String> {
        self.remaps.borrow().get(from).map(String::clone)
    }
}
