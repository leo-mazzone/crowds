use std::collections::HashMap;
use crate::k_anonimity::generalisation::RuleMap;

pub enum Suitability {
    Suitable,
    Unsuitable,
    Unknown,
}

pub struct Lattice {
    pub rules: RuleMap,
}

pub struct Node {
    pub lattice: Lattice,
    pub children: Vec<i32>,
    pub parents: Vec<i32>,
    pub suitability: Suitability,
    pub gen_state: HashMap<String, u8>,
}