pub trait Network: Copy + Clone + Send + Sync + 'static {
    const TYPE: &'static str;
}
