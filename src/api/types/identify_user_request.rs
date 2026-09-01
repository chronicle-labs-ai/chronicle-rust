pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IdentifyUserRequest {
    #[serde(default)]
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<HashMap<String, serde_json::Value>>,
}

impl IdentifyUserRequest {
    pub fn builder() -> IdentifyUserRequestBuilder {
        <IdentifyUserRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IdentifyUserRequestBuilder {
    user_id: Option<String>,
    traits: Option<HashMap<String, serde_json::Value>>,
}

impl IdentifyUserRequestBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn traits(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.traits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IdentifyUserRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](IdentifyUserRequestBuilder::user_id)
    pub fn build(self) -> Result<IdentifyUserRequest, BuildError> {
        Ok(IdentifyUserRequest {
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            traits: self.traits,
        })
    }
}
