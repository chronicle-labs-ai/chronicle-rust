pub use crate::prelude::*;

/// One explicit case accepted by `POST /v1/backtests/jobs` for non-Dataset recipes. Dataset-backed requests use these entries only as a case-id selection; the server always loads the instruction and expected outcome from the pinned Dataset Version.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBacktestJobRequestCasesItem {
    #[serde(rename = "caseCluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_cluster: Option<String>,
    #[serde(rename = "caseId")]
    #[serde(default)]
    pub case_id: String,
    #[serde(rename = "expectedOutcome")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_outcome: Option<String>,
    /// Required for composed recipes. Dataset-backed requests may omit it because the server loads canonical content from the pinned Version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
}

impl CreateBacktestJobRequestCasesItem {
    pub fn builder() -> CreateBacktestJobRequestCasesItemBuilder {
        <CreateBacktestJobRequestCasesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBacktestJobRequestCasesItemBuilder {
    case_cluster: Option<String>,
    case_id: Option<String>,
    expected_outcome: Option<String>,
    instruction: Option<String>,
}

impl CreateBacktestJobRequestCasesItemBuilder {
    pub fn case_cluster(mut self, value: impl Into<String>) -> Self {
        self.case_cluster = Some(value.into());
        self
    }

    pub fn case_id(mut self, value: impl Into<String>) -> Self {
        self.case_id = Some(value.into());
        self
    }

    pub fn expected_outcome(mut self, value: impl Into<String>) -> Self {
        self.expected_outcome = Some(value.into());
        self
    }

    pub fn instruction(mut self, value: impl Into<String>) -> Self {
        self.instruction = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBacktestJobRequestCasesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`case_id`](CreateBacktestJobRequestCasesItemBuilder::case_id)
    pub fn build(self) -> Result<CreateBacktestJobRequestCasesItem, BuildError> {
        Ok(CreateBacktestJobRequestCasesItem {
            case_cluster: self.case_cluster,
            case_id: self
                .case_id
                .ok_or_else(|| BuildError::missing_field("case_id"))?,
            expected_outcome: self.expected_outcome,
            instruction: self.instruction,
        })
    }
}
