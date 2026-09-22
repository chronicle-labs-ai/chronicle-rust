pub use crate::prelude::*;

/// A paged Dataset catalog response. Cursor query parameters remain owned by the HTTP layer; this response is shared by every first-party client.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskSuitePage {
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub items: Vec<TaskSuitePageItemsItem>,
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl TaskSuitePage {
    pub fn builder() -> TaskSuitePageBuilder {
        <TaskSuitePageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskSuitePageBuilder {
    has_more: Option<bool>,
    items: Option<Vec<TaskSuitePageItemsItem>>,
    next_cursor: Option<String>,
}

impl TaskSuitePageBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<TaskSuitePageItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskSuitePage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_more`](TaskSuitePageBuilder::has_more)
    /// - [`items`](TaskSuitePageBuilder::items)
    pub fn build(self) -> Result<TaskSuitePage, BuildError> {
        Ok(TaskSuitePage {
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
