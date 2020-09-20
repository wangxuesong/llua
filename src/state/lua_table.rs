use crate::state::LuaValue;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct LuaTable {
    array: Vec<LuaValue>,
    map: HashMap<LuaValue, LuaValue>,
}

impl PartialEq for LuaTable {
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array
    }
}

impl Hash for LuaTable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.array.hash(state)
    }
}

impl LuaTable {
    pub fn new(array_size: usize, hash_size: usize) -> LuaTable {
        LuaTable {
            array: vec![LuaValue::Nil; array_size + 1],
            map: HashMap::with_capacity(hash_size),
        }
    }

    pub fn len(&self) -> usize {
        self.array.len() - 1
    }

    pub fn get_array(&self, index: isize) -> LuaValue {
        self.array[index as usize].clone()
    }

    pub fn set_array(&mut self, index: isize, value: LuaValue) {
        self.array[index as usize] = value
    }

    pub fn get_hash(&self, key: LuaValue) -> LuaValue {
        if let Some(value) = self.map.get(&key) {
            return value.clone();
        }
        LuaValue::Nil
    }

    pub fn get(&self, key: LuaValue) -> LuaValue {
        match key {
            LuaValue::Nil => LuaValue::Nil,
            LuaValue::Integer(i) => self.get_array(i as isize),
            _ => self.get_hash(key),
        }
    }

    pub fn set_hash(&mut self, key: LuaValue, value: LuaValue) {
        self.map.insert(key, value);
    }
}
