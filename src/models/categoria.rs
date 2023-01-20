pub struct Categoria{
    pub id: String,
    pub is_unsafe: bool,
    pub salt: String,
    pub alg: Vec<Alg>,
    pub owner: User
}