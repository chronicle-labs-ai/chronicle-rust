pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TrackSignalsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signals: Option<Vec<SignalRequest>>,
}

impl TrackSignalsRequest {
    pub fn builder() -> TrackSignalsRequestBuilder {
        <TrackSignalsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TrackSignalsRequestBuilder {
    signals: Option<Vec<SignalRequest>>,
}

impl TrackSignalsRequestBuilder {
    pub fn signals(mut self, value: Vec<SignalRequest>) -> Self {
        self.signals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TrackSignalsRequest`].
    pub fn build(self) -> Result<TrackSignalsRequest, BuildError> {
        Ok(TrackSignalsRequest {
            signals: self.signals,
        })
    }
}
