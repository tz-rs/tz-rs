pub enum ChainType {
    Main,
    Other(String),
}

impl ChainType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Main => "main",
            Self::Other(chain_id) => chain_id,
        }
        .to_string()
    }
}
