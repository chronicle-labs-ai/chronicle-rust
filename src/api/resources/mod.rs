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
//! - **agents**
//! - **datasets**
//! - **environments**
//! - **backtests**
//! - **credentials**

use crate::{ApiError, ClientConfig};

pub mod agents;
pub mod backtests;
pub mod credentials;
pub mod datasets;
pub mod discover;
pub mod environments;
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
    pub agents: AgentsClient,
    pub datasets: DatasetsClient,
    pub environments: EnvironmentsClient,
    pub backtests: BacktestsClient,
    pub credentials: CredentialsClient,
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
            agents: AgentsClient::new(config.clone())?,
            datasets: DatasetsClient::new(config.clone())?,
            environments: EnvironmentsClient::new(config.clone())?,
            backtests: BacktestsClient::new(config.clone())?,
            credentials: CredentialsClient::new(config.clone())?,
        })
    }
}

pub use agents::AgentsClient;
pub use backtests::BacktestsClient;
pub use credentials::CredentialsClient;
pub use datasets::DatasetsClient;
pub use discover::DiscoverClient;
pub use environments::EnvironmentsClient;
pub use events::EventsClient;
pub use links::LinksClient;
pub use sdk::SdkClient;
pub use search::SearchClient;
pub use timeline::TimelineClient;
