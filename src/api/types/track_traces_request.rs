pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TrackTracesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traces: Option<Vec<TraceRequest>>,
}

impl TrackTracesRequest {
    pub fn builder() -> TrackTracesRequestBuilder {
        <TrackTracesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TrackTracesRequestBuilder {
    traces: Option<Vec<TraceRequest>>,
}

impl TrackTracesRequestBuilder {
    pub fn traces(mut self, value: Vec<TraceRequest>) -> Self {
        self.traces = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TrackTracesRequest`].
    pub fn build(self) -> Result<TrackTracesRequest, BuildError> {
        Ok(TrackTracesRequest {
            traces: self.traces,
        })
    }
}
