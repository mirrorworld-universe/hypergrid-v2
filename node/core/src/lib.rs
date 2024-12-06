pub trait Network: Send + Sync + 'static {
    const TYPE: &'static str;
}
