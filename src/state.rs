
use alloc::{collections::btree_map::BTreeMap, string::String};
use spin::Mutex;

#[derive(Clone, Debug)]
pub struct State {
    pub values: BTreeMap<String, String>,
}

// Global state stored in memory
static GLOBAL_STATE: Mutex<State> = Mutex::new(State { values: BTreeMap::new() });

impl State {
    
    pub fn new() -> Self {
        State { values: BTreeMap::new() }
    }

    pub fn save(&self) -> Result<(), &'static str> {
        *GLOBAL_STATE.lock() = self.clone();
        crate::println!("State saved in memory: values = {:?}", self.values);
        Ok(())
    }

    pub fn load() -> Result<Self, &'static str> {
        Ok((*GLOBAL_STATE.lock()).clone())
    }

    pub fn current() -> Self {
        (*GLOBAL_STATE.lock()).clone()
    }

    pub fn update(key: String, value: String) {
        let mut state = GLOBAL_STATE.lock();
        state.values.insert(key, value);
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }
}
