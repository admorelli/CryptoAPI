use crate::security::auth_key::{ApiKey};

#[get("/<category>")]
pub fn index(_key: ApiKey, category: &str) -> String{
    format!("Default '{}' route", category)
}

#[get("/<_category>/<key>")]
pub fn get(_key: ApiKey, _category: &str, key: &str) -> String{
    format!("Found item with key '{}'",key)
}

#[post("/<_category>/<key>")]
pub fn add(_key: ApiKey, _category: &str, key: &str) -> String{
    format!("Added item with key '{}'",key)
}

#[delete("/<_category>/<key>")]
pub fn del(_key: ApiKey, _category: &str, key: &str) -> String{
    format!("Deleted item with key '{}'",key)
}

#[patch("/<_category>/<key>")]
pub fn patch(_key: ApiKey, _category: &str, key: &str) -> String{
    format!("Changed item with key '{}'",key)
}