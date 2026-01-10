pub trait MonitorProviderPort {
    fn get_monitors(&self) -> anyhow::Result<Vec<String>>;
}
