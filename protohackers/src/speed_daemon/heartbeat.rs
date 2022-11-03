use std::time::Duration;

use anyhow::anyhow;
use tokio::time::{Instant, Interval, MissedTickBehavior};
use tracing::info;

use super::error::{Error, Result};

#[derive(Default)]
pub struct Hearbeat {
    inner: Option<Interval>,
}

impl Hearbeat {
    pub async fn tick(&mut self) -> Instant {
        if let Some(interval) = &mut self.inner {
            interval.tick().await
        } else {
            futures::future::pending().await
        }
    }

    pub fn set(&mut self, delay: u32) -> Result<()> {
        if self.inner.is_some() {
            return Err(Error::Client(anyhow!(
                "Trying to replace existing heartbeat"
            )));
        } else if delay == 0 {
            return Ok(());
        }

        info!("Setting heartbeat to {:.1}s", (delay as f64) / 10.);
        let mut interval = tokio::time::interval(Duration::from_millis(100 * u64::from(delay)));
        interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
        interval.reset();
        self.inner = Some(interval);
        Ok(())
    }
}
