pub use crate::prelude::*;

/// One stable, ascending page of persisted trial rows and only the rewards attached to those rows.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListBacktestJobTrialsResponse {
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    #[serde(rename = "nextOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<i64>,
    /// Outer key = trial id; inner key = reward name.
    #[serde(default)]
    pub rewards: HashMap<String, HashMap<String, f64>>,
    #[serde(default)]
    pub trials: Vec<ListBacktestJobTrialsResponseTrialsItem>,
}

impl ListBacktestJobTrialsResponse {
    pub fn builder() -> ListBacktestJobTrialsResponseBuilder {
        <ListBacktestJobTrialsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBacktestJobTrialsResponseBuilder {
    has_more: Option<bool>,
    next_offset: Option<i64>,
    rewards: Option<HashMap<String, HashMap<String, f64>>>,
    trials: Option<Vec<ListBacktestJobTrialsResponseTrialsItem>>,
}

impl ListBacktestJobTrialsResponseBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn next_offset(mut self, value: i64) -> Self {
        self.next_offset = Some(value);
        self
    }

    pub fn rewards(mut self, value: HashMap<String, HashMap<String, f64>>) -> Self {
        self.rewards = Some(value);
        self
    }

    pub fn trials(mut self, value: Vec<ListBacktestJobTrialsResponseTrialsItem>) -> Self {
        self.trials = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBacktestJobTrialsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_more`](ListBacktestJobTrialsResponseBuilder::has_more)
    /// - [`rewards`](ListBacktestJobTrialsResponseBuilder::rewards)
    /// - [`trials`](ListBacktestJobTrialsResponseBuilder::trials)
    pub fn build(self) -> Result<ListBacktestJobTrialsResponse, BuildError> {
        Ok(ListBacktestJobTrialsResponse {
            has_more: self
                .has_more
                .ok_or_else(|| BuildError::missing_field("has_more"))?,
            next_offset: self.next_offset,
            rewards: self
                .rewards
                .ok_or_else(|| BuildError::missing_field("rewards"))?,
            trials: self
                .trials
                .ok_or_else(|| BuildError::missing_field("trials"))?,
        })
    }
}
