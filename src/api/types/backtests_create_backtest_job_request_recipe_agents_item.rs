pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBacktestJobRequestRecipeAgentsItem {
    /// CSS color token / hex. Used by `CandidateHueDot`.
    #[serde(default)]
    pub hue: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
    /// Tag rendered next to the label, e.g. "current production".
    #[serde(default)]
    pub notes: String,
    /// Defaults to `candidate`; the Results table treats the first baseline as the reference column.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<CreateBacktestJobRequestRecipeAgentsItemRole>,
}

impl CreateBacktestJobRequestRecipeAgentsItem {
    pub fn builder() -> CreateBacktestJobRequestRecipeAgentsItemBuilder {
        <CreateBacktestJobRequestRecipeAgentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBacktestJobRequestRecipeAgentsItemBuilder {
    hue: Option<String>,
    id: Option<String>,
    label: Option<String>,
    notes: Option<String>,
    role: Option<CreateBacktestJobRequestRecipeAgentsItemRole>,
}

impl CreateBacktestJobRequestRecipeAgentsItemBuilder {
    pub fn hue(mut self, value: impl Into<String>) -> Self {
        self.hue = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn role(mut self, value: CreateBacktestJobRequestRecipeAgentsItemRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBacktestJobRequestRecipeAgentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hue`](CreateBacktestJobRequestRecipeAgentsItemBuilder::hue)
    /// - [`id`](CreateBacktestJobRequestRecipeAgentsItemBuilder::id)
    /// - [`label`](CreateBacktestJobRequestRecipeAgentsItemBuilder::label)
    /// - [`notes`](CreateBacktestJobRequestRecipeAgentsItemBuilder::notes)
    pub fn build(self) -> Result<CreateBacktestJobRequestRecipeAgentsItem, BuildError> {
        Ok(CreateBacktestJobRequestRecipeAgentsItem {
            hue: self.hue.ok_or_else(|| BuildError::missing_field("hue"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
            role: self.role,
        })
    }
}
