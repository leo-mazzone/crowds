use std::collections::HashMap;

pub type RuleMap = HashMap<String, GenRule>;

pub struct GenRule {
    pub max_level: u8,
}