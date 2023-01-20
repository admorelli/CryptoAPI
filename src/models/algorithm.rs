use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlainTextAlgorithmn;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ComplementSalting;


#[derive(Debug, Serialize, Deserialize)]
pub enum CryptoAlgorithm{
    Plain(PlainTextAlgorithmn)
}

impl Default for CryptoAlgorithm{
    fn default() -> Self{
        CryptoAlgorithm::Plain(PlainTextAlgorithmn::default())
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum SaltingStrategy{
    Complement(ComplementSalting)
}

impl Default for SaltingStrategy{
    fn default() -> Self{
        SaltingStrategy::Complement(ComplementSalting::default())
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Alg{
    pub id: Option<String>,
    pub algorithm: CryptoAlgorithm,
    pub salting: SaltingStrategy
}