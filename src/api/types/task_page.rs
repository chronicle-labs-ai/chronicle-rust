pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskPage {
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub items: Vec<TaskPageItemsItem>,
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl TaskPage {
    pub fn builder() -> TaskPageBuilder {
        <TaskPageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskPageBuilder {
    has_more: Option<bool>,
    items: Option<Vec<TaskPageItemsItem>>,
    next_cursor: Option<String>,
}

impl TaskPageBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<TaskPageItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskPage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_more`](TaskPageBuilder::has_more)
    /// - [`items`](TaskPageBuilder::items)
    pub fn build(self) -> Result<TaskPage, BuildError> {
        Ok(TaskPage {
            has_more: self
                .has_more
                .ok_or_else(|| BuildError::missing_field("has_more"))?,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            next_cursor: self.next_cursor,
        })
    }
}
