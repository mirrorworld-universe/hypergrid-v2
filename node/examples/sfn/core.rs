pub enum NodeType {
    Grid,
}

pub trait Network {
    const NAME: &'static str;
}

pub struct Canary {}
impl Network for Canary {
    const NAME: &'static str = "Canary";
}
