pub enum Chain {
    Main,
    Test,
    Other(String),
}

impl Chain {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Main => "main",
            Self::Test => "test",
            Self::Other(chain_id) => chain_id,
        }
    }
}

pub enum Block {
    Head,
    Genesis,
    Other(String),
}

impl Block {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Head => "head",
            Self::Genesis => "genesis",
            Self::Other(block_id) => block_id,
        }
    }
}
