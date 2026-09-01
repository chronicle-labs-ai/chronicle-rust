pub use crate::prelude::*;

/// Media attached to an event, stored inline or by reference.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MediaAttachment {
    #[serde(default)]
    pub media_type: String,
    /// Inline bytes for small media. Absent when stored externally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_blob: Option<String>,
    /// URI for externally stored media. Absent when inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ref: Option<String>,
    #[serde(default)]
    pub size_bytes: i64,
}

impl MediaAttachment {
    pub fn builder() -> MediaAttachmentBuilder {
        <MediaAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MediaAttachmentBuilder {
    media_type: Option<String>,
    inline_blob: Option<String>,
    external_ref: Option<String>,
    size_bytes: Option<i64>,
}

impl MediaAttachmentBuilder {
    pub fn media_type(mut self, value: impl Into<String>) -> Self {
        self.media_type = Some(value.into());
        self
    }

    pub fn inline_blob(mut self, value: impl Into<String>) -> Self {
        self.inline_blob = Some(value.into());
        self
    }

    pub fn external_ref(mut self, value: impl Into<String>) -> Self {
        self.external_ref = Some(value.into());
        self
    }

    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MediaAttachment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`media_type`](MediaAttachmentBuilder::media_type)
    /// - [`size_bytes`](MediaAttachmentBuilder::size_bytes)
    pub fn build(self) -> Result<MediaAttachment, BuildError> {
        Ok(MediaAttachment {
            media_type: self
                .media_type
                .ok_or_else(|| BuildError::missing_field("media_type"))?,
            inline_blob: self.inline_blob,
            external_ref: self.external_ref,
            size_bytes: self
                .size_bytes
                .ok_or_else(|| BuildError::missing_field("size_bytes"))?,
        })
    }
}
