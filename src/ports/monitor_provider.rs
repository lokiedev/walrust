pub trait MonitorProviderPort {
    fn get_monitors() -> anyhow::Result<Vec<String>>;
}
