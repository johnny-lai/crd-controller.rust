/// State shared between the controller and the web server
#[derive(Clone, Default)]
pub struct State {
    /// Metrics registry
    registry: prometheus::Registry,
}

impl State {
    /// Metrics getter
    pub fn metrics(&self) -> Vec<prometheus::proto::MetricFamily> {
        self.registry.gather()
    }
}
