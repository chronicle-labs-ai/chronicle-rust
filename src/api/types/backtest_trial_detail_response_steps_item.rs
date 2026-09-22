pub use crate::prelude::*;

/// Row projection of `"BacktestTrialStep"`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BacktestTrialDetailResponseStepsItem {
    /// Who performed a step; one timeline lane per actor.
    pub actor: BacktestTrialDetailResponseStepsItemActor,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// `None` for instantaneous steps (seed events, state changes).
    #[serde(rename = "endedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<DateTime<FixedOffset>>,
    /// Frozen verifier id (or recipe grader id) for `grade` steps.
    #[serde(rename = "graderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grader_id: Option<String>,
    #[serde(default)]
    pub id: String,
    /// What a step is. Drives the glyph on the timeline and which detail renderer opens when the span is clicked.
    pub kind: BacktestTrialDetailResponseStepsItemKind,
    /// Position within the trial; steps are returned ordered by it.
    #[serde(default)]
    pub ordinal: i64,
    /// Free-form detail: `request`/`response` for tool calls, `text` for messages, `reasoning` for judge grades, and so on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    /// Grader score in `[0, 1]` for `grade` steps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub started_at: DateTime<FixedOffset>,
    /// Outcome colouring for a step. `None` renders neutral.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BacktestTrialDetailResponseStepsItemStatus>,
    #[serde(default)]
    pub title: String,
    #[serde(rename = "trialId")]
    #[serde(default)]
    pub trial_id: String,
}

impl BacktestTrialDetailResponseStepsItem {
    pub fn builder() -> BacktestTrialDetailResponseStepsItemBuilder {
        <BacktestTrialDetailResponseStepsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BacktestTrialDetailResponseStepsItemBuilder {
    actor: Option<BacktestTrialDetailResponseStepsItemActor>,
    created_at: Option<DateTime<FixedOffset>>,
    ended_at: Option<DateTime<FixedOffset>>,
    grader_id: Option<String>,
    id: Option<String>,
    kind: Option<BacktestTrialDetailResponseStepsItemKind>,
    ordinal: Option<i64>,
    payload: Option<serde_json::Value>,
    score: Option<f64>,
    started_at: Option<DateTime<FixedOffset>>,
    status: Option<BacktestTrialDetailResponseStepsItemStatus>,
    title: Option<String>,
    trial_id: Option<String>,
}

impl BacktestTrialDetailResponseStepsItemBuilder {
    pub fn actor(mut self, value: BacktestTrialDetailResponseStepsItemActor) -> Self {
        self.actor = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn ended_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.ended_at = Some(value);
        self
    }

    pub fn grader_id(mut self, value: impl Into<String>) -> Self {
        self.grader_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: BacktestTrialDetailResponseStepsItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn ordinal(mut self, value: i64) -> Self {
        self.ordinal = Some(value);
        self
    }

    pub fn payload(mut self, value: serde_json::Value) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: BacktestTrialDetailResponseStepsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn trial_id(mut self, value: impl Into<String>) -> Self {
        self.trial_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BacktestTrialDetailResponseStepsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actor`](BacktestTrialDetailResponseStepsItemBuilder::actor)
    /// - [`created_at`](BacktestTrialDetailResponseStepsItemBuilder::created_at)
    /// - [`id`](BacktestTrialDetailResponseStepsItemBuilder::id)
    /// - [`kind`](BacktestTrialDetailResponseStepsItemBuilder::kind)
    /// - [`ordinal`](BacktestTrialDetailResponseStepsItemBuilder::ordinal)
    /// - [`started_at`](BacktestTrialDetailResponseStepsItemBuilder::started_at)
    /// - [`title`](BacktestTrialDetailResponseStepsItemBuilder::title)
    /// - [`trial_id`](BacktestTrialDetailResponseStepsItemBuilder::trial_id)
    pub fn build(self) -> Result<BacktestTrialDetailResponseStepsItem, BuildError> {
        Ok(BacktestTrialDetailResponseStepsItem {
            actor: self
                .actor
                .ok_or_else(|| BuildError::missing_field("actor"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            ended_at: self.ended_at,
            grader_id: self.grader_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            ordinal: self
                .ordinal
                .ok_or_else(|| BuildError::missing_field("ordinal"))?,
            payload: self.payload,
            score: self.score,
            started_at: self
                .started_at
                .ok_or_else(|| BuildError::missing_field("started_at"))?,
            status: self.status,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            trial_id: self
                .trial_id
                .ok_or_else(|| BuildError::missing_field("trial_id"))?,
        })
    }
}
