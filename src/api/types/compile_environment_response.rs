pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompileEnvironmentResponse {
    #[serde(rename = "environmentId")]
    #[serde(default)]
    pub environment_id: String,
    #[serde(rename = "environmentSlug")]
    #[serde(default)]
    pub environment_slug: String,
    #[serde(rename = "versionId")]
    #[serde(default)]
    pub version_id: String,
    #[serde(default)]
    pub version: String,
    #[serde(rename = "tenantId")]
    #[serde(default)]
    pub tenant_id: String,
    #[serde(rename = "datasetSnapshotId")]
    #[serde(default)]
    pub dataset_snapshot_id: String,
    #[serde(rename = "scenarioId")]
    #[serde(default)]
    pub scenario_id: String,
    #[serde(rename = "bundleId")]
    #[serde(default)]
    pub bundle_id: String,
    #[serde(default)]
    pub sha256: String,
    #[serde(default)]
    pub uri: String,
    #[serde(rename = "packageUri")]
    #[serde(default)]
    pub package_uri: String,
    #[serde(rename = "rootDir")]
    #[serde(default)]
    pub root_dir: String,
    #[serde(rename = "sizeBytes")]
    #[serde(default)]
    pub size_bytes: i64,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub manifest: HashMap<String, serde_json::Value>,
}

impl CompileEnvironmentResponse {
    pub fn builder() -> CompileEnvironmentResponseBuilder {
        <CompileEnvironmentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompileEnvironmentResponseBuilder {
    environment_id: Option<String>,
    environment_slug: Option<String>,
    version_id: Option<String>,
    version: Option<String>,
    tenant_id: Option<String>,
    dataset_snapshot_id: Option<String>,
    scenario_id: Option<String>,
    bundle_id: Option<String>,
    sha256: Option<String>,
    uri: Option<String>,
    package_uri: Option<String>,
    root_dir: Option<String>,
    size_bytes: Option<i64>,
    warnings: Option<Vec<String>>,
    files: Option<Vec<String>>,
    manifest: Option<HashMap<String, serde_json::Value>>,
}

impl CompileEnvironmentResponseBuilder {
    pub fn environment_id(mut self, value: impl Into<String>) -> Self {
        self.environment_id = Some(value.into());
        self
    }

    pub fn environment_slug(mut self, value: impl Into<String>) -> Self {
        self.environment_slug = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn dataset_snapshot_id(mut self, value: impl Into<String>) -> Self {
        self.dataset_snapshot_id = Some(value.into());
        self
    }

    pub fn scenario_id(mut self, value: impl Into<String>) -> Self {
        self.scenario_id = Some(value.into());
        self
    }

    pub fn bundle_id(mut self, value: impl Into<String>) -> Self {
        self.bundle_id = Some(value.into());
        self
    }

    pub fn sha256(mut self, value: impl Into<String>) -> Self {
        self.sha256 = Some(value.into());
        self
    }

    pub fn uri(mut self, value: impl Into<String>) -> Self {
        self.uri = Some(value.into());
        self
    }

    pub fn package_uri(mut self, value: impl Into<String>) -> Self {
        self.package_uri = Some(value.into());
        self
    }

    pub fn root_dir(mut self, value: impl Into<String>) -> Self {
        self.root_dir = Some(value.into());
        self
    }

    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn files(mut self, value: Vec<String>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn manifest(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.manifest = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompileEnvironmentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment_id`](CompileEnvironmentResponseBuilder::environment_id)
    /// - [`environment_slug`](CompileEnvironmentResponseBuilder::environment_slug)
    /// - [`version_id`](CompileEnvironmentResponseBuilder::version_id)
    /// - [`version`](CompileEnvironmentResponseBuilder::version)
    /// - [`tenant_id`](CompileEnvironmentResponseBuilder::tenant_id)
    /// - [`dataset_snapshot_id`](CompileEnvironmentResponseBuilder::dataset_snapshot_id)
    /// - [`scenario_id`](CompileEnvironmentResponseBuilder::scenario_id)
    /// - [`bundle_id`](CompileEnvironmentResponseBuilder::bundle_id)
    /// - [`sha256`](CompileEnvironmentResponseBuilder::sha256)
    /// - [`uri`](CompileEnvironmentResponseBuilder::uri)
    /// - [`package_uri`](CompileEnvironmentResponseBuilder::package_uri)
    /// - [`root_dir`](CompileEnvironmentResponseBuilder::root_dir)
    /// - [`size_bytes`](CompileEnvironmentResponseBuilder::size_bytes)
    /// - [`warnings`](CompileEnvironmentResponseBuilder::warnings)
    /// - [`files`](CompileEnvironmentResponseBuilder::files)
    /// - [`manifest`](CompileEnvironmentResponseBuilder::manifest)
    pub fn build(self) -> Result<CompileEnvironmentResponse, BuildError> {
        Ok(CompileEnvironmentResponse {
            environment_id: self
                .environment_id
                .ok_or_else(|| BuildError::missing_field("environment_id"))?,
            environment_slug: self
                .environment_slug
                .ok_or_else(|| BuildError::missing_field("environment_slug"))?,
            version_id: self
                .version_id
                .ok_or_else(|| BuildError::missing_field("version_id"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            tenant_id: self
                .tenant_id
                .ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            dataset_snapshot_id: self
                .dataset_snapshot_id
                .ok_or_else(|| BuildError::missing_field("dataset_snapshot_id"))?,
            scenario_id: self
                .scenario_id
                .ok_or_else(|| BuildError::missing_field("scenario_id"))?,
            bundle_id: self
                .bundle_id
                .ok_or_else(|| BuildError::missing_field("bundle_id"))?,
            sha256: self
                .sha256
                .ok_or_else(|| BuildError::missing_field("sha256"))?,
            uri: self.uri.ok_or_else(|| BuildError::missing_field("uri"))?,
            package_uri: self
                .package_uri
                .ok_or_else(|| BuildError::missing_field("package_uri"))?,
            root_dir: self
                .root_dir
                .ok_or_else(|| BuildError::missing_field("root_dir"))?,
            size_bytes: self
                .size_bytes
                .ok_or_else(|| BuildError::missing_field("size_bytes"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            files: self
                .files
                .ok_or_else(|| BuildError::missing_field("files"))?,
            manifest: self
                .manifest
                .ok_or_else(|| BuildError::missing_field("manifest"))?,
        })
    }
}
