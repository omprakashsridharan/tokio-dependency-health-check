use std::time::Duration;
use tokio_dependency_health_check::middleware::{
    monitor_dependencies, ActiveDependency, MonitorConfig,
};

struct ActiveConnection;

#[async_trait::async_trait]
impl ActiveDependency for ActiveConnection {
    async fn is_active(&self) -> bool {
        true
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let active_connection = ActiveConnection;
    let monitor_config = MonitorConfig {
        duration: Duration::from_secs(5),
        dependencies: vec![Box::new(active_connection)],
    };
    monitor_dependencies(monitor_config).await;
    Ok(())
}
