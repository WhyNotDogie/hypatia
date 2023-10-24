use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Fs<T> {
    pub nodes: BTreeMap<String, Node<T>>
}

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Node<T> {
    Folder(Fs<T>),
    File(T)
}