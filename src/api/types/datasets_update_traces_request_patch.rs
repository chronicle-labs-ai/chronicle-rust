pub use crate::prelude::*;

/// Patch to apply to one or more memberships. Nullable annotations preserve the same three states as [`PatchField`]: explicit JSON `null` clears while omission is a no-op.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTracesRequestPatch {
    /// New cluster id, or `null` to drop the trace from any cluster.
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// Replace the membership note, or `null` to clear it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// New split, or `null` to mark unassigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split: Option<UpdateTracesRequestPatchSplit>,
    /// Health status of a trace as judged by the dataset owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateTracesRequestPatchStatus>,
}

impl UpdateTracesRequestPatch {
    pub fn builder() -> UpdateTracesRequestPatchBuilder {
        <UpdateTracesRequestPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTracesRequestPatchBuilder {
    cluster_id: Option<String>,
    note: Option<String>,
    split: Option<UpdateTracesRequestPatchSplit>,
    status: Option<UpdateTracesRequestPatchStatus>,
}

impl UpdateTracesRequestPatchBuilder {
    pub fn cluster_id(mut self, value: impl Into<String>) -> Self {
        self.cluster_id = Some(value.into());
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn split(mut self, value: UpdateTracesRequestPatchSplit) -> Self {
        self.split = Some(value);
        self
    }

    pub fn status(mut self, value: UpdateTracesRequestPatchStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateTracesRequestPatch`].
    pub fn build(self) -> Result<UpdateTracesRequestPatch, BuildError> {
        Ok(UpdateTracesRequestPatch {
            cluster_id: self.cluster_id,
            note: self.note,
            split: self.split,
            status: self.status,
        })
    }
}
