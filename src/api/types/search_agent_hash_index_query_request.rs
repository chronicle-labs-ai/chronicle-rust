pub use crate::prelude::*;

/// Query parameters for searchAgentHashIndex
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchAgentHashIndexQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// Comma-separated hash domains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<String>,
}

impl SearchAgentHashIndexQueryRequest {
    pub fn builder() -> SearchAgentHashIndexQueryRequestBuilder {
        <SearchAgentHashIndexQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchAgentHashIndexQueryRequestBuilder {
    q: Option<String>,
    domains: Option<String>,
}

impl SearchAgentHashIndexQueryRequestBuilder {
    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }

    pub fn domains(mut self, value: impl Into<String>) -> Self {
        self.domains = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchAgentHashIndexQueryRequest`].
    pub fn build(self) -> Result<SearchAgentHashIndexQueryRequest, BuildError> {
        Ok(SearchAgentHashIndexQueryRequest {
            q: self.q,
            domains: self.domains,
        })
    }
}
