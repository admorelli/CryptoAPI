use super::categoria::Categoria;
pub struct Hash{
    //calculated by the key of a model(not saved) + user salt + category salt +its own salt
    //calculated by one of the algmodels associated to the category
    pub id: String,
    //define if it is safe to use
    pub is_unsafe: bool,
    //salt used fo calculate its own key and its children(Hash and Storage)
    pub salt: String,
    ///Category owner of the Hash record
    pub owner: Vec<Categoria>
}

