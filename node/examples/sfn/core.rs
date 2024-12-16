pub enum NodeType {
    Grid,
}

pub trait Network {
    const NAME: &'static str;
}
