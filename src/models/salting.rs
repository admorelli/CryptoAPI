use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::util::EnumStringParse;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Default)]
pub struct ComplementSalting;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SaltingStrategy{
    Complement(ComplementSalting)
}

impl SaltingStrategy{
    pub fn apply(&self, key: &str, salt: &Vec<&str>) -> String{
        match self {
            SaltingStrategy::Complement(s) => s.apply(key, &salt)
        }
    }
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
trait ApplySalting{
    fn apply(&self, key: &str, salt: &Vec<&str>) -> String;
}

impl ApplySalting for ComplementSalting{
    fn apply(&self, key: &str, salt: &Vec<&str>) -> String {
        let mut salted: String = key.to_string();
        for  s in salt {
            salted = format!("{}:{}", salted.as_str(), s);
        }
        salted
    }
}