pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskEventPage {
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub items: Vec<TaskEventPageItemsItem>,
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl TaskEventPage {
    pub fn builder() -> TaskEventPageBuilder {
        <TaskEventPageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskEventPageBuilder {
    has_more: Option<bool>,
    items: Option<Vec<TaskEventPageItemsItem>>,
    next_cursor: Option<String>,
}

impl TaskEventPageBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<TaskEventPageItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskEventPage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_more`](TaskEventPageBuilder::has_more)
    /// - [`items`](TaskEventPageBuilder::items)
    pub fn build(self) -> Result<TaskEventPage, BuildError> {
        Ok(TaskEventPage {
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
