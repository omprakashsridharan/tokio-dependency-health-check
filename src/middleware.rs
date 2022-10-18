use std::time::Duration;

use futures::future::join_all;
use tokio::{task, time};

#[async_trait::async_trait]
pub trait ActiveDependency: Send + Sync {
    async fn is_active(&self) -> bool;
}

pub struct MonitorConfig {
    pub duration: Duration,
    pub dependencies: Vec<Box<dyn ActiveDependency>>,
}

pub async fn monitor_dependencies(monitor_config: MonitorConfig) {
    let forever = task::spawn(async move {
        let mut interval = time::interval(monitor_config.duration);

        loop {
            interval.tick().await;
            let checks: Vec<_> = monitor_config
                .dependencies
                .iter()
                .map(|x| x.is_active())
                .collect();
            let result = join_all(checks).await;
            println!("result {:?}", result);
        }
    });

    forever.await.expect("Error while monitoring");
}
