use alloc::borrow::ToOwned;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use lolid::Uuid;


#[derive(Default, Debug)]
pub struct File {
    metadata: BTreeMap<String, String>,
}

impl Index<&str> for File {
    type Output = String;

    fn index(&self, index: &str) -> &Self::Output {
        &self.metadata[&index.to_owned()]
    }
}

impl IndexMut<&str> for File {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.metadata.entry(index.to_owned()).or_default()
    }
}


#[derive(Default, Debug)]
pub struct FileSystem {
    items: BTreeMap<Uuid, File>,
}

impl FileSystem {
    pub fn spawn(&mut self) -> &mut File {
        self.items.entry(Uuid::v4_prng()).or_default()
    }

    pub fn lookup(&self, name: &str) -> Option<&File> {
        for (_, v) in &self.items {
            if v["name"] == name {
                return Some(v)
            }
        }

        None
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut File> {
        for (_, v) in &mut self.items {
            if v["name"] == name {
                return Some(v)
            }
        }

        None
    }
}