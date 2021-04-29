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
