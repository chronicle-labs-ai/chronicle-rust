pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTracesRequest {
    /// Patch to apply to one or more memberships. Nullable annotations preserve the same three states as [`PatchField`]: explicit JSON `null` clears while omission is a no-op.
    #[serde(default)]
    pub patch: UpdateTracesRequestPatch,
    #[serde(rename = "traceIds")]
    #[serde(default)]
    pub trace_ids: Vec<String>,
}

impl UpdateTracesRequest {
    pub fn builder() -> UpdateTracesRequestBuilder {
        <UpdateTracesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTracesRequestBuilder {
    patch: Option<UpdateTracesRequestPatch>,
    trace_ids: Option<Vec<String>>,
}

impl UpdateTracesRequestBuilder {
    pub fn patch(mut self, value: UpdateTracesRequestPatch) -> Self {
        self.patch = Some(value);
        self
    }

    pub fn trace_ids(mut self, value: Vec<String>) -> Self {
        self.trace_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateTracesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`patch`](UpdateTracesRequestBuilder::patch)
    /// - [`trace_ids`](UpdateTracesRequestBuilder::trace_ids)
    pub fn build(self) -> Result<UpdateTracesRequest, BuildError> {
        Ok(UpdateTracesRequest {
            patch: self
                .patch
                .ok_or_else(|| BuildError::missing_field("patch"))?,
            trace_ids: self
                .trace_ids
                .ok_or_else(|| BuildError::missing_field("trace_ids"))?,
        })
    }
}
