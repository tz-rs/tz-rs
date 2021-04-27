pub enum Chain {
    Main,
    Other(String),
}

impl Chain {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Main => "main",
            Self::Other(chain_id) => chain_id,
        }
    }
}

pub enum Block {
    Head,
    Tail,
    Other(String),
}

impl Block {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Head => "head",
            Self::Tail => "tail",
            Self::Other(block_id) => block_id,
        }
    }
}
