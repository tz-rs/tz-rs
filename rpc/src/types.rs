pub enum ChainType {
    Main,
    Other(String),
}

impl ChainType {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Main => "main",
            Self::Other(chain_id) => chain_id,
        }
    }
}
