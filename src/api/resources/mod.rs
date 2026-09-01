//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **events**
//! - **timeline**
//! - **search**
//! - **discover**
//! - **links**
//! - **sdk**

use crate::{ApiError, ClientConfig};

pub mod discover;
pub mod events;
pub mod links;
pub mod sdk;
pub mod search;
pub mod timeline;
pub struct Chronicle {
    pub config: ClientConfig,
    pub events: EventsClient,
    pub timeline: TimelineClient,
    pub search: SearchClient,
    pub discover: DiscoverClient,
    pub links: LinksClient,
    pub sdk: SdkClient,
}

impl Chronicle {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            events: EventsClient::new(config.clone())?,
            timeline: TimelineClient::new(config.clone())?,
            search: SearchClient::new(config.clone())?,
            discover: DiscoverClient::new(config.clone())?,
            links: LinksClient::new(config.clone())?,
            sdk: SdkClient::new(config.clone())?,
        })
    }
}

pub use discover::DiscoverClient;
pub use events::EventsClient;
pub use links::LinksClient;
pub use sdk::SdkClient;
pub use search::SearchClient;
pub use timeline::TimelineClient;
