pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphRequest {
    #[serde(default)]
    pub start_event_id: String,
    pub direction: GraphRequestDirection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min_confidence: Option<f64>,
}

impl GraphRequest {
    pub fn builder() -> GraphRequestBuilder {
        <GraphRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraphRequestBuilder {
    start_event_id: Option<String>,
    direction: Option<GraphRequestDirection>,
    link_types: Option<Vec<String>>,
    max_depth: Option<i64>,
    min_confidence: Option<f64>,
}

impl GraphRequestBuilder {
    pub fn start_event_id(mut self, value: impl Into<String>) -> Self {
        self.start_event_id = Some(value.into());
        self
    }

    pub fn direction(mut self, value: GraphRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn link_types(mut self, value: Vec<String>) -> Self {
        self.link_types = Some(value);
        self
    }

    pub fn max_depth(mut self, value: i64) -> Self {
        self.max_depth = Some(value);
        self
    }

    pub fn min_confidence(mut self, value: f64) -> Self {
        self.min_confidence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraphRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_event_id`](GraphRequestBuilder::start_event_id)
    /// - [`direction`](GraphRequestBuilder::direction)
    pub fn build(self) -> Result<GraphRequest, BuildError> {
        Ok(GraphRequest {
            start_event_id: self
                .start_event_id
                .ok_or_else(|| BuildError::missing_field("start_event_id"))?,
            direction: self
                .direction
                .ok_or_else(|| BuildError::missing_field("direction"))?,
            link_types: self.link_types,
            max_depth: self.max_depth,
            min_confidence: self.min_confidence,
        })
    }
}
