use std::fmt::{Debug, Display};

pub trait EnumStringParse
where
    Self: Sized + Debug,
{
    fn string_to_enum(key: String) -> Option<Self> {
        Self::str_to_enum(key.as_str())
    }
    fn str_to_enum(key: &str) -> Option<Self>;

    fn enum_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
pub struct Error(pub Option<String>);

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl std::error::Error for Error {}
