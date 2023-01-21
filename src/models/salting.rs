use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::util::EnumStringParse;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Default)]
pub struct ComplementSalting;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SaltingStrategy{
    Complement(ComplementSalting)
}

impl Default for SaltingStrategy{
    fn default() -> Self{
        SaltingStrategy::Complement(ComplementSalting::default())
    }
}


impl EnumStringParse for SaltingStrategy{
    fn str_to_enum(key: &str) -> Option<Self>{
        match key {
            "complement" => Some(SaltingStrategy::Complement(ComplementSalting::default())),
            _ => Option::None
        }
    }
}