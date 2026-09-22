pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAgentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
}

impl UpdateAgentRequest {
    pub fn builder() -> UpdateAgentRequestBuilder {
        <UpdateAgentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAgentRequestBuilder {
    description: Option<String>,
    environment: Option<String>,
    owner: Option<String>,
    purpose: Option<String>,
}

impl UpdateAgentRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn owner(mut self, value: impl Into<String>) -> Self {
        self.owner = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: impl Into<String>) -> Self {
        self.purpose = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAgentRequest`].
    pub fn build(self) -> Result<UpdateAgentRequest, BuildError> {
        Ok(UpdateAgentRequest {
            description: self.description,
            environment: self.environment,
            owner: self.owner,
            purpose: self.purpose,
        })
    }
}
