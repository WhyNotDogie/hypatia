//! This crate consists of:
//! - Server/Client messages passed over the network
//! - Commonly used helper functions

pub mod fs;

use std::{num::NonZeroU16, collections::{HashMap, BTreeMap}};

use serde::{Serialize, Deserialize};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Module {
    pub functions: Vec<LuaItem>
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct LuaItem {
    name: String
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum LuaItemType {
    Function {
        args: Vec<LuaItemType>,
        return_value: Box<LuaItemType>
    },
    String(String),
    Number(ordered_float::NotNan<f64>),
    Bool(bool),
    Table(BTreeMap<LuaItem, LuaItemType>)
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Error {
    pub code: NonZeroU16,
    pub reason: String
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct FindGame {
    /// Id of source game
    pub game: u128
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct FoundGame {
    /// Id of running game
    pub id: u128
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct GameInfo {
    pub id: u128,
    pub max_players: NonZeroU16,
    pub min_players: NonZeroU16,
    pub items: Vec<LuaItem>,
    pub lua: fs::Fs<String>
}