use std::hash::{BuildHasher, Hasher};
use std::hash::Hash;

#[derive(Debug, Copy, Clone, Eq)]
pub struct Entity {
    id: usize
}

impl Entity {
    pub fn new(id: usize) -> Entity { Entity { id: id } }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Entity) -> bool {
        self.id == other.id
    }
}

impl Hash for Entity {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        state.write_usize(self.id);
    }
}


pub trait CreatesEntities {
    fn new_entity(&mut self) -> Entity;
}

pub struct EntityFactory {
    id: usize
}

impl EntityFactory {
    pub fn new() -> EntityFactory { EntityFactory{ id : 0 } }
}

impl Default for EntityFactory {
    fn default() -> EntityFactory { EntityFactory::new() }
}

impl CreatesEntities for EntityFactory {
    fn new_entity(&mut self) -> Entity {
        self.id += 1;
        Entity::new(self.id)
    }
}



pub struct EntityHashState;

impl BuildHasher for EntityHashState {
    type Hasher = EntityHasher;
    fn build_hasher(&self) -> EntityHasher {
        EntityHasher { id: 0 }
    }
}



pub struct EntityHasher {
    id: usize
}

impl Hasher for EntityHasher {
    fn finish(&self) -> u64 {
        self.id as u64
    }

    fn write_usize(&mut self, i: usize) {
        self.id = i;
    }

    fn write(&mut self, _: &[u8]) {}
    fn write_u8(&mut self, _: u8) {}
    fn write_u16(&mut self, _: u16) {}
    fn write_u32(&mut self, _: u32) {}
    fn write_u64(&mut self, _: u64) {}
    fn write_i8(&mut self, _: i8) {}
    fn write_i16(&mut self, _: i16) {}
    fn write_i32(&mut self, _: i32) {}
    fn write_i64(&mut self, _: i64) {}
    fn write_isize(&mut self, _: isize) {}
}
