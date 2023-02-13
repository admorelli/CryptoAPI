use super::salting::SaltingStrategy;

mod plain;
mod sha1;
mod sha2;
pub mod enumerator;

trait ApplyCripto {
    fn apply(&self, key: &str, salt: &Vec<&str>, salter: &SaltingStrategy) -> String;
}
