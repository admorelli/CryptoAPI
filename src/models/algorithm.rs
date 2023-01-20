pub struct PlainTextAlgorithmn;
pub struct ComplementSalting;
pub enum CryptoAlgorithm{
    Plain(PlainTextAlgorithmn)
}

pub enum SaltingStrategy{
    Complement(ComplementSalting)
}
pub struct Alg{
    pub id: Option<String>,
    pub algorithm: CryptoAlgorithm,
    pub salting: SaltingStrategy
}