use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use anyhow::{Context, Result};
use async_channel::{unbounded, Receiver, Sender};
use futures::StreamExt;
use tokio::sync::RwLock;
use tracing::{info, warn};

use super::message::TicketMessage;

/// Number of seconds in a day
const DAY_SECS: u32 = 86_400;

#[derive(Debug)]
pub struct SeenPlate {
    pub plate: String,
    pub timestamp: u32,
    pub road: u16,
    pub mile: u16,
}

type ChannelPair<T> = (Sender<T>, Receiver<T>);

/// A collection of channels for each roads
pub struct RoadChannels<T> {
    inner: RwLock<HashMap<u16, ChannelPair<T>>>,
}

impl<T> RoadChannels<T> {
    pub async fn get(&self, road: u16) -> (Sender<T>, Receiver<T>) {
        if let Some(channels) = self.inner.read().await.get(&road) {
            return channels.clone();
        }

        (self.inner.write().await)
            .entry(road)
            .or_insert_with(unbounded)
            .clone()
    }
}

impl<T> Default for RoadChannels<T> {
    fn default() -> Self {
        Self {
            inner: RwLock::default(),
        }
    }
}

#[derive(Default)]
pub struct State {
    // Communication channels
    ticket_channels: RoadChannels<TicketMessage>,
    plate_channels: RoadChannels<SeenPlate>,

    /// Roads for which a daemon has already been spawned
    registered_roads: RwLock<HashSet<u16>>,

    /// Keep tracks of tickets sent to each cars
    days_for_car: RwLock<HashMap<String, HashSet<u32>>>,
}

impl State {
    pub async fn register_road(self: &Arc<Self>, road: u16, limit: u16) {
        // Non-blocking quick check for road already registered
        if self.registered_roads.read().await.contains(&road) {
            return;
        }

        // Mark the road as registered, early return if it has already been registered during the
        // lock acquire
        if !self.registered_roads.write().await.insert(road) {
            return;
        }

        // Spawn daemon to keep track of speed history
        let (_, plate_receiver) = self.plate_channels.get(road).await;
        let (ticket_sender, _) = self.ticket_channels.get(road).await;
        let state = self.clone();

        tokio::spawn(async move {
            road_daemon(state, road, limit, plate_receiver, ticket_sender)
                .await
                .expect("road daemon stopped unexpectedly")
        });
    }

    pub async fn get_ticket_receiver(&self, road: u16) -> Receiver<TicketMessage> {
        self.ticket_channels.get(road).await.1
    }

    pub async fn get_plate_sender(&self, road: u16) -> Sender<SeenPlate> {
        self.plate_channels.get(road).await.0
    }

    /// Specify that there should be a ticket for given days and returns true if no ticket was send
    /// for these days (and mark theses days as used).
    pub async fn new_ticket_for_days(
        &self,
        car: String,
        days: impl Clone + IntoIterator<Item = u32>,
    ) -> bool {
        let already_sent_for_days = |days_for_car: &HashSet<_>| {
            days.clone()
                .into_iter()
                .any(|day| days_for_car.contains(&day))
        };

        // Check if a ticket can be send without acquiring a write access
        if let Some(days_for_car) = self.days_for_car.read().await.get(&car) {
            if already_sent_for_days(days_for_car) {
                return false;
            }
        }

        // Get write access to the entry
        let mut days_for_car = self.days_for_car.write().await;
        let days_for_car = days_for_car.entry(car).or_default();

        // Check if no days was added while acquiring the write access
        if already_sent_for_days(days_for_car) {
            return false;
        }

        days_for_car.extend(days);
        true
    }
}

/// Deamon that is spawned for each known road to keep track of speed infractions on that road.
#[tracing::instrument(skip(state, plate_receiver, ticket_sender))]
pub async fn road_daemon(
    state: Arc<State>,
    road: u16,
    limit: u16,
    mut plate_receiver: Receiver<SeenPlate>,
    ticket_sender: Sender<TicketMessage>,
) -> Result<()> {
    info!("Initializing");
    let limit: i64 = limit.into();
    let mut prev_speeds: HashMap<String, Vec<(u16, u32)>> = HashMap::new();

    while let Some(plate) = plate_receiver.next().await {
        let prevs = prev_speeds.entry(plate.plate.clone()).or_default();

        for &mut (prev_mile, prev_ts) in prevs.iter_mut() {
            // Check if 3600 * (prev_mile - plate.mile) / (prev_ts - plate.ts) > limit
            let mut x_mile: i64 = prev_mile.into();
            let mut x_ts: i64 = prev_ts.into();
            let mut y_mile: i64 = plate.mile.into();
            let mut y_ts: i64 = plate.timestamp.into();

            match x_ts.cmp(&y_ts) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => {
                    warn!(
                        plate = &plate.plate,
                        ts = x_ts,
                        "Received plate for the same timestamp twice",
                    );

                    continue;
                }
                std::cmp::Ordering::Greater => {
                    // Ensures that y_ts > x_ts to allow reverting fraction
                    std::mem::swap(&mut x_ts, &mut y_ts);
                    std::mem::swap(&mut x_mile, &mut y_mile);
                }
            }

            if 3600 * (y_mile - x_mile).abs() > limit * (y_ts - x_ts) {
                let min_day = x_ts as u32 / DAY_SECS;
                let max_day = y_ts as u32 / DAY_SECS;

                if state
                    .new_ticket_for_days(plate.plate.clone(), min_day..=max_day)
                    .await
                {
                    let speed = ((100 * 3600 * (y_mile - x_mile).abs()) / (y_ts - x_ts)) as u16;

                    let ticket = TicketMessage {
                        plate: plate.plate.clone(),
                        road,
                        mile1: x_mile as _,
                        timestamp1: x_ts as _,
                        mile2: y_mile as _,
                        timestamp2: y_ts as _,
                        speed,
                    };

                    ticket_sender
                        .send(ticket)
                        .await
                        .context("ticket channel closed unexpectedly")?;
                }
            }
        }

        prevs.push((plate.mile, plate.timestamp));
    }

    Ok(())
}
