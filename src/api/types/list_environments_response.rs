pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListEnvironmentsResponse {
    #[serde(default)]
    pub environments: Vec<ListEnvironmentsResponseEnvironmentsItem>,
}

impl ListEnvironmentsResponse {
    pub fn builder() -> ListEnvironmentsResponseBuilder {
        <ListEnvironmentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEnvironmentsResponseBuilder {
    environments: Option<Vec<ListEnvironmentsResponseEnvironmentsItem>>,
}

impl ListEnvironmentsResponseBuilder {
    pub fn environments(mut self, value: Vec<ListEnvironmentsResponseEnvironmentsItem>) -> Self {
        self.environments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEnvironmentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environments`](ListEnvironmentsResponseBuilder::environments)
    pub fn build(self) -> Result<ListEnvironmentsResponse, BuildError> {
        Ok(ListEnvironmentsResponse {
            environments: self
                .environments
                .ok_or_else(|| BuildError::missing_field("environments"))?,
        })
    }
}
