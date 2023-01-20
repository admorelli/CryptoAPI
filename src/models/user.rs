#[derive(Debug, Serialize, Deserialize)]
pub struct User{
        pub id: String,
        ///secret used for authentication
        pub secret: String,
        ///salt to be used by the user child objects(Categories, Hashes and Storages)
        pub salt: String,
        ///set if the user is active
        pub active: bool,
        ///algorithmn that can be used to find a category
        pub alg: Vec<Alg>
}

