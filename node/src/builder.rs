#[derive(Clone, Debug)]
pub struct NodeBuilder {}

impl NodeBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait NodeBuilderConfig {}
