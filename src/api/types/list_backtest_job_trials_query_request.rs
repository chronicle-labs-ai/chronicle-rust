pub use crate::prelude::*;

/// Query parameters for listBacktestJobTrials
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBacktestJobTrialsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl ListBacktestJobTrialsQueryRequest {
    pub fn builder() -> ListBacktestJobTrialsQueryRequestBuilder {
        <ListBacktestJobTrialsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBacktestJobTrialsQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
}

impl ListBacktestJobTrialsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBacktestJobTrialsQueryRequest`].
    pub fn build(self) -> Result<ListBacktestJobTrialsQueryRequest, BuildError> {
        Ok(ListBacktestJobTrialsQueryRequest {
            limit: self.limit,
            offset: self.offset,
        })
    }
}
